use crate::Result;

use structopt::StructOpt;

use crate::util::{load_user_info, write_user_info};

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum MeOpts {
    /// Modify your status
    Status(StatusOpts),
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum StatusOpts {
    /// Get your current status
    Get,

    /// Set a new status
    Set{
        /// The new status
        status: String
    },
}

pub fn me(subcmd: &MeOpts) -> Result<()> {
    match subcmd {
        MeOpts::Status(opts) => status(opts),
    }
}

fn status(opts: &StatusOpts) -> Result<()> {
    let mut user_info = load_user_info()?;

    match opts {
        StatusOpts::Get => {
            match user_info.status {
                Some(status) => println!("{}", status),
                None => println!("You haven't set a status yet!"),
            }
        },
        StatusOpts::Set { status } => {
            user_info.status = Some(status.clone());

            write_user_info(&user_info)?;
        },
    }

    Ok(())
}
