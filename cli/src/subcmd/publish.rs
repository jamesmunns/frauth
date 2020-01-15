use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

use base64::encode;
use chrono::Utc;
use structopt::StructOpt;
use toml::{from_str, to_string};

use crate::{
    schema::{PublishFriend, PublishUserInfo, UserInfo},
    util::load_friends,
    Result, PATHS,
};

pub const HEADER_TOP: &str = "FRAUTH-CONTENTS\n";
pub const HEADER_SIGNATURE: &str = "FRAUTH-SIGNATURE\n";
pub const HEADER_END_OF_FILE: &str = "FRAUTH-ENDOFFILE\n";

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct PublishOpts {
    /// File to output to. If omitted, the file will be output to stdout
    #[structopt(short = "o", long = "output")]
    output: Option<PathBuf>,
}

pub fn publish(opts: &PublishOpts) -> Result<()> {
    let user_info = load_user_file()?;
    let contents = render_to_string(user_info)?;

    if let Some(ref path) = opts.output {
        let mut opt = OpenOptions::new();
        opt.write(true);
        opt.truncate(true);
        opt.create(true);

        let mut file = opt.open(path)?;
        file.write_all(contents.as_bytes())?;
    } else {
        println!("{}", contents);
    }

    Ok(())
}

fn load_user_file() -> Result<UserInfo> {
    let contents = read_to_string(&PATHS.user_info)?;
    from_str(&contents)
        .map_err(|e| {
            println!("{:?}", e);
            e
        })
        .map_err(|_| "Failed to parse user info!".into())
}

fn render_to_string(mut user_info: UserInfo) -> Result<String> {
    let friends = load_friends()?;
    let pub_friends = friends
        .map
        .iter()
        .filter_map(|(uri, friend)| {
            if friend.public {
                Some(PublishFriend {
                    uri: uri.to_string(),
                    pubkey: friend.info.pubkey.to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    let pub_info = PublishUserInfo {
        name: user_info.name,
        status: user_info.status,
        pubkey: encode(user_info.keypair.public.as_bytes()),
        last_updated: Some(Utc::now()),
        identities: user_info.identities.drain().collect(),
        friends: pub_friends,
    };

    let toml_contents = to_string(&pub_info)?;
    let sig = user_info.keypair.sign(toml_contents.as_bytes());

    let mut contents = String::new();
    contents += HEADER_TOP;
    contents += &toml_contents;
    contents += HEADER_SIGNATURE;
    contents += &encode(&sig.to_bytes()[..]);
    contents += "\n";
    contents += HEADER_END_OF_FILE;

    Ok(contents)
}
