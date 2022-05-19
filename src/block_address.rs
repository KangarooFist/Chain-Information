#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockAddress {
    pub address: String,
    pub txids: Vec<String>
}

//Vectors are Lists in Rust - using to easily deserialise.
