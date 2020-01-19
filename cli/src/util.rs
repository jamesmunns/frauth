use std::{
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    path::Path,
};

use toml::{from_str, to_string};

use crate::{
    consts::USER_INFO_HEADER,
    schema::{Friends, UserInfo},
    {Error, Result, PATHS},
};

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
        "Warning! You should set the permissions for {} to only be readable by this user!",
        PATHS.user_info.display()
    );

    opt.open(path).map_err(|e| {
        Error::from(format!("Failed to open file: {}\nReason: {}", path.display(), e).as_str())
    })
}

fn open_existing_file(path: &Path) -> Result<File> {
    let mut opt = OpenOptions::new();
    opt.write(true);

    opt.open(path).map_err(|e| {
        Error::from(format!("Failed to open file: {}\nReason: {}", path.display(), e).as_str())
    })
}

pub fn load_user_info() -> Result<UserInfo> {
    let contents = read_to_string(&PATHS.user_info)?;

    from_str(&contents)
        .map_err(|e| {
            println!("{:?}", e);
            e
        })
        .map_err(|_| "Failed to parse user info!".into())
}

pub fn write_user_info(user_info: &UserInfo) -> Result<()> {
    let mut user_info_file = open_existing_file(&PATHS.user_info)?;
    let contents = to_string(&user_info)?;

    user_info_file.set_len(0)?;
    user_info_file.write_all(USER_INFO_HEADER.as_bytes())?;
    user_info_file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn load_friends() -> Result<Friends> {
    let contents = read_to_string(&PATHS.friend_info)?;
    let data = from_str(&contents)?;
    Ok(data)
}
