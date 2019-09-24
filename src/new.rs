use ed25519_dalek::{Keypair, Signature, PublicKey};
use rand::{rngs::OsRng, Rng};
use serde::{Deserialize, Serialize, ser::Serializer};
use toml::to_string as ser;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use base_emoji;

#[derive(Deserialize, Debug)]
struct EmojiSignature(Signature);

#[derive(Deserialize, Debug)]
struct EmojiPublicKey(PublicKey);


impl Serialize for EmojiSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base_emoji::to_string(&self.0.to_bytes()[..]))
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
struct FriendInfo {
    name: String,
    uri: String,
    pubkey: EmojiPublicKey,
}

#[derive(Deserialize, Debug)]
struct PrivateKey {
    bytes: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicFile {
    info: PublicInfo,
    sig: EmojiSignature,
}

impl PublicFile {
    pub fn to_file_repr(&self) -> String {
        let mut out = self.info.to_file_repr();
        out += "\n";
        out += "FRAUTH-SIGNATURE\n";

        out += &base_emoji::to_string(&self.sig.0.to_bytes()[..]);

        out += "\n";
        out += "FRAUTH-ENDOFFILE\n";

        out
    }

    pub fn from_public_info(keypair: &Keypair, pinfo: PublicInfo) -> Self {
        let sig = keypair.sign(pinfo.to_file_repr().as_bytes());

        Self { info: pinfo, sig: EmojiSignature(sig) }
    }
}

impl PublicInfo {
    pub fn to_file_repr(&self) -> String {
        format!("FRAUTH-CONTENTS\n{}", &(ser(self).unwrap()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicInfo {
    name: String,
    note: String,
    pubkey: EmojiPublicKey,

    // TODO, URL/URI keys? Probably "raw" vs "internal" formats
    identities: BTreeMap<String, String>,
    friends: Vec<FriendInfo>,
}

pub(crate) fn new(private_path: &Path) {
    //////////
    // Generate private key

    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let to_out = PrivateKey {
        bytes: keypair.secret.to_bytes(),
    };

    // TODO: detect if file exists already? Warn?
    let mut ofile = File::create(private_path).unwrap();
    let serd = base_emoji::to_string(&to_out.bytes);
    ofile.write_all(serd.as_bytes()).unwrap();
    ofile.write_all("\n".as_bytes()).unwrap();

    //////////
    // Generate personal file
    let idents: BTreeMap<String, String> = [
        ("twitter".into(), "https://twitter.com/bitshiftmask".into()),
        ("github".into(), "https://github.com/jamesmunns".into()),
        ("email".into(), "james.munns@ferrous-systems.com".into()),
    ]
    .iter()
    .cloned()
    .collect();

    let keys = [
        Keypair::generate(&mut csprng),
        Keypair::generate(&mut csprng),
    ];

    let friends: Vec<FriendInfo> = vec![
        FriendInfo {
            name: "Alice Shamir".into(),
            uri: "https://example.com/.well-known/alice-shamir.frauth".into(),
            pubkey: EmojiPublicKey(keys[0].public),
        },
        FriendInfo {
            name: "Bob Diffie".into(),
            uri: "https://beispiel.com/.well-known/bob-diffie.frauth".into(),
            pubkey: EmojiPublicKey(keys[1].public),
        },
    ];

    let note: String = "Hello, I'm James!".into();

    let pinfo = PublicInfo {
        name: "James Munns".into(),
        note,
        identities: idents,
        friends,
        pubkey: EmojiPublicKey(keypair.public),
    };

    // This probably isn't sound
    let pubfilecont = PublicFile::from_public_info(&keypair, pinfo);

    println!("{}", pubfilecont.to_file_repr());

    // TODO: detect if file exists already? Warn?
    let mut ofile = File::create("/home/james/.frauth/james-munns.frauth").unwrap();
    ofile
        .write_all(pubfilecont.to_file_repr().as_bytes())
        .unwrap();

    // println!("\n\n\n\n");

    // println!("=========> TO HASH");
    // println!("{}", to_hash);
    // println!("=========> SIGNATURE");
    // println!("{}", ser(&sig).unwrap());
    // println!("=========> SPINFO");
    // println!("{}", &serdpinfo);
    // sig.to_bytes().iter().for_each(|b| print!("{:02X}", b));
    // println!();
    // println!("=========> DONE");
}
