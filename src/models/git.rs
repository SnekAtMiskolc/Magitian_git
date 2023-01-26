use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Service {
    pub service: String,
}

#[derive(Deserialize, Serialize)]
pub struct GitPath {
    pub u: String,
    pub r: String,
}