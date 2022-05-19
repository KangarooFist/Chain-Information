
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blockbook {
    pub coin: String,
    // host: String,
    // version: String,
    // git_commit: String,
    // build_time: String,
    // sync_mode: bool,
    // initial_sync: bool,
    // in_sync: bool,
    // best_height: i64,
    // latest_block_time: String,
    // in_sync_mempool: bool,
    // last_mempool_time: String,
    // mempool_size: i64,
    // decimals: i64,
    // db_size: i64,
    // abount: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub chain: String,
    // blocks: i64,
    // best_block_hash: String,
    // difficulty: String,
    // version: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockStatus {
    pub blockbook: Blockbook,
    pub backend: Backend,
}