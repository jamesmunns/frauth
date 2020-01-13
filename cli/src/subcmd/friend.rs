use std::{fs::read_to_string, io::Write};

use async_std::task;
use base64::decode;
use dialoguer::Input;
use ed25519_dalek::PublicKey;
use structopt::StructOpt;
use surf::get;
use toml::{from_str, to_string};

use crate::{
    consts::FRIEND_INFO_HEADER,
    schema::{FriendInfo, Friends, PublishUserInfo},
    subcmd::publish::{HEADER_END_OF_FILE, HEADER_SIGNATURE, HEADER_TOP},
    util::{create_private_file, load_friends},
    {Error, Result, PATHS},
};

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum FriendOpts {
    /// Add a friend to your list
    Add {
        /// The URL identifying your friend
        url: String,
    },

    /// Remove a friend from your list
    Remove {
        /// The URL identifying your friend
        url: String,
    },

    /// List all friends
    List {
        /// Show all info for each friend
        #[structopt(long = "detailed", short = "d")]
        detailed: bool,
    },

    /// Update a friend's info
    Update {
        /// The URL identifying your friend
        url: String,
    },
}

pub fn friend(subcmd: &FriendOpts) -> Result<()> {
    let friends = load_friends()?;

    match subcmd {
        FriendOpts::Add { url } => add(url, friends),
        FriendOpts::Remove { url } => remove(url, friends),
        FriendOpts::List { detailed } => list(*detailed, friends),
        FriendOpts::Update { url } => update(url, friends),
    }
}

fn add(url: &str, mut friends: Friends) -> Result<()> {
    if friends.map.contains_key(url) {
        eprintln!("\nWe already know about '{}'!", url);
        eprintln!("\nRun `frauth update <url>` to update information about a friend,");
        eprintln!("or remove this friend first with `frauth remove <url>`.");
        return Err(Error::from("Friend already known!"));
    }

    print!(
        "\nPlease enter `{}`'s public key. You should ask them for it via a separate",
        url
    );
    println!(" route, such as a private message, email, or text message.");

    println!("\nThey can print their public key on their PC with the command `frauth pubkey`.");

    let pubkey_maybe_str = Input::<String>::new()
        .with_prompt("Public Key")
        .interact()?;

    println!("\nConfirming public key...");

    let pub_info = url_to_pub_info(url)?;

    if pub_info.pubkey != pubkey_maybe_str {
        eprintln!("\nPublic Key Mismatch! Please double check the public key, or ask your friend to re-send.");
        return Err(Error::from("public key mismatch"));
    }

    friends
        .map
        .insert(url.to_string(), FriendInfo { info: pub_info });

    save_friends(&friends)?;

    println!("\nAdded '{}' succesfully!", url);

    Ok(())
}

fn remove(url: &str, mut friends: Friends) -> Result<()> {
    Ok(())
}

fn list(detailed: bool, friends: Friends) -> Result<()> {
    Ok(())
}

fn update(url: &str, mut friends: Friends) -> Result<()> {
    Ok(())
}

fn url_to_pub_info(url: &str) -> Result<PublishUserInfo> {
    let body_res: Result<String> = task::block_on(async {
        Ok(get(url)
            .await
            .map_err(|_e| Error::from("lol"))?
            .body_string()
            .await
            .map_err(|_e| Error::from("lol"))?)
    });
    let body = body_res?;
    let pub_info = try_from_str(&body)
        .map_err(|e| Error::from(format!("Failed to decode: {:?}", e).as_str()))?;

    Ok(pub_info)
}

#[derive(Debug)]
enum DecodeError {
    DecodeLayoutFailure,
    DecodeTomlFailure,
    DecodeSignatureFailure,
    DecodeVerificationFailure,
    DecodePublicKeyFailure,
}

fn save_friends(friends: &Friends) -> Result<()> {
    // TODO: I should do the "save then copy" trick, not overwrite directly.
    let mut file = create_private_file(&PATHS.friend_info)?;
    let contents = to_string(&friends)?;
    file.write_all(FRIEND_INFO_HEADER.as_bytes())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn try_from_str(input: &str) -> ::std::result::Result<PublishUserInfo, DecodeError> {
    // TODO: This could probably be done in a way more efficient way
    // that doesn't require splitting the content into lines and collecting
    // them and recombining them. But that isn't a big deal for now

    let lines = input.lines().collect::<Vec<_>>();

    // Check 0: There are at least some lines. We need at least:
    // * A top header
    // * At least one body line
    // * A signature header
    // * A signature
    // * An End of File footer
    assert_or(lines.len() >= 5, DecodeError::DecodeLayoutFailure)?;

    // Check 1: Make sure first line is sane
    assert_or(
        lines[0] == HEADER_TOP.trim(),
        DecodeError::DecodeLayoutFailure,
    )?;

    // Check 2: Make sure last line is sane
    assert_or(
        lines[lines.len() - 1] == HEADER_END_OF_FILE.trim(),
        DecodeError::DecodeLayoutFailure,
    )?;

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

    assert_or(dividers.len() == 1, DecodeError::DecodeLayoutFailure)?;

    let pivot = dividers[0];

    assert_or(pivot != 0, DecodeError::DecodeLayoutFailure)?;
    assert_or(
        pivot != (lines_body.len() - 1),
        DecodeError::DecodeLayoutFailure,
    )?;

    let (toml_body, sig_plus) = lines_body.split_at(pivot);
    let (_hdr, sig_body_lines) = sig_plus.split_at(1);

    assert_or(sig_body_lines.len() == 2, DecodeError::DecodeLayoutFailure)?;
    assert_or(toml_body.len() >= 1, DecodeError::DecodeLayoutFailure)?;

    // Check 4: Make sure signature parses
    let sig_body = sig_body_lines[0];

    // TODO: The rest of this check probably should just be serde?
    let sig_decoded = decode(&sig_body.trim()).map_err(|_| DecodeError::DecodeSignatureFailure)?;

    assert_or(
        sig_decoded.len() == ed25519_dalek::SIGNATURE_LENGTH,
        DecodeError::DecodeSignatureFailure,
    )?;

    let signature = ed25519_dalek::Signature::from_bytes(sig_decoded.as_ref())
        .map_err(|_| DecodeError::DecodeSignatureFailure)?;

    // Check 5: Make sure toml de-tomls
    let mut combined = toml_body.join("\n");
    combined += "\n";
    let pub_info: PublishUserInfo =
        toml::from_str(&combined).map_err(|_x| DecodeError::DecodeTomlFailure)?;

    // Get pubkey from base64
    let pubkey_bytes = decode(&pub_info.pubkey).map_err(|_| DecodeError::DecodePublicKeyFailure)?;

    let public_key =
        PublicKey::from_bytes(&pubkey_bytes).map_err(|_| DecodeError::DecodePublicKeyFailure)?;

    // Check 6: Make sure toml matches signature
    let good_sig = public_key.verify(combined.as_bytes(), &signature).is_ok();

    assert_or(good_sig, DecodeError::DecodeVerificationFailure)?;

    Ok(pub_info)
}

fn assert(me: bool) -> Result<()> {
    if me {
        Ok(())
    } else {
        Err(Error::from("Failed Assert!"))
    }
}

fn assert_or<E>(me: bool, err: E) -> ::std::result::Result<(), E> {
    assert(me).map_err(|_| err)
}
