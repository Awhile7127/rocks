// Clap used for CLI argument parsing
use clap::Parser;

// HTTP request crate
use isahc;

// Standard Libraries
use std::thread;
use std::time::Duration;

// Time handling libraries
use chrono::Local;

// SQLITE database handling
use rusqlite::Connection;

#[derive(Parser, Debug)]
struct Args {
    
    #[clap(long="url", short='u', default_value="", required=true)]
    url: String,

    #[clap(long="rate", short='r', default_value="60", required=false)]
    rate: u64
}

// Handle making HTTP requests
fn request(url: String) -> isahc::http::StatusCode {
    let resp = isahc::get(url).expect("Could not find URL")
        .status();
    return resp;
}

// Update an external database
fn update_database(status: String) {

    // Connect to database
    let conn = Connection::open("statuses.db");

    // Get the current timestamp
    let time: String = Local::now().to_string();

    // Declare query
    let query = "INSERT INTO statuses (time, status) VALUES (?1, ?2);";

    // Execute query on database
    let _result = conn.expect("No connection found").execute(query, (status.clone(), time.clone(),));
}

fn main() {

    // Fetch command line arguments
    // Declare rate limiting
    let args = Args::parse();
    let rate = Duration::new(args.rate, 0);

    // Repeatedly send requests to the server
    // and update an SQL database
    loop {
        let resp = request(args.url.clone()).to_string();
        update_database(resp);
        thread::sleep(rate);
    }
}
