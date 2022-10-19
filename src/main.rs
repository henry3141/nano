use rocket::serde::{Serialize, Deserialize};
use std::sync::{mpsc::{Receiver,Sender}, Mutex, Arc};
#[macro_use] extern crate rocket;


#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
