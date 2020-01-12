use std::collections::HashSet;
use ed25519_dalek::PublicKey;
use models::PublicInfo;

pub struct Node {
    vectors: Vec<Identity>,
    information: PublicInfo,
}

pub struct Identity {
    pub url: String,
    pub pubkey: PublicKey,
}
