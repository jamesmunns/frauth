use std::collections::HashMap;

use ed25519_dalek::Keypair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub status: Option<String>,
    pub keypair: Keypair,
    pub identities: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishUserInfo {
    pub name: String,
    pub status: Option<String>,
    pub pubkey: String,
    pub friends: Vec<PublishFriend>,
    pub identities: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishFriend {
    pub name: String,
    pub uri: String,
    pub pubkey: String,
}
