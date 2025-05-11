use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::fs;
use actix_web::{get,post};

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
    Entertainment,
    Others,
    Cloth,
}

#[derive(Deserialize, Serialize,Clone)]
struct Entry {
    id: u16,
    title : String,
    amount:u16,
    date: u16,
    basket : Basket,
    category: Category,
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: String::from("Server is running"),
    })
}

fn read()-> Vec<Entry>{
    let file_path = "entries.json";
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

#[post("/add_entry")]
async fn add_entry(entry: web::Json<Entry>) -> impl Responder {
   
    let mut entries = read(); 
    let mut new_entry = entry.into_inner();

    entries.push(new_entry.clone());  

    new_entry.id = (entries.len() as u16) + 1;

    let file_path = "entries.json";

    let json_data = serde_json::to_string(&entries).unwrap();
    fs::write(file_path, json_data).unwrap();

    let json_data = serde_json::to_string_pretty(&entries).unwrap();

    HttpResponse::Ok().json(json_data) 
}



#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(add_entry)
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
