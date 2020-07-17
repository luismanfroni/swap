use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    name: String,
    author: String,
    pack: String,
    tags: Vec<String>
}