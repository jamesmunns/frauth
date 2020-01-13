use std::{
    fs::{read_to_string, File, OpenOptions},
    path::Path,
};

use toml::from_str;

use crate::{schema::Friends, Error, Result, PATHS};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

pub fn create_private_file(path: &Path) -> Result<File> {
    let mut opt = OpenOptions::new();
    opt.write(true);
    opt.truncate(true);
    opt.create(true);

    #[cfg(unix)]
    opt.mode(0o600);

    // TODO: Figure out how file permissions work on Windows, or link to an issue
    #[cfg(not(unix))]
    eprintln!(
        "Warning! You should set the permissions for {} to only be readable by this user!"
        PATHS.user_info.display()
    );

    opt.open(path).map_err(|e| {
        Error::from(format!("Failed to open file: {}\nReason: {}", path.display(), e).as_str())
    })
}

pub fn load_friends() -> Result<Friends> {
    let contents = read_to_string(&PATHS.friend_info)?;
    let data = from_str(&contents)?;
    Ok(data)
}
