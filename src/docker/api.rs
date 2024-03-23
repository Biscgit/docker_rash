use crate::docker::curl_lib::{CurlBuilder, format_api_endpoint};
use crate::docker::models;

type SResult<T> = serde_json::Result<T>;

pub struct DockerAPI<'a> {
    socket_path: &'a str,
}

impl Default for DockerAPI<'_> {
    fn default() -> Self {
        DockerAPI {
            socket_path: "/var/run/docker.sock"
        }
    }
}

impl DockerAPI<'_> {
    pub async fn get_all_containers(&self) -> Vec<models::ContainerEntry> {
        let data = CurlBuilder::new(self.socket_path)
            .http_get(&format_api_endpoint("/containers/json"))
            .unwrap()
            .execute()
            .await;

        let data: SResult<Vec<models::ContainerEntry>> = serde_json::from_str(&data);
        match data {
            Ok(d) => { d.unwrap() }
            Err(e) => { panic!(e) }
        }
    }
}