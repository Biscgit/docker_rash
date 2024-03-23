use tokio::process::Command;
use std::error::Error;

const DEFAULT_API_PATH: &str = "http://localhost/v1.44";

pub fn format_api_endpoint(api_endpoint: &str) -> String {
    format!(r"{}{}", DEFAULT_API_PATH, api_endpoint)
}

pub struct CurlBuilder<'a> {
    command: Command,
    command_args: Vec<&'a str>,

    endpoint_set: bool,
}

impl<'a> CurlBuilder<'a> {
    pub fn new(socket_path: &'a str) -> CurlBuilder<'a> {
        CurlBuilder {
            command: Command::new("curl"),
            command_args: vec![
                "-s",
                "--unix-socket",
                socket_path,
            ],
            endpoint_set: false,
        }
    }

    pub fn http_get(&mut self, api_endpoint: &'a str) -> Result<&mut Self, Box<dyn Error>> {
        if !self.endpoint_set {
            self.endpoint_set = true;

            self.command_args.push("-X");
            self.command_args.push("GET");
            self.command_args.push(api_endpoint);
            return Ok(self);
        }
        Err("Endpoint already set!".into())
    }

     pub fn http_post(&mut self, api_endpoint: &'a str) -> Result<&mut Self, Box<dyn Error>> {
         if !self.endpoint_set {
             self.endpoint_set = true;

             self.command_args.push("-X");
             self.command_args.push("POST");
             self.command_args.push(api_endpoint);
             return Ok(self);
         }
         Err("Endpoint already set!".into())
     }

    pub async fn execute(&mut self) -> String {
        let result = self.command
            .args(&*self.command_args)
            .output()
            .await
            .unwrap()
            .stdout;

        String::from_utf8(result).unwrap()

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


