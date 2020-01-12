use structopt::StructOpt;
use lazy_static::lazy_static;
use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Paths {
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

fn main() {
    let opt = SubCommands::from_args();

    println!("{:#?}", &*PATHS);

    match opt {
        SubCommands::Init => {
            unimplemented!()
        }
    }
}

pub fn bail(reason: &str) -> ! {
    eprintln!("{}", reason);
    ::std::process::exit(1);
}
