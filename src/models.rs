//! # Models
//!
//! There are two major models currently:
//!
//! * The Public File (a "frauth" file)
//! * The Private File

use base_emoji;
use ed25519_dalek::{Keypair, PublicKey, Signature};
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::collections::BTreeMap;
use toml::to_string as ser;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicFile {
    pub info: PublicInfo,
    pub sig: EmojiSignature,
}

#[derive(Debug)]
pub struct EmojiSignature(pub Signature);

#[derive(Debug)]
pub struct EmojiPublicKey(pub PublicKey);

struct StrVisitor;
impl<'de> Visitor<'de> for StrVisitor {
    type Value = &'de str;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("a base emoji thingy")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
        Ok(value)
    }
}

impl Serialize for EmojiSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base_emoji::to_string(&self.0.to_bytes()[..]))
    }
}

impl<'de> Deserialize<'de> for EmojiSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let my_str = deserializer.deserialize_str(StrVisitor)?;
        let emoji = base_emoji::try_from_str(my_str)
            .map_err(|_| de::Error::custom("emoji decode failed"))?;
        let signature = Signature::from_bytes(emoji.as_slice()).map_err(de::Error::custom)?;

        Ok(Self(signature))
    }
}

impl<'de> Deserialize<'de> for EmojiPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let my_str = deserializer.deserialize_str(StrVisitor)?;
        let emoji = base_emoji::try_from_str(my_str)
            .map_err(|_| de::Error::custom("emoji decode failed"))?;
        let pub_key = PublicKey::from_bytes(emoji.as_slice()).map_err(de::Error::custom)?;

        Ok(Self(pub_key))
    }
}

impl Serialize for EmojiPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base_emoji::to_string(&self.0.to_bytes()[..]))
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base_emoji::to_string(&self.bytes))
    }
}

// TODO: Don't use JSON. It doesn't have a canonical format,
// bad for hashing, okay for prototyping for now
#[derive(Serialize, Deserialize, Debug)]
pub struct FriendInfo {
    pub name: String,
    pub uri: String,
    pub pubkey: EmojiPublicKey,
}

#[derive(Deserialize, Debug)]
pub struct PrivateKey {
    pub bytes: [u8; 32],
}

#[derive(Debug)]
pub enum Error {
    DecodeLayoutFailure,
    DecodeTomlFailure,
    DecodeSignatureFailure,
}

const HEADER_TOP: &str = "FRAUTH-CONTENTS\n";
const HEADER_SIGNATURE: &str = "FRAUTH-SIGNATURE\n";
const HEADER_END_OF_FILE: &str = "FRAUTH-ENDOFFILE\n";

impl PublicFile {
    pub fn to_file_repr(&self) -> String {
        let mut out = String::new();
        out += HEADER_TOP;

        out += &self.info.to_file_repr();
        out += "\n";

        out += HEADER_SIGNATURE;

        out += &base_emoji::to_string(&self.sig.0.to_bytes()[..]);
        out += "\n";

        out += HEADER_END_OF_FILE;

        out
    }

    pub fn from_public_info(keypair: &Keypair, pinfo: PublicInfo) -> Self {
        let sig = keypair.sign(pinfo.to_file_repr().as_bytes());

        Self {
            info: pinfo,
            sig: EmojiSignature(sig),
        }
    }

    pub fn try_from_str(input: &str) -> Result<Self, Error> {
        let lines = input.lines().collect::<Vec<_>>();

        // Check 0: There are at least some lines
        if lines.len() < 2 {
            dbg!(lines.len());
            return Err(Error::DecodeLayoutFailure);
        }

        // Check 1: Make sure first line is sane
        if lines[0] != HEADER_TOP.trim() {
            dbg!(lines[0]);
            return Err(Error::DecodeLayoutFailure);
        }

        // Check 2: Make sure last line is sane
        if lines[lines.len() - 1] != HEADER_END_OF_FILE.trim() {
            dbg!(lines[lines.len() - 1]);
            return Err(Error::DecodeLayoutFailure);
        }

        // Check 3: Make sure only one middle divider
        let lines_body = &lines[1..lines.len()];

        let dividers = lines_body
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if *x == HEADER_SIGNATURE.trim() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if dividers.len() != 1 {
            dbg!(dividers.len());
            return Err(Error::DecodeLayoutFailure);
        }

        let pivot = dividers[0];

        if (pivot == 0) || (pivot == (lines_body.len() - 1)) {
            dbg!(pivot);
            dbg!(lines_body.len());
            return Err(Error::DecodeLayoutFailure);
        }

        let (toml_body, sig_plus) = lines_body.split_at(pivot);
        let (_hdr, sig_body_lines) = sig_plus.split_at(1);

        if sig_body_lines.len() != 2 {
            dbg!(sig_body_lines);
            return Err(Error::DecodeLayoutFailure);
        }

        // Check 4: Make sure signature parses
        let sig_body = sig_body_lines[0];

        // TODO: The rest of this check probably should just be serde?
        let sig_decoded = if let Ok(byte_vec) = base_emoji::try_from_str(&sig_body.trim()) {
            byte_vec
        } else {
            dbg!(":(");
            return Err(Error::DecodeSignatureFailure);
        };

        if sig_decoded.len() != ed25519_dalek::SIGNATURE_LENGTH {
            dbg!(sig_decoded.len());
            return Err(Error::DecodeSignatureFailure);
        }

        let signature = if let Ok(sig) = ed25519_dalek::Signature::from_bytes(sig_decoded.as_ref())
        {
            sig
        } else {
            dbg!(":((");
            return Err(Error::DecodeSignatureFailure);
        };

        // Check 5: Make sure toml de-tomls
        let combined = toml_body.join("\n");
        let pub_info: PublicInfo = if let Ok(toml) = toml::from_str(&combined).map_err(|x| dbg!(x))
        {
            toml
        } else {
            dbg!(combined);
            return dbg!(Err(Error::DecodeTomlFailure));
        };

        // Check 6: Make sure toml matches signature
        if pub_info
            .pubkey
            .0
            .verify(pub_info.to_file_repr().as_bytes(), &signature)
            .is_err()
        {
            return dbg!(Err(Error::DecodeSignatureFailure));
        }

        Ok(Self {
            info: pub_info,
            sig: EmojiSignature(signature),
        })
    }
}

impl PublicInfo {
    pub fn to_file_repr(&self) -> String {
        ser(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicInfo {
    pub name: String,
    pub note: String,
    pub pubkey: EmojiPublicKey,

    // TODO, URL/URI keys? Probably "raw" vs "internal" formats
    pub identities: BTreeMap<String, String>,
    pub friends: Vec<FriendInfo>,
}
