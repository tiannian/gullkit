use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gull {
    module_path: String,
    database: Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    connection: String,
    prefix: String,
}
