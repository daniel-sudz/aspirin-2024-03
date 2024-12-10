use display_json::{DisplayAsJson, FromStrAsJson};
use serde::{Deserialize, Serialize};

/// Struct that represents an order
#[derive(Serialize, Deserialize, DisplayAsJson, FromStrAsJson, Debug, PartialEq, Clone)]
pub struct Order {
    /// Order ID (unique). Should be generated by the SQL database
    pub id: Option<i64>,

    /// Customer Name
    pub customer: String,

    /// Vec of all of the food items in the order
    pub food: Vec<MenuItem>,

    /// Current status of the order
    pub status: OrderStatus,

    /// Total price of the order
    pub total: f64,
}

/// Struct that represents an incoming order request to be added to the database. Separate from the
/// Order struct because many of the fields will be generated for new orders
#[derive(Deserialize, Serialize, FromStrAsJson)]
pub struct OrderRequest {
    /// Customer Name
    pub customer: String,

    /// Vec of all the food items in the order
    pub food: Vec<MenuItem>,
}

impl From<OrderRequest> for Order {
    /// Create an Order from an OrderRequest by filling in the ID, status, and total fields
    fn from(order_request: OrderRequest) -> Self {
        Order {
            id: None,
            customer: order_request.customer,
            status: OrderStatus::Pending,
            total: order_request.food.iter().map(|item| item.price()).sum(),
            food: order_request.food,
        }
    }
}

/// Enum that represents the status of an order
#[derive(Serialize, Deserialize, DisplayAsJson, FromStrAsJson, Debug, PartialEq, Clone)]
pub enum OrderStatus {
    Pending,
    Preparing,
    Transporting,
    Completed,
    Cancelled,
}

/// Enum that represents a particular menu item
#[derive(Serialize, Deserialize, DisplayAsJson, Debug, PartialEq, Clone)]
pub enum MenuItem {
    Burger(Burger),
    Fries,
    Drink,
}

impl MenuItem {
    fn price(&self) -> f64 {
        match self {
            MenuItem::Burger(burger) => burger.price(),
            MenuItem::Fries => 5.0,
            MenuItem::Drink => 3.0,
        }
    }
}

/// Struct that represents a burger
#[derive(Serialize, Deserialize, DisplayAsJson, Debug, PartialEq, Clone)]
pub struct Burger {
    pub bun: Bun,
    pub patty: Patty,
    pub toppings: Vec<Topping>,
}

impl Burger {
    pub fn new(bun: Bun, patty: Patty, toppings: Vec<Topping>) -> Burger {
        Burger {
            bun,
            patty,
            toppings,
        }
    }

    fn price(&self) -> f64 {
        self.bun.price()
            + self.patty.price()
            + self
                .toppings
                .iter()
                .map(|topping| topping.price())
                .sum::<f64>()
    }
}

/// Enum that represents a type of bun
#[derive(Serialize, Deserialize, DisplayAsJson, Debug, PartialEq, Clone)]
pub enum Bun {
    Sesame,
    Plain,
    GlutenFree,
}

impl Bun {
    fn price(&self) -> f64 {
        match self {
            Bun::Sesame => 1.0,
            Bun::Plain => 0.0,
            Bun::GlutenFree => 2.0,
        }
    }
}

/// Enum that represents a type of patty
#[derive(Serialize, Deserialize, DisplayAsJson, Debug, PartialEq, Clone)]
pub enum Patty {
    Beef,
    Chicken,
    Veggie,
}

impl Patty {
    fn price(&self) -> f64 {
        match self {
            Patty::Beef => 8.0,
            Patty::Chicken => 7.0,
            Patty::Veggie => 6.0,
        }
    }
}

/// Enum that represents a type of topping
#[derive(Serialize, Deserialize, DisplayAsJson, Debug, PartialEq, Clone)]
pub enum Topping {
    Lettuce,
    Tomato,
    Onion,
    Pickle,
    Cheese,
    Bacon,
}

impl Topping {
    fn price(&self) -> f64 {
        match self {
            Topping::Lettuce => 0.0,
            Topping::Tomato => 0.0,
            Topping::Onion => 0.0,
            Topping::Pickle => 0.0,
            Topping::Cheese => 1.0,
            Topping::Bacon => 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_from_order_request() {
        let food = vec![
            MenuItem::Burger(Burger::new(
                Bun::Sesame,
                Patty::Beef,
                vec![Topping::Cheese, Topping::Bacon],
            )),
            MenuItem::Fries,
            MenuItem::Drink,
        ];

        let order_request = OrderRequest {
            customer: "Alice".to_string(),
            food: vec![
                MenuItem::Burger(Burger::new(
                    Bun::Sesame,
                    Patty::Beef,
                    vec![Topping::Cheese, Topping::Bacon],
                )),
                MenuItem::Fries,
                MenuItem::Drink,
            ],
        };
        let order = Order::from(order_request);
        assert_eq!(
            order,
            Order {
                id: None,
                customer: "Alice".to_string(),
                status: OrderStatus::Pending,
                total: 20.0,
                food,
            }
        );
    }
}
