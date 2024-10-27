use aspirin_eats::db::AspirinEatsDb;
use aspirin_eats::tcp::read_http_packet_tcp_stream;
use anyhow::Result;
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};


const DB_FILE_NAME: &str = "aspirin_eats.db";
const BIND_ADDRESS: &str = "127.0.0.1:8080";


// gets the db root path from the environment variable DB_ROOT_PATH or the cargo manifest directory
fn get_db_path() -> Result<String> {
    let db_root_path: Result<String> = std::env::var("DB_ROOT_PATH").or_else(|_| {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        Ok(format!("{}/db", manifest_dir))
    });
    let db_root_path = db_root_path?;
    Ok(format!("{}/{}", db_root_path, DB_FILE_NAME))
}

// gets the db from the db path 
// if in test mode then returns a new in memory db
fn get_db() -> Result<AspirinEatsDb> {
    match cfg!(test) {
        true => Ok(AspirinEatsDb::in_memory()?),
        false => {
            let db_path = get_db_path()?;
            // Create the directory if it doesn't exist
            std::fs::create_dir_all(std::path::Path::new(&db_path).parent().unwrap())?;
            Ok(AspirinEatsDb::from_path(db_path)?)
        }
    }
}

fn handle_connection(mut stream: TcpStream, db: &AspirinEatsDb) -> Result<()> {
    println!("Handling connection");

    let lines = read_http_packet_tcp_stream(&mut stream)?;
    for line in lines {
        println!("Line: {:?}", line);
    }
    Ok(())
}

fn main() -> Result<()> {
    let db = get_db()?;
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream, &db)?;
    }
    Ok(())
}
