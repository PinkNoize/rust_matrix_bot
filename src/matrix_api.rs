extern crate fractal_matrix_api;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use matrix_api::fractal_matrix_api::backend::Backend;
use matrix_api::fractal_matrix_api::backend::BKResponse;
use matrix_api::fractal_matrix_api::backend::BKCommand;

pub struct matrix_api {
    config: HashMap<String,String>,
    messages: Vec<String>,
    rx: Receiver<BKResponse>,
    apptx: Sender<BKCommand>,
    since: Option<String>,
    syncing: bool
}
impl matrix_api {
    pub fn new() -> matrix_api {
        let (tx, rx): (Sender<BKResponse>, Receiver<BKResponse>) = channel();
        let bk=Backend::new(tx);
        let apptx: Sender<BKCommand> = bk.run();
        let config=HashMap::new();
        let messages=Vec::new();

        //load config
        
        matrix_api {
            messages,
            config,
            apptx,
            rx,
            since: Option::new(),
            syncing: false
        }
    }
    pub fn start(&self) {
        let user=
            match self.config.get("user") {
                Some(usr) => usr,
                None => panic!("User not found")
            };
        let password=
            match self.config.get("password") {
                Some(usr) => usr,
                None => panic!("Password not found")
            };
        let server=
            match self.config.get("server") {
                Some(usr) => usr,
                None => panic!("Server not found")
            };
        self.apptx.send(BKCommand::Login(user.to_string(),password.to_string(),server.to_string()));
    }

    pub fn send(&self,message: String) {
        
    }

    pub fn receive(& mut self) -> Option<String>{
        let response=self.rx.recv();
        //some taken from fractal-gtk
        match response {
            Ok(BKResponse::Rooms(rooms, default)) => {
                self.rooms=rooms.iter().filter(|x| !x.left).cloned().collect();
            }
            Ok(BKResponse::Sync(since)) => {
                self.syncing=false;
                self.since=since;
            }
            OK(BKResponse::RoomMembers(room, members)) => {
            
            }

            //errors
            Ok(BKResponse::NewRoomError(err, internal_id)) => {
                println!("ERROR: {:?}", err);
            },
            Ok(BKResponse::JoinRoomError(err)) => {
                println!("ERROR: {:?}", err);
            },
            Ok(BKResponse::LoginError(_)) => {
                println!("ERROR: {:?}",err);
            },
            Ok(BKResponse::SendMsgError(err)) => {
                match err {
                    Error::SendMsgError(txid) => {
                        println!("ERROR sending {}: retrying send", txid);
                    },
                    _ => {
                        println!("Error sending message");
                    }
                }
            }
            Ok(BKResponse::SyncError(err)) => {
                self.syncing=false;
                println!("SYNC Error: {:?}", err);
            }
            Ok(err) => {
                println!("Query error: {:?}", err);
            }
            Err(RecvError) => { break; }
        }
    }
}
