
use actix_web::{get, post, web, HttpResponse, Responder};
use std::fs::{self, File};
use std::io::{self,Write};
use crate::models::{Entry, HealthResponse};

const FILE_PATH: &str = "entries.json";

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: String::from("Server is running"),
    })
}

fn ensure_file_exists(file_path : &str) -> io::Result<()> {
    if !fs::metadata(file_path).is_ok() {
        let mut file = File::create(file_path)?;
        write!(file, "[]")?;
    }
    Ok(())
}


fn read()-> Vec<Entry> {

    ensure_file_exists(FILE_PATH).unwrap();

    let data = fs::read_to_string(FILE_PATH);

    match data {
        Ok(content) => {
            match serde_json::from_str(&content) {
                Ok(entries) => entries,
                Err(_) => Vec::new(),
            }
        }
        Err(_) => Vec::new(),
    }
}


#[post("/add_entry")]
async fn add_entry(entry: web::Json<Entry>) -> impl Responder {
   
    let mut entries = read(); 
    let mut new_entry = entry.into_inner();

    new_entry.id = Some((entries.len() as u16) + 1);

    entries.push(new_entry.clone());  

    let json_data = serde_json::to_string(&entries).unwrap();
    fs::write(FILE_PATH, &json_data).unwrap();

    HttpResponse::Ok().json(new_entry) 
}

#[get("/entries")]
async fn get_entries()->impl Responder{
    let entries = read();
    HttpResponse::Ok().json(entries)
}