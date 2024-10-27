use crate::{db::AspirinEatsDb, error::AspirinEatsError, http::{HttpRequest, HttpResponse}};
use anyhow::Result;
use serde_json::Value;
use regex::Regex;

pub trait PathHandler {
    fn handle(&self, request: &HttpRequest, db: &AspirinEatsDb) -> Result<HttpResponse>;
    fn matches(&self, method: &str, path: &str) -> Result<Box<dyn PathHandler>>;
}

// path "/" returns a welcome message
pub struct RootPathHandler;

impl PathHandler for RootPathHandler {
    fn handle(&self, _request: &HttpRequest, _db: &AspirinEatsDb) -> Result<HttpResponse> {
        Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: "".to_string() })
    }

    fn matches(&self, method: &str, path: &str) -> Result<Box<dyn PathHandler>> {
        if method == "GET" && path == "/" {
            Ok(Box::new(RootPathHandler))
        } else {
            Err(anyhow::anyhow!("failed to match"))
        }
    }
}

// path "/orders" returns all orders
pub struct GetOrdersPathHandler;

impl PathHandler for GetOrdersPathHandler {
    fn handle(&self, _request: &HttpRequest, _db: &AspirinEatsDb) -> Result<HttpResponse> {
        Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: "".to_string() })
    }

    fn matches(&self, method: &str, path: &str) -> Result<Box<dyn PathHandler>> {
        if method == "GET" && path == "/orders" {
            Ok(Box::new(GetOrdersPathHandler))
        } else {
            Err(anyhow::anyhow!("failed to match"))
        }
    }
}

// path "/orders/{id}" returns an order with the given id
pub struct GetOrderWithIdPathHandler {
    pub id: i32,
}

impl PathHandler for GetOrderWithIdPathHandler {
    fn handle(&self, _request: &HttpRequest, _db: &AspirinEatsDb) -> Result<HttpResponse> {
        Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: "".to_string() })
    }

    fn matches(&self, method: &str, path: &str) -> Result<Box<dyn PathHandler>> {
        let re = Regex::new(r"/orders/(\d+)")?;
        match (method, re.captures(path)) {   
            ("GET", Some(captures)) => {
                let id: i32 = captures[1].parse()?;
                Ok(Box::new(GetOrderWithIdPathHandler { id }))
            }
            _ => Err(anyhow::anyhow!("failed to match"))
        }
    }
}


// path "/orders" with POST method creates a new order
pub struct CreateOrderPathHandler;

impl PathHandler for CreateOrderPathHandler {
    fn handle(&self, request: &HttpRequest, _db: &AspirinEatsDb) -> Result<HttpResponse> {
        println!("CreateOrderPathHandler");
        

        if let Some(body) = serde_json::from_str(&request.body.) else {
            println!("Body: {:?}", body);
        }
        Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: "".to_string() })
    }

    fn matches(&self, method: &str, path: &str) -> Result<Box<dyn PathHandler>> {
        if method == "POST" && path == "/orders" {
            Ok(Box::new(CreateOrderPathHandler))
        } else {
            Err(anyhow::anyhow!("failed to match"))
        }
    }
}

// path "/orders" with DELETE methods deletes all orders
pub struct DeleteOrdersPathHandler;

impl PathHandler for DeleteOrdersPathHandler {
    fn handle(&self, _request: &HttpRequest, _db: &AspirinEatsDb) -> Result<HttpResponse> {
        Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: "".to_string() })
    }

    fn matches(&self, method: &str, path: &str) -> Result<Box<dyn PathHandler>> {
        if method == "DELETE" && path == "/orders" {
            Ok(Box::new(DeleteOrdersPathHandler))
        } else {
            Err(anyhow::anyhow!("failed to match"))
        }
    }
}


// path "/orders/{id}" with DELETE method deletes an order with the given id
pub struct DeleteOrderWithIdPathHandler {
    pub id: i32,
}

impl PathHandler for DeleteOrderWithIdPathHandler {
    fn handle(&self, _request: &HttpRequest, _db: &AspirinEatsDb) -> Result<HttpResponse> {
        Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: "".to_string() })
    }

    fn matches(&self, method: &str, path: &str) -> Result<Box<dyn PathHandler>> {
        let re = Regex::new(r"/orders/(\d+)")?;
        match (method, re.captures(path)) {
            ("DELETE", Some(captures)) => {
                let id: i32 = captures[1].parse()?;
                Ok(Box::new(DeleteOrderWithIdPathHandler { id }))
            }
            _ => Err(anyhow::anyhow!("failed to match"))
        }
    }
}