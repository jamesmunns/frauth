use ed25519_dalek::Keypair;
use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;
use toml;
use serde_json;
use postcard;

#[derive(Debug, Serialize, Deserialize)]
struct Demo {
    keypair: Keypair
}

fn main() {
    println!("Make a key...");

    let demo = Demo { keypair: Keypair::generate(&mut OsRng) };

    println!("\n\nWrite to toml");
    let demo_toml = toml::to_string(&demo).unwrap();
    println!("{}", demo_toml);
    let demo_toml_rebuild: Result<Demo, _> = toml::from_str(&demo_toml);
    println!("{:?}", demo_toml_rebuild);

    println!("\n\nWrite to json");
    let demo_json = serde_json::to_string(&demo).unwrap();
    println!("{}", demo_json);
    let demo_json_rebuild: Result<Demo, _> = serde_json::from_str(&demo_json);
    println!("{:?}", demo_json_rebuild);

    println!("\n\nWrite to postcard");
    let demo_postcard = postcard::to_stdvec(&demo).unwrap();
    println!("{:?}", demo_postcard);
    let demo_postcard_rebuild: Result<Demo, _> = postcard::from_bytes(&demo_postcard);
    println!("{:?}", demo_postcard_rebuild);
}
