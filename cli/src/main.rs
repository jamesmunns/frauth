use std::path::PathBuf;

use directories::ProjectDirs;
use lazy_static::lazy_static;
use structopt::StructOpt;

use crate::subcmd::{friend::FriendOpts, publish::PublishOpts};

pub mod consts;
pub mod schema;
pub mod subcmd;
pub mod util;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) struct Paths {
    base_data: PathBuf,
    base_cache: PathBuf,
    user_info: PathBuf,
    friend_info: PathBuf,
    peer_info: PathBuf,
}

lazy_static! {
    pub(crate) static ref PATHS: Paths = {
        let project_dirs = ProjectDirs::from("com", "Frauth", "frauth-cli")
            .unwrap_or_else(|| bail("Failed to find a suitable home/config directory!"));

        let base_data = project_dirs.data_dir();
        let base_cache = project_dirs.cache_dir();

        Paths {
            base_data: base_data.into(),
            base_cache: base_cache.into(),
            user_info: base_data.join("me.frauth"),
            friend_info: base_data.join("known.frauth"),
            peer_info: base_cache.join("peer.frauth"),
        }
    };
}

/// frauth provides mechanisms to verify identities.
///
/// In the future it will also provide functionality to sign or verify messages
/// and files, encrypt or decrypt files, and easily discover and verify
/// friends-of-friends.
#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
enum SubCommands {
    /// Initialize frauth, creating keys and necessary directories
    Init,

    /// Modify your own config
    Me,

    /// Render a file that can be placed on a static website
    Publish(PublishOpts),

    /// Operations around your friend list
    Friend(FriendOpts),
}

fn main() -> Result<()> {
    let opt = SubCommands::from_args();

    let ret = match opt {
        SubCommands::Init => subcmd::init::init(),
        SubCommands::Me => subcmd::me::me(),
        SubCommands::Publish(opts) => subcmd::publish::publish(&opts),
        SubCommands::Friend(opts) => subcmd::friend::friend(&opts),
    };

    if ret.is_err() {
        println!();
    }

    ret
}

pub fn bail(reason: &str) -> ! {
    eprintln!("{}", reason);
    ::std::process::exit(1);
}
