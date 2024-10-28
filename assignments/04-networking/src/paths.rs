use crate::{db::AspirinEatsDb, error::AspirinEatsError, food::{Order, OrderRequest}, http::{HttpRequest, HttpResponse}};
use anyhow::Result;
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
    fn handle(&self, _request: &HttpRequest, db: &AspirinEatsDb) -> Result<HttpResponse> {
        match db.get_all_orders() {
            Ok(orders) => Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: serde_json::to_string(&orders)? }),
            Err(e) => Ok(HttpResponse::from(AspirinEatsError::Database(e)))
        }
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
    fn handle(&self, _request: &HttpRequest, db: &AspirinEatsDb) -> Result<HttpResponse> {
        match db.get_order(self.id.into()) {
            Ok(order) => Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: serde_json::to_string(&order)? }),
            Err(e) => Ok(HttpResponse::from(AspirinEatsError::Database(e)))
        }
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
    fn handle(&self, request: &HttpRequest, db: &AspirinEatsDb) -> Result<HttpResponse> {
        println!("CreateOrderPathHandler");

        match serde_json::from_str::<OrderRequest>(&request.body) {
            Ok(order_request) => {
                match db.add_order(order_request.into()) {
                    Ok(order_id) => {
                        println!("Order created with id {}", order_id);
                        Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: format!("created order with id {order_id}\n") })
                    }
                    Err(e) => {
                        Ok(HttpResponse::from(AspirinEatsError::Database(e)))
                    }
                }
            }
            Err(e) => {
                println!("Error parsing order: {:?}", e);
                Ok(HttpResponse::from(AspirinEatsError::ParseError(e)))
            }
        }
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
    fn handle(&self, _request: &HttpRequest, db: &AspirinEatsDb) -> Result<HttpResponse> {
        match db.reset_orders() {
            Ok(_) => Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: "reset all orders".to_string() }),
            Err(e) => Ok(HttpResponse::from(AspirinEatsError::Database(e)))
        }
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
    fn handle(&self, _request: &HttpRequest, db: &AspirinEatsDb) -> Result<HttpResponse> {
        match db.remove_order(self.id.into()) {
            Ok(_) => Ok(HttpResponse { status_code: 200, status_text: "OK".to_string(), body: format!("deleted order with id {}\n", self.id) }),
            Err(e) => Ok(HttpResponse::from(AspirinEatsError::Database(e)))
        }
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


mod tests {
    use crate::food::{Bun, Burger, MenuItem, Patty, Topping};

    use super::*;

    fn order_req_one() -> OrderRequest {
        OrderRequest {
            customer: String::from("John Doe"),
            food: vec![MenuItem::Burger(Burger {
            bun: Bun::Sesame,
            patty: Patty::Beef,
                toppings: vec![Topping::Lettuce, Topping::Tomato]
            })],
        }
    }

    #[test]
    fn test_all_path_handler() {
        let db = AspirinEatsDb::in_memory().unwrap();
        let handler = GetOrdersPathHandler;
        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/orders".to_string(),
            body: "".to_string(),
        };
        let response = handler.handle(&request, &db).unwrap();
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, "[]");

        // add 100 orders and check that the response is correct
        for i in 1..100 {
            db.add_order(order_req_one().into()).unwrap();
            let response = handler.handle(&request, &db).unwrap();
            assert_eq!(response.status_text, "OK");
            assert_eq!(response.status_code, 200);
            assert_eq!(response.body, serde_json::to_string(&db.get_all_orders().unwrap()).unwrap());

            let handler_specific = GetOrderWithIdPathHandler { id: i };
            let response_specific = handler_specific.handle(&request, &db).unwrap();
            assert_eq!(response_specific.status_text, "OK");
            assert_eq!(response_specific.status_code, 200);
            assert_eq!(response_specific.body, serde_json::to_string(&db.get_order(i.into()).unwrap()).unwrap());
        }

        // delete all orders and check that the response is correct
        let delete_handler = DeleteOrdersPathHandler;
        let response_delete = delete_handler.handle(&request, &db).unwrap();
        assert_eq!(response_delete.status_text, "OK");
        assert_eq!(response_delete.status_code, 200);
        assert_eq!(response_delete.body, "reset all orders");
        assert_eq!(db.get_all_orders().unwrap().len(), 0);

        // add the 100 orders back and check that the response is correct
        for i in 1..101 {
            db.add_order(order_req_one().into()).unwrap();
            let response = handler.handle(&request, &db).unwrap();
            assert_eq!(response.status_text, "OK");
            assert_eq!(response.status_code, 200);
            assert_eq!(response.body, serde_json::to_string(&db.get_all_orders().unwrap()).unwrap());

            let handler_specific = GetOrderWithIdPathHandler { id: i };
            let response_specific = handler_specific.handle(&request, &db).unwrap();
            assert_eq!(response_specific.status_text, "OK");
            assert_eq!(response_specific.status_code, 200);
            assert_eq!(response_specific.body, serde_json::to_string(&db.get_order(i.into()).unwrap()).unwrap());
        }

        // delete the orders one by one and check that the response is correct
        for i in 1..101 {
            let delete_handler_specific = DeleteOrderWithIdPathHandler { id: i };
            let response_delete_specific = delete_handler_specific.handle(&request, &db).unwrap();
            assert_eq!(response_delete_specific.status_text, "OK");
            assert_eq!(response_delete_specific.status_code, 200);
            assert_eq!(response_delete_specific.body, format!("deleted order with id {}\n", i));
            assert_eq!(db.get_all_orders().unwrap().len(), 100 - i as usize);
        }
    }

    #[test]
    fn test_path_matches_root() {
        let handler = RootPathHandler;
        assert!(handler.matches("GET", "/").is_ok());
        assert!(handler.matches("GET", "/orders").is_err());
    }

    #[test]
    fn test_path_matches_orders() {
        let handler = GetOrdersPathHandler;
        assert!(handler.matches("GET", "/orders").is_ok());
        assert!(handler.matches("POST", "/orders").is_err());
        assert!(handler.matches("DELETE", "/orders").is_err());
        assert!(handler.matches("GET", "/").is_err());
        assert!(handler.matches("GET", "/orders/1").is_err());
    }

    #[test]
    fn test_path_matches_order_with_id() {
        let handler = GetOrderWithIdPathHandler { id: 1 };
        assert!(handler.matches("GET", "/orders/1").is_ok());
        assert!(handler.matches("GET", "/orders").is_err());
        assert!(handler.matches("GET", "/orders/abc").is_err());
    }

    #[test]
    fn test_path_matches_delete_order_with_id() {
        let handler = DeleteOrderWithIdPathHandler { id: 1 };
        assert!(handler.matches("DELETE", "/orders/1").is_ok());
        assert!(handler.matches("DELETE", "/orders").is_err());
        assert!(handler.matches("GET", "/orders/1").is_err());
        assert!(handler.matches("DELETE", "/orders/abc").is_err());
    }
    #[test]
    fn test_path_matches_delete_orders() {
        let handler = DeleteOrdersPathHandler;
        assert!(handler.matches("DELETE", "/orders").is_ok());
        assert!(handler.matches("GET", "/orders").is_err());
        assert!(handler.matches("POST", "/orders").is_err());
        assert!(handler.matches("DELETE", "/").is_err());
        assert!(handler.matches("DELETE", "/orders/1").is_err());
    }
}
