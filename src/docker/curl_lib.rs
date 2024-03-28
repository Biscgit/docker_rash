use crate::types::AppResult;
use tokio::process::Command;

const DEFAULT_API_PATH: &str = "http://localhost/v1.44";


pub struct CurlBuilder<'a> {
    curl_args: Vec<&'a str>,
    payload_args: Option<Vec<&'a str>>,
    endpoint_args: Option<Vec<String>>,
}

impl<'a> CurlBuilder<'a> {
    pub fn new(socket_path: &'a str) -> CurlBuilder<'a> {
        CurlBuilder {
            curl_args: vec![
                "-s",
                "--unix-socket",
                socket_path,
            ],
            payload_args: None,
            endpoint_args: None,
        }
    }

    pub fn http_get(&mut self, api_endpoint: &'a str) -> AppResult<&mut Self> {
        match self.endpoint_args {
            Some(_) => Err("HTTP Endpoint already set!".into()),
            None => {
                self.endpoint_args = Some(vec![
                    "-X".to_string(),
                    "GET".to_string(),
                    format!(r"{}{}", DEFAULT_API_PATH, api_endpoint),
                ]);
                Ok(self)
            }
        }
    }

    pub fn http_post(&mut self, api_endpoint: &'a str) -> AppResult<&mut Self> {
        match self.endpoint_args {
            Some(_) => Err("HTTP Endpoint already set!".into()),
            None => {
                self.endpoint_args = Some(vec![
                    "-X".to_string(),
                    "POST".to_string(),
                    format!(r"{}{}", DEFAULT_API_PATH, api_endpoint),
                ]);
                Ok(self)
            }
        }
    }

    pub fn http_delete(&mut self, api_endpoint: &'a str) -> AppResult<&mut Self> {
        match self.endpoint_args {
            Some(_) => Err("HTTP Endpoint already set!".into()),
            None => {
                self.endpoint_args = Some(vec![
                    "-X".to_string(),
                    "DELETE".to_string(),
                    format!(r"{}{}", DEFAULT_API_PATH, api_endpoint),
                ]);
                Ok(self)
            }
        }
    }

    pub fn json_payload(&mut self, payload: &'a str) -> AppResult<&mut Self> {
        match self.payload_args {
            Some(_) => Err("JSON Payload already set!".into()),
            None => {
                self.payload_args = Some(vec![
                    "-H",
                    "Content-Type: application/json",
                    "-d",
                    payload,
                ]);
                Ok(self)
            }
        }
    }

    pub async fn execute_command(&mut self) -> AppResult<String> {
        let mut command = Command::new("curl");
        let result = command
            .args(&*self.curl_args)
            .args(self.payload_args.take().unwrap_or_default())
            .args(self.endpoint_args.take().unwrap_or_default())
            .output()
            .await?
            .stdout;

        Ok(String::from_utf8(result).unwrap())

        // match result.status.success() {
        //     true => { Ok(String::from_utf8(result.stdout)?) }
        //     false => {
        //         Err(ExecutionFailed::new(
        //             String::from_utf8(result.stderr)?, result.status.code(),
        //         ))
        //     }
        // }
    }
}


