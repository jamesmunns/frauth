use std::collections::{BTreeMap, HashMap};

use ed25519_dalek::Keypair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub status: Option<String>,
    pub keypair: Keypair,
    pub identities: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Friends {
    pub map: BTreeMap<String, FriendInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendInfo {
    // TODO: Last Checked
    pub info: PublishUserInfo,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Peers {
    pub map: BTreeMap<String, FriendInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    // TODO: Last Checked
    pub info: PublishUserInfo,
    pub score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishUserInfo {
    pub name: String,
    pub status: Option<String>,
    pub pubkey: String,
    pub friends: Vec<PublishFriend>,
    pub identities: BTreeMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishFriend {
    pub uri: String,
    pub pubkey: String,
}
