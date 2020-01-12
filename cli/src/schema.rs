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
