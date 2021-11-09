use std::{collections::HashMap, hash::Hash, net::TcpListener, thread::spawn};

use tungstenite::{Message, accept, accept_hdr, handshake::server::{Request, Response}, http::response};

struct Room {
    name: String,
    players: Vec<String>,
}

pub struct Server {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub max_players: u32,
    pub current_players: u32,
    pub address: String,
    pub battle_count: u32,
    rooms: HashMap<u32, Vec<String>>,
}

impl Server {
    pub fn new(id: u32, name: String, description: String, max_players: u32, current_players: u32, address: String) -> Server {
        Server {
            id,
            name,
            description,
            max_players,
            current_players,
            address,
            battle_count: 0,
            rooms: HashMap::new(),
        }
    }

    pub fn serve(&self, log_level: u32) {
        let server = TcpListener::bind(&self.address).unwrap();
        for stream in server.incoming() {
            spawn(|| {
                let mut ws = accept(stream.unwrap()).unwrap();
                loop {
                    let msg = ws.read_message().unwrap();

                    if msg.is_text() {
                        let text = msg.into_text().unwrap();
                        //TODO: message handlers 
                    }
                }
            });
        }        
    }
}