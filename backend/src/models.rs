use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct HealthResponse {
   pub status: String,
}

#[derive(Serialize, Deserialize,Clone)]
pub enum Basket {
    Daily,
    Entertainment,
    Savings,
    Investment
}

#[derive(Serialize, Deserialize,Clone)]
pub enum Category {
    FoodAndDrink,
    Transportation,
    Entertainment,
    Others,
    Cloth,
}

#[derive(Deserialize, Serialize,Clone)]
pub struct Entry {
    pub id: Option<u16>,
     title: String,
     amount: u16,
     date: u16,
     basket: Basket,
     category: Category,
}