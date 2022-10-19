use rocket::serde;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::sync::{mpsc::{Receiver,Sender}, Mutex, Arc};
use rocket_contrib::json::Json;
use nano::Message;

fn main()  {
}