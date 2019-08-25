use ed25519_dalek::{Keypair, Signature};
use rand::{rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::to_string as ser;
use serde_json::to_string_pretty as ser_pretty;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// TODO: Don't use JSON. It doesn't have a canonical format,
// bad for hashing, okay for prototyping for now

#[derive(Serialize, Deserialize, Debug)]
struct PrivateKey {
    bytes: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicFile {
    info: PublicInfo,
    sig: Signature,
}

impl PublicFile {
    pub fn to_file_repr(&self) -> String {
        let mut out = self.info.to_file_repr();
        out += "\n";
        out += "FRAUTH-SIGNATURE\n";

        self.sig
            .to_bytes()
            .iter()
            .for_each(|b| out += &format!("{:02X}", b));
        out += "\n";
        out += "FRAUTH-ENDOFFILE\n";

        out
    }

    pub fn from_public_info(keypair: &Keypair, pinfo: PublicInfo) -> Self {
        let sig = keypair.sign(pinfo.to_file_repr().as_bytes());

        Self { info: pinfo, sig }
    }
}

impl PublicInfo {
    pub fn to_file_repr(&self) -> String {
        format!("FRAUTH-CONTENTS\n{}", &(ser_pretty(self).unwrap()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicInfo {
    name: String,
    note: String,

    // TODO, URL/URI keys?
    identities: HashMap<String, String>,
    friends: HashMap<String, String>,
}

pub(crate) fn new(private_path: &Path) {
    //////////
    // Generate private key

    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let to_out = PrivateKey {
        bytes: keypair.secret.to_bytes(),
    };

    // TODO: detect if file exists already? Warn?
    let mut ofile = File::create(private_path).unwrap();
    let serd = ser(&to_out).unwrap();
    ofile.write_all(serd.as_bytes()).unwrap();
    ofile.write_all("\n".as_bytes()).unwrap();

    //////////
    // Generate personal file
    let idents: HashMap<String, String> = [
        ("twitter".into(), "https://twitter.com/bitshiftmask".into()),
        ("github".into(), "https://github.com/jamesmunns".into()),
        ("email".into(), "james.munns@ferrous-systems.com".into()),
    ]
    .iter()
    .cloned()
    .collect();

    let friends: HashMap<String, String> = [
        (
            "Florian Gilcher".into(),
            "https://yakshav.es/.well-known/frauth.pub".into(),
        ),
        (
            "Felix Gilcher".into(),
            "https://felix.yakshav.es/.well-known/frauth.pub".into(),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let note: String = "Hello, I'm James!".into();

    let pinfo = PublicInfo {
        name: "James Munns".into(),
        note,
        identities: idents,
        friends,
    };

    // This probably isn't sound
    let pubfilecont = PublicFile::from_public_info(&keypair, pinfo);

    println!("{}", pubfilecont.to_file_repr());

    // TODO: detect if file exists already? Warn?
    let mut ofile = File::create("/home/james/.frauth/frauth.public").unwrap();
    ofile
        .write_all(pubfilecont.to_file_repr().as_bytes())
        .unwrap();

    // println!("\n\n\n\n");

    // println!("=========> TO HASH");
    // println!("{}", to_hash);
    // println!("=========> SIGNATURE");
    // println!("{}", ser(&sig).unwrap());
    // println!("=========> SPINFO");
    // println!("{}", &serdpinfo);
    // sig.to_bytes().iter().for_each(|b| print!("{:02X}", b));
    // println!();
    // println!("=========> DONE");
}
