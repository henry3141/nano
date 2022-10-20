use rocket::serde::{Serialize, Deserialize};
use std::{sync::{mpsc::{Receiver,Sender, self}, Mutex, Arc, MutexGuard, TryLockError}, hash::Hash, collections::HashMap};
use std::{thread, time};
#[macro_use] extern crate rocket;

#[derive(Debug,Clone)]
pub struct Video<T:Copy> {
    main:T,
    from:T,
}

impl<T:Copy> Video<T> {
    fn from(from:&T) -> Video<T> {
        Video {from:*from,main:*from}
    }
}



#[derive(Serialize, Deserialize,Clone,Debug,PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum Message {
    Basic(String),
    OkSend,
    WebSend(Box<Message>,i32), 
    WebRec(Box<Message>),
    WebReg(i32,Turtle),
    OK(i32),
    NanoReg(i32,Turtle,String),
    BAD(i32),
    PANIC(String),
}

#[derive(Serialize, Deserialize,Clone,Debug,Copy,PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum Item {
    Stone(i32),
}

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Turtle {
    pos:[i32;3],
    items:Vec<Item>,
    fuel:i32,
    recv:Vec<Message>,
}

impl Turtle {
    pub fn new(pos:[i32;3],items:Vec<Item>,fuel:i32) -> Turtle {
        Turtle { pos: pos, items: items, fuel: fuel, recv: vec![] }
    }
}


#[derive(Clone,Debug)]
pub struct Task {
    turtle:Arc<Mutex<Turtle>>,
    main:Arc<Mutex<Receiver<Message>>>,
    thread:Arc<Mutex<Sender<Message>>>,
    messages:Vec<Message>,
}

impl Task {
    pub fn WebSend(task:Arc<Mutex<Task>>,data:Message,id:i32) {
        task.lock().unwrap().thread.lock().unwrap().send(Message::WebSend(Box::new(data),id));
        loop {
            wait(20);
            let mut guard = task.try_lock();
            if guard.is_ok() {
                let mut guard = guard.unwrap();
                let mut index:usize = 0;
                for i in &guard.messages {
                    if i == &Message::OkSend {
                        guard.messages.remove(index);
                        return;
                    }
                    index = index + 1;
                }
            }
        }
    }
}

pub fn wait(time:u64) {
    thread::sleep(time::Duration::from_millis(time));
}

#[derive(Debug,Clone)]
pub enum Block {
    Grass,
}

#[derive(Clone,Debug)]
pub struct Nano {
    tasks:Vec<(Arc<Mutex<Receiver<Message>>>,Arc<Mutex<Sender<Message>>>,i32)>,
    world:HashMap<(i32,i32,i32),Block>,
    to_send:Vec<(i32,Message)>,
}

impl Nano {
    pub fn new() -> Nano {
        Nano {world:HashMap::new(),tasks:vec![],to_send:vec![]}
    }

    pub fn nano(mut self,send:Sender<Message>,recv:Receiver<Message>,kinds:HashMap<String,fn(Arc<Mutex<Task>>)>) {
        thread::spawn(move || {
            for i in recv {
                match &i {
                    Message::NanoReg(id,turtle,kind) => {
                        let handler = kinds.get(kind);
                        if handler.is_none() {
                            self.PANIC(format!("Tryed adding unknown turtle type!\nConsider adding type:{:#?} to fix this problem when running Nano::nano",&kind));
                            panic!("Tryed adding unknown turtle type!\nConsider adding type:{:#?} to fix this problem when running Nano::nano",kind);
                        }
                        self.add(handler.unwrap().clone(),turtle.clone(),id.clone());
                    },
                    _ => {},
                }
            }

        });
    }

    pub fn PANIC(&mut self,s:String) {
        for i in &self.tasks {
            i.1.lock().unwrap().send(Message::PANIC(String::from(&s)));
        }
        panic!("{}",s);
    }

    pub fn add(&mut self,handler:fn(Arc<Mutex<Task>>),turtle:Turtle,id:i32) {
        let (mut main_rec,mut main_send) = mpsc::channel::<Message>();
        let mut main_rec = Arc::new(Mutex::new(main_rec));
        let mut main_send = Arc::new(Mutex::new(main_send));
        let (mut thread_rec,mut thread_send) = mpsc::channel::<Message>();
        let mut thread_rec = Arc::new(Mutex::new(thread_rec));
        let mut thread_send = Arc::new(Mutex::new(thread_send));
        let mut task = Arc::new(Mutex::new(Task {
            turtle:Arc::new(Mutex::new(turtle)),
            main:main_send,
            thread:thread_rec,
            messages:vec![],
        }));
        self.tasks.push((thread_send,main_rec,id));
        let mut t1 = task.clone();
        thread::spawn(move || {
            (handler)(t1);
        });
        let mut t1 = task.clone();
        thread::spawn(move || {
            loop {
                wait(10);
                let mut guard = t1.lock().unwrap();
                let mut main = guard.main.try_lock();
                if main.is_ok() {
                    let mut main = main.unwrap().try_recv();
                    if main.is_ok() {
                        guard.messages.push(main.unwrap());
                    }
                }
            }
        });
    }
}
