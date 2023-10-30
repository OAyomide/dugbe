use std::ffi::OsStr;
use std::path::{PathBuf};
use std::time::Instant;
use clap::{arg, Command, value_parser};
use cursive::{views::TextView};
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
    dotenv::dotenv().ok();
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    // println!("🚂 DUGBE: A RAILWAY DB BACKUP TOOL. THIS TOOL IS ALPHA QUALITY. ITS SAFE ENOUGH TO USE BUT ITS NOT BEEN SO BATTLE TESTED FOR THE SCALE OF THE USAGE (PRODUCTION DB BACKUP ON RAILWAY INFRA.). PLEASE REPORT ANY ISSUES AND BE PATIENT WITH ISSUES THAT MAY ARISE.");
    siv.add_layer(
        TextView::new("🚂 DUGBE: A RAILWAY DB BACKUP TOOL. THIS TOOL IS ALPHA QUALITY. ITS SAFE ENOUGH TO USE BUT ITS NOT BEEN SO BATTLE TESTED FOR THE SCALE OF THE USAGE (PRODUCTION DB BACKUP ON RAILWAY INFRA.). PLEASE REPORT ANY ISSUES AND BE PATIENT WITH ISSUES THAT MAY ARISE.")
            .h_align(cursive::align::HAlign::Center)
    );

//     let cmd = clap::Command::new("railway-db-backup")
//         .bin_name("railway-db-backup")
//         .subcommand_required(true)
//         .subcommand(
//             Command::new("backup")
//                 .about("Backup a Railway database")
//                 .arg(
//                     arg!(-o --output <PATH> "The final location to dump the backup data. Must be an sql file.")
//                         .required(true)
//                         .value_parser(value_parser!(PathBuf))
//                 )
//                 .arg(
//                     arg!(-d --database <URL> "The database we want to backup.")
//                         .required(true)
//                         .value_parser(value_parser!(String))
//                 )
//         );
//
//     let matches = cmd.get_matches();
//
//     let backup_matches = matches.subcommand_matches("backup").unwrap();
//
//     let backup_arguments = match backup_matches {
//         bk_matches => {
//             let output = bk_matches.get_one::<PathBuf>("output").expect("Please provide a valid output path");
//             let database = bk_matches.get_one::<String>("database").expect("Please provide a valid Railway database URL.");
//             (database, output)
//         }
//     };
//
//     match backup_arguments.1.as_path().extension().and_then(OsStr::to_str) {
//         Some("sql") => {},
//         _ => {
//             println!("Invalid output file extension. Please provide a valid output file extension. The filename you specified is not an sql file. You specified: {}", backup_arguments.1.as_path().display());
//             std::process::exit(1);
//         }
//     }
//
//     let (database, output) = backup_arguments;
//     let database_info = match url::Url::parse(database.as_str()) {
//         Ok(url) => {
//             println!("Valid Database URL passed...: {}", url);
//             url
//         },
//         Err(_) => {
//             println!("Invalid Railway database URL passed. Please provide a valid Railway database URL.");
//             std::process::exit(1);
//         }
//     };
//     println!("Database connection info: {:?}", database_info);
//
//     if database_info.scheme() != "postgresql" {
//         println!("Invalid Railway database URL passed. Please provide a valid Railway database URL.11");
//         std::process::exit(1);
//     }
//
//
//     let token = match std::env::var("RAILWAY_API_TOKEN") {
//         Ok(token) => token,
//         Err(_) => {
//             println!("Failed to fetch Railway API token from environment. Please set the RAILWAY_API_TOKEN environment variable");
//             std::process::exit(1);
//         }
//     };
//
//
//     // check if pg_dump is installed. Please make sure pg_dump is installed before running this tool.
//     let pg_dump = match which::which("pg_dump") {
//         Ok(path) => path,
//         Err(_) => {
//             println!("pg_dump not found. Please install pg_dump and try again");
//             std::process::exit(1);
//         }
//     };
//
//
//     let db_host = database_info.host().unwrap().to_string();
//     let db_port = database_info.port().unwrap_or(5432).to_string();
//     let database_name = database_info.path().trim_start_matches("/");
//
//     let pg_dump_args = vec![
//         "-h", db_host.as_str(),
//         "-p", db_port.as_str(),
//         "-U", "postgres",
//         "-d", database_name,
//         "-f", output.to_str().unwrap()
//     ];
//
//     // start a timer
//     let start = Instant::now();
//     let pg_dump = std::process::Command::new(pg_dump)
//         .args(&pg_dump_args)
//         .env("PGPASSWORD", database_info.password().unwrap())
//         .output()
//         .expect("Failed to run pg_dump");
// let end = start.elapsed();
//
//     let time_spent = format!("{}.{}s", end.as_secs(), end.subsec_millis());
//     if !pg_dump.status.success() {
//         let error = String::from_utf8_lossy(&pg_dump.stderr);
//         println!("Failed to run pg_dump. There seems to be an error connecting to the database. Please check your connection info and try again.");
//         println!("{:?}", error.to_string());
//         println!("Time spent: {}", time_spent);
//         std::process::exit(1);
//     }
//
//
//     println!("pg_dump output: {:?}", pg_dump);
//     println!("Done backing up: Took {}", time_spent);
    siv.run();
}

