use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize,Clone)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize, Deserialize,Clone)]
enum Basket {
    Daily,
    Entertainment,
    Savings,
    Investment
}

#[derive(Serialize, Deserialize,Clone)]
enum Category {
    FoodAndDrink,
    Transportation,
    Entertainment
}

#[derive(Deserialize, Serialize,Clone)]
struct Entry {
    basket : Basket,
    category: Category,
    title : String,
    amount:u16,
    date: u16,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: String::from("Server is running"),
    })
}

async fn add_entry(entry: web::Json<Entry>) -> impl Responder {
    let mut entries = read();  // Step 1: Read existing entries
    let new_entry = entry.into_inner();

    entries.push(new_entry.clone());  // Step 2: Add the new entry to the list

    let file_path = "transactions.json";

    // Step 3: Write the updated list back to the file
    let json_data = serde_json::to_string(&entries).unwrap();
    fs::write(file_path, json_data).unwrap();

    // Return the added entry in the response
    HttpResponse::Ok().json(new_entry) // This is the corrected part
}

fn read()-> Vec<Entry>{
  let file_path = "transactions.json";

   let data = fs::read_to_string(file_path);

   match data {
       Ok(content) => {
           match serde_json::from_str(&content) {
               Ok(entries) => entries,
               Err(_) => Vec::new(),    
           }
       }
        Err(_) => {
         Vec::new()
       }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health)) 
            .route("/transactions", web::post().to(add_entry)) 
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
