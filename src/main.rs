use rocket::serde;
use rocket::{State};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::sync::{mpsc::{Receiver,Sender,channel}, Mutex, Arc};
use std::thread;
use rocket::serde::json::{Json,to_string};
use nano::{Nano,Message,Turtle};
#[macro_use] extern crate rocket;


#[post("/register", data = "<user>")]
fn register(user: Json<Message>,send:&State<Arc<Mutex<Sender<Message>>>>) -> Json<Message> {
    match user.into_inner() {
        Message::WebReg(id,turtle) => {
            send.lock().unwrap().send(Message::NanoReg(id, turtle, "worker".to_string()));
            println!("CONNECT!");
            Json(Message::OK(202))
        },
        _ => {Json(Message::BAD(202))},
    }
}

//#[get("/com/<id>")]
//fn get_com(id:i32) -> Json<Message> {
//
//}

#[post("/echo", data = "<string>")]
fn echo(string:String) -> String {
    println!("{:#?}",&string);
    string
}

#[launch]
fn rocket() -> _ {
    let mut data = Nano::new();
    let (w_sender,n_recv) = channel::<Message>();
    let (n_sender,w_recv) = channel::<Message>();
    let w_sender = Arc::new(Mutex::new(w_sender));
    let mut messages = Arc::new(Mutex::new(vec![]));
    let mut m2 = messages.clone();
    thread::spawn(move || {
        for i in w_recv {
            match i {
                Message::PANIC(s) => {
                    messages.lock().unwrap().push(Message::PANIC(String::from(&s)));
                    panic!("{}",s);
                },
                _ => {messages.lock().unwrap().push(i)},
            } 
        }
    });
    let mut messages = m2;
    data.nano(n_sender, n_recv,HashMap::new());
    //println!("{:#?}",to_string(&Message::WebReg(1,Turtle::new([0,0,0],vec![],0))));
    rocket::build()
        .mount("/", routes![register])
        .mount("/",routes![echo])
        .manage(w_sender.clone())
        .manage(messages.clone())
}

