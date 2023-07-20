use std::path::{Path, PathBuf};
use clap::{Arg, arg, Command, value_parser};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Plugin {
    id: String,
    name: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Edge {
    node: Plugin
}

#[derive(Debug, Deserialize, Serialize)]
struct Edges {
    edges: Vec<Edge>
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectInfo {
    id: String,
    name: String,
    plugins: Edges
}

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    project: ProjectInfo
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectInfoResponse {
    data: Project
}
fn main() {
    let cmd = clap::Command::new("railway-db-backup")
        .bin_name("railway-db-backup")
        .subcommand_required(true)
        .subcommand(
            Command::new("backup")
                .about("Backup a Railway database")
                .arg(
                    arg!(-o --output <PATH> "The final location to dump the backup data")
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    arg!(-d --database <ID> "The database to backup")
                        .required(true)
                )
        );

    let matches = cmd.get_matches();

    let backup_matches = matches.subcommand_matches("backup").unwrap();

    let db_and_output = match backup_matches {
        Matches => {
            let database  = Matches.get_one::<String>("database").expect("Please provide a valid Railway database ID");
            let output = Matches.get_one::<PathBuf>("output").expect("Please provide a valid output path");
            println!("Backing up {} to {}", database, output.to_str().unwrap());
            (database, output)
        }
    };

    let valid_uuid = match uuid::Uuid::parse_str(db_and_output.0.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => {
            println!("Invalid Railway database project ID passed. Please provide a valid Railway database ID");
            std::process::exit(1);
        }
    };

    let token = match std::env::var("RAILWAY_API_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("Failed to fetch Railway API token from environment. Please set the RAILWAY_API_TOKEN environment variable");
            std::process::exit(1);
        }
    };

    println!("Valid Database Project UUID passed...: {}\n", valid_uuid);
    let (database, output) = db_and_output;

    let query = format!(r#"
        query project {{
          project(id: "{}") {{
            id
            name
            plugins {{
              edges {{
                node {{
                  id
                  name
                }}
              }}
            }}
          }}
        }}
    "#, database);
    let client = Client::new();

    let body = &serde_json::json!({
         "query": query,
         "operationName": "project",
        });

    let request = client.post("https://backboard.railway.app/graphql/v2")
        // todo: implement fetching keys from env var
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(body);

    let req_clone = request.try_clone().unwrap().build().unwrap();
    let headers = req_clone
        .headers()
        .clone();

    let resp_body = req_clone
        .body()
        .unwrap();

    let response = request
        .send()
        .expect("Failed to make request to Railway API");

    let status = response.status();
    if !status.is_success() {
        println!("Failed to fetch project info from Railway API. Status: {}", status);
        std::process::exit(1);
    }

    let resp_body = response
        .json::<ProjectInfoResponse>()
        .expect("Failed to parse response from Railway API");

    println!("Response: {:?}", resp_body);

}

