use crate::{Result, PATHS};
use crate::schema::{UserInfo, PublishUserInfo};
use toml::{from_str, to_string};
use std::fs::read_to_string;
use base64::encode;
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::OpenOptions;
use std::io::Write;


const HEADER_TOP: &str = "FRAUTH-CONTENTS\n";
const HEADER_SIGNATURE: &str = "FRAUTH-SIGNATURE\n";
const HEADER_END_OF_FILE: &str = "FRAUTH-ENDOFFILE\n";

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct PublishOpts {
    /// File to output to. If omitted, the file will be output to stdout
    #[structopt(short = "o", long = "output")]
    output: Option<PathBuf>
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

fn render_to_string(user_info: UserInfo) -> Result<String> {
    let pub_info = PublishUserInfo {
        name: user_info.name,
        status: user_info.status,
        pubkey: encode(user_info.keypair.public.as_bytes()),
        identities: user_info.identities,
        friends: vec![], // TODO
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
