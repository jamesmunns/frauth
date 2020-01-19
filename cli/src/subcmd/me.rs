use crate::Result;

use base64::encode;
use structopt::StructOpt;

use crate::{
    util::{load_user_info, write_user_info},
    Error,
};

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum MeOpts {
    /// View your configuration
    View {
        #[structopt(subcommand)]
        cmd: Option<ViewCmd>,
    },
    /// Edit your configuration
    Edit(EditOpts),
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum ViewCmd {
    /// Print your name
    Name,
    /// Print your current status, if any is set
    Status,
    /// Print your public key
    Pubkey,
    /// Print all identities
    Identities,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum EditOpts {
    /// Edit your display name
    Name {
        /// The display name to set
        name: String,
    },
    /// Edit your status
    Status(StatusOpts),
    /// Manage your identities
    Identities(IdentitiesOpts),
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct StatusOpts {
    /// The status to set
    #[structopt(required_unless = "clear")]
    status: Option<String>,
    /// Clear the status
    #[structopt(long)]
    clear: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum IdentitiesOpts {
    /// Add a new identity
    Add {
        /// The name of the identity (i.e. 'twitter', 'email' etc.)
        name: String,
        /// The id of the identity (i.e. 'my_twitter_id', 'me@example.com', etc.)
        id: String,
    },
    /// Modify an existing identity
    Modify {
        /// The name of the identity
        name: String,
        /// The id of the identity
        id: String,
    },
    /// Remove an identity
    Remove {
        /// The name of the identity
        name: String,
    },
}

pub fn me(subcmd: &MeOpts) -> Result<()> {
    match subcmd {
        MeOpts::View { cmd } => match cmd {
            Some(cmd) => view(cmd),
            None => view_all(),
        },
        MeOpts::Edit(opts) => edit(opts),
    }
}

fn view_all() -> Result<()> {
    let user_info = load_user_info()?;

    println!("Name:       {}", user_info.name);
    println!("Status:     {}", user_info.status.unwrap_or_else(|| "<no status is set>".to_string()));
    println!("Public key: {}", encode(user_info.keypair.public.as_bytes()));

    println!("\nIdentities:");
    for (name, id) in user_info.identities.iter() {
        println!("  - {}: {}", name, id);
    }

    Ok(())
}

fn view(cmd: &ViewCmd) -> Result<()> {
    let user_info = load_user_info()?;

    match cmd {
        ViewCmd::Name => {
            println!("{}", user_info.name);
        }
        ViewCmd::Status => match user_info.status {
            Some(status) => println!("{}", status),
            None => println!("You haven't set a status!"),
        },
        ViewCmd::Pubkey => {
            println!("{}", encode(user_info.keypair.public.as_bytes()));
        }
        ViewCmd::Identities => {
            for (name, id) in user_info.identities.iter() {
                println!("{}: {}", name, id);
            }
        }
    }
    Ok(())
}

fn edit(opts: &EditOpts) -> Result<()> {
    let mut user_info = load_user_info()?;

    match opts {
        EditOpts::Name { name } => {
            user_info.name = name.clone();
        }
        EditOpts::Status(opts) => {
            user_info.status = opts.status.clone();
        }
        EditOpts::Identities(opts) => match opts {
            IdentitiesOpts::Add { id, name } => {
                if user_info.identities.contains_key(name) {
                    return Err(Error::from(format!("identity '{}' already exists, use modify instead", name)));
                }
                user_info.identities.insert(name.clone(), id.clone());
            }
            IdentitiesOpts::Modify { id, name } => {
                if !user_info.identities.contains_key(name) {
                    return Err(Error::from(format!("identity '{}' does not exist, use add instead", name)));
                }
                user_info.identities.insert(name.clone(), id.clone());
            }
            IdentitiesOpts::Remove { name } => {
                match user_info.identities.remove(name) {
                    Some(_) => {},
                    None => return Err(Error::from(format!("can't remove identity '{}', identity does not exist!", name))),
                };
            }
        },
    }
    write_user_info(&user_info)?;
    println!("Updated configuration, don't forget to (re)publish me.frauth!");

    Ok(())
}
