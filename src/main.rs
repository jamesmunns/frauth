use structopt::StructOpt;
use std::path::PathBuf;

mod new;

#[derive(StructOpt, Debug)]
enum SubCommands {
    #[structopt(name = "new")]
    New {
        #[structopt(short = "o")]
        output_path: Option<PathBuf>,
    },

    #[structopt(name = "verify")]
    Verify,
}

fn main() {
    let opt = SubCommands::from_args();

    match opt {
        SubCommands::New { output_path } => {
            let private = output_path.unwrap_or_else(||
                // TODO: get home path, figure out canonical path
                PathBuf::from("/home/james/.frauth/frauth.private")
            );

            new::new(&private);
        }
        x @ SubCommands::Verify => {
            unimplemented!();
        }
    }
}
