use aspirin_eats::http::HttpResponse;
use aspirin_eats::{db::AspirinEatsDb, http::HttpRequest, error::AspirinEatsError};
use aspirin_eats::tcp::read_http_packet_tcp_stream;
use aspirin_eats::paths::{
    CreateOrderPathHandler, DeleteOrderWithIdPathHandler, DeleteOrdersPathHandler, GetOrderWithIdPathHandler, GetOrdersPathHandler, PathHandler, RootPathHandler
};
use anyhow::Result;
use regex::Regex;
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

fn handle_connection(stream: &mut TcpStream, db: &AspirinEatsDb) -> Result<()> {
    let resp: HttpResponse = create_response(stream, db).unwrap_or_else(|_| {
        HttpResponse::from(AspirinEatsError::InternalServerError)
    });
    stream.write(resp.to_string().as_bytes()).map_err(|e| AspirinEatsError::Io(e))?;
    Ok(())
}

fn create_response(mut stream: &mut TcpStream, db: &AspirinEatsDb) -> Result<HttpResponse> {
    println!("Handling connection");

    let lines = read_http_packet_tcp_stream(&mut stream)?;
    for line in lines.clone() {
        println!("Line: {:?}", line);
    }

    let Ok(request) = HttpRequest::try_from(lines) else {
        return Ok(HttpResponse::from(AspirinEatsError::InvalidRequest));
    };

    let path_handlers: Vec<Box<dyn PathHandler>> = vec![
        Box::new(RootPathHandler {}),
        Box::new(GetOrdersPathHandler {}),
        Box::new(GetOrderWithIdPathHandler { id: 0 }),
        Box::new(CreateOrderPathHandler {}),
        Box::new(DeleteOrdersPathHandler {}),
        Box::new(DeleteOrderWithIdPathHandler { id: 0 }),
    ];
    for path_handler in path_handlers {
        match path_handler.matches(&request.method, &request.path) {
            Ok(path_handler) => {
                return path_handler.handle(&request, db);
            }
            Err(_) => {}
        }
    }
    Ok(HttpResponse::from(AspirinEatsError::InvalidRequest))
}

fn main() -> Result<()> {
    let db = get_db()?;
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = handle_connection(&mut stream, &db);
            }
            // stream can be dropped by the client
            Err(_e) => {}
        }
    }
    Ok(())
}
