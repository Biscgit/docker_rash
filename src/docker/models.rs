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

    pub fn example() -> Self {
        ContainerEntry {
            id: "f3183789dbfb5167379cd0d9c10c8a0b3c1365e648ea1a67a1fdfc8043361b82".to_string(),
            names: vec!["/stupefied_hellman".to_string()],
            image: "nginx".to_string(),
            command: "/docker-entrypoint.sh nginx -g 'daemon off;'".to_string(),
            created: 1711278459,
            state: "running".to_string(),
            status: "Up 1 second".to_string(),
        }
    }
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