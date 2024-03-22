use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ContainerList {
    containers: Vec<ContainerEntry>,
}

#[derive(Serialize, Deserialize)]
struct ContainerEntry {
    id: String,
    names: Vec<String>,
    command: String,
    created: i64,
    state: String,
    status: String,
}