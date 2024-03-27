// use std::collections::HashMap;
// use serde_json::Value;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ContainerEntry {
    #[serde(alias="Id")]
    id: String,
    #[serde(alias="Names")]
    names: Vec<String>,
    #[serde(alias="Image")]
    image: String,
    #[serde(alias="Command")]
    command: String,
    #[serde(alias="Created")]
    created: i64,
    #[serde(alias="State")]
    state: String,
    #[serde(alias="Status")]
    status: String,
    // #[serde(alias="NetworkSettings")]
    // network_settings: HashMap<String, HashMap<String, HashMap<String, Value>>>,
}

impl ContainerEntry {
    pub fn to_view_vector(&self) -> [&str; 2] {
        let name: &str = match self.names.first() {
            Some(n) => n,
            None => ""
        };

        [
            name,
            &self.id,
        ]
    }
}