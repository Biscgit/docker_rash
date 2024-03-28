use crate::docker::curl_lib::CurlBuilder;
use crate::docker::models;
use crate::types::AppResult;

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

    pub async fn test_connection(&self) -> AppResult<bool>{
        let data = CurlBuilder::new(self.socket_path)
            .http_get("/_ping")?
            .execute_command()
            .await?;

        Ok(&data == "OK")
    }

    pub async fn get_all_containers(&self) -> AppResult<Vec<models::ContainerEntry>> {
        let data = CurlBuilder::new(self.socket_path)
            .http_get("/containers/json")?
            .execute_command()
            .await?;

        let data: SResult<Vec<models::ContainerEntry>> = serde_json::from_str(&data);
        match data {
            Ok(d) => { Ok(d) }
            Err(e) => { panic!("{}", e) }
        }
    }
}