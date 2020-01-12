use structopt::StructOpt;
use lazy_static::lazy_static;
use directories::ProjectDirs;
use std::path::PathBuf;

pub mod subcmd;
pub mod schema;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) struct Paths {
    base_data: PathBuf,
    base_cache: PathBuf,
    user_info: PathBuf,
    known_db: PathBuf,
    network: PathBuf,
}

lazy_static!{
    pub(crate) static ref PATHS: Paths = {
        let project_dirs = ProjectDirs::from(
            "com",
            "Frauth",
            "frauth-cli"
        ).unwrap_or_else(|| {
            bail("Failed to find a suitable home/config directory!")
        });

        let base_data = project_dirs.data_dir();
        let base_cache = project_dirs.cache_dir();

        Paths {
            base_data: base_data.into(),
            base_cache: base_cache.into(),
            user_info: base_data.join("me.frauth"),
            known_db: base_data.join("known.frauth"),
            network: base_cache.join("network.frauth"),
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
    Init
}

fn main() -> Result<()> {
    let opt = SubCommands::from_args();

    println!("{:#?}", &*PATHS);

    match opt {
        SubCommands::Init => {
            subcmd::init::init()
        }
    }
}

pub fn bail(reason: &str) -> ! {
    eprintln!("{}", reason);
    ::std::process::exit(1);
}
