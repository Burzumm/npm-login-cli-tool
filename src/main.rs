use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::Path;

use std::fs::File;
use std::io::prelude::*;
use tokio::time::Duration;

const NPM_FILE_MAME: &str = ".npmrc";

#[tokio::main]
async fn main() {
    let mut args = Args::parse();
    let request_url = format!("{}-/user/org.couchdb.user:npm-user", args.npm_url);
    let client = reqwest::Client::new();
    let request = NpmLoginRequest {
        name: args.username.to_string(),
        password: args.password.to_string(),
    };

    let response = client
        .put(request_url)
        .json(&request)
        .timeout(Duration::from_secs(10))
        .send()
        .await;

    let result_response = match response {
        Ok(result) => result,
        Err(err) => panic!("{}", format!("response error: {}", err).red()),
    };
    if !result_response.status().is_success() {
        panic!(
            "{}",
            format!(
                "response status code: {}, response: {:?}",
                &result_response.status(),
                &result_response
            )
            .red()
        )
    }
    let token = result_response
        .json::<NpmResponse>()
        .await
        .unwrap_or_else(|error| {
            panic!(
                "{}",
                format!(
                    "error status code:{}, {}",
                    if error.is_status() {
                        error.status().unwrap().to_string()
                    } else {
                        " ".to_string()
                    },
                    error
                )
            )
        });

    let result_output_path_str = match &args.output_file_path.chars().last().unwrap() {
        '/' => {
            args.output_file_path
                .remove(&args.output_file_path.len() - 1)
                .to_string();
            format!("{}/{}", &args.output_file_path, NPM_FILE_MAME)
        }
        '\\' => {
            args.output_file_path
                .remove(&args.output_file_path.len() - 1)
                .to_string();
            format!("{}\\{}", &args.output_file_path, NPM_FILE_MAME)
        }
        _ => {
            format!("{}/{}", &args.output_file_path, NPM_FILE_MAME)
        }
    };
    let result_output_path = Path::new(&result_output_path_str)
        .as_os_str()
        .to_str()
        .unwrap();
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(format!("{}", &result_output_path))
        .unwrap_or_else(|error| {
            panic!(
                "{}",
                format!("error open file: {}, error: {}", &result_output_path, error).red()
            )
        });
    let file_data = format!(
        "registry={}\n//{}:_authToken={}\n",
        args.npm_url,
        args.npm_url.replace("https://", ""),
        token.token
    );
    file.write_all(file_data.as_bytes())
        .unwrap_or_else(|error| {
            panic!(
                "{}",
                format!(
                    "write data: {} to file: {} error: {}",
                    file_data, &result_output_path, error
                )
                .red()
            )
        });
    println!(
        "{}, to path: {}",
        "Write token Success".green(),
        &result_output_path.green()
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct NpmLoginRequest {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NpmResponse {
    token: String,
}

#[derive(Parser, Debug)]
#[command(author="Nikita Vezhlivtsev", version, about, long_about = None)]
struct Args {
    /// Npm username
    #[arg(long, short)]
    username: String,

    /// Npm user password
    #[arg(long, short)]
    password: String,

    /// Npm Url
    #[arg(long, short)]
    npm_url: String,

    /// Output file path(default ./)
    #[arg(long, short, default_value = "./")]
    output_file_path: String,
}
