use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use ed25519_dalek::Keypair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub status: Option<String>,
    pub keypair: Keypair,
    pub identities: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Friends {
    pub map: BTreeMap<String, FriendInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendInfo {
    pub last_updated: DateTime<Utc>,
    pub public: bool,
    pub info: PublishUserInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Peers {
    pub map: BTreeMap<String, FriendInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    pub last_updated: DateTime<Utc>,
    pub info: PublishUserInfo,
    pub score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishUserInfo {
    pub name: String,
    pub status: Option<String>,
    pub pubkey: String,
    pub friends: Vec<PublishFriend>,
    pub identities: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishFriend {
    pub uri: String,
    pub pubkey: String,
}
