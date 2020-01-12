use models::PublicFile;
use std::path::PathBuf;
use structopt::StructOpt;

mod decode;
mod models;
mod new;
mod web;

#[derive(StructOpt, Debug)]
enum SubCommands {
    #[structopt(name = "new")]
    New {
        #[structopt(short = "o")]
        output_path: Option<PathBuf>,
    },

    #[structopt(name = "verify")]
    Verify {
        #[structopt(short = "i")]
        input_path: Option<PathBuf>,
    },
}

fn main() {
    let opt = SubCommands::from_args();

    match opt {
        SubCommands::New { output_path } => {
            let private = output_path.unwrap_or_else(||
                // TODO: get home path, figure out canonical path
                PathBuf::from("/home/james/.frauth/frauth.private"));

            new::new(&private);
        }
        SubCommands::Verify { input_path } => {
            let frauth = input_path.unwrap_or_else(||
                // TODO: get home path, figure out canonical path
                PathBuf::from("/home/james/.frauth/james-munns.frauth"));

            let file_string =
                ::std::fs::read_to_string("/home/james/.frauth/james-munns.frauth").unwrap();

            PublicFile::try_from_str(&file_string)
                .map_err(|x| dbg!(x))
                .expect("verify failed");

            println!("Successfully verified {:?}.", frauth);
        }
    }
}
