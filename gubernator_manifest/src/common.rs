use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Service {
    pub name: String,
    pub containers: Vec<Container>,
}

#[derive(Deserialize, Serialize)]
pub struct Container {
    pub image: String,
}
