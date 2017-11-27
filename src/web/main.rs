#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
//extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate rand;
extern crate websocket;
extern crate tokio_core;
extern crate futures;
//extern crate ws;
extern crate pso3p;

use std::collections::{HashMap, VecDeque};

use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
//use std::sync::mpsc;
use futures::{Future, Stream, Sink};
use futures::stream::{SplitSink, SplitStream, MergedItem};
use futures::sync::mpsc;
use std::time::Duration;

use websocket::async::{Server, MessageCodec};
use websocket::message::{Message, OwnedMessage};
use websocket::server::InvalidConnection;
use websocket::client::async::Framed;
use websocket::WebSocketError;
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpStream;

use rand::{StdRng, SeedableRng, Rng};


use pso3p::action::{Action, PlayerAction};
use pso3p::pso3simulation::PSO3Simulation;
use pso3p::deck::{DeckBuilder, DeckType};
use pso3p::player::PlayerId;
use pso3p::field::Field;
use pso3p::cardlibrary::CardLibrary;
use pso3p::fieldobject::Position;
use pso3p::statechange::StateChange;



//use rocket::Request;
//use rocket::response::NamedFile;
//use rocket_contrib::Template;


type ClientId = u64;
type MatchId = u64;



static PORT: u32 = 28305;


struct Match {
    sim: PSO3Simulation,
}

impl Match {
    fn new() -> Match {
        let card_library = CardLibrary::new("./resources/cards/");

        let mut db = DeckBuilder::new()
            .faction(DeckType::Hunter)
            .character(card_library.get_by_id(1).unwrap());
        
        for c in vec![9, 12, 22, 23, 40, 44, 371, 197, 253, 246] {
            for _ in 0..3 {
                db = db.card(card_library.get_by_id(c).unwrap());
            }
        }
        let deck1 = db.deck().unwrap();

        let mut db = DeckBuilder::new()
            .faction(DeckType::Hunter)
            .character(card_library.get_by_id(2).unwrap());
        
        for c in vec![12, 22, 52, 380, 381, 382, 632, 197, 253, 246] {
            for _ in 0..3 {
                db = db.card(card_library.get_by_id(c).unwrap());
            }
        }
        let deck2 = db.deck().unwrap();    
        
        let sim = PSO3Simulation::new(Field::new(), deck1, deck2);
        
        Match {
            sim: sim,
        }
    }
}


/*struct Client<T, R, A> {
    id: ClientId,
    tx: T,
    rx: R,
    addr: A,
}


impl<T, R, A> Client<T, R, A> {
    fn new(tx: T, rx: R, addr: A) -> Client<T, R, A> {
        Client {
            id: rand::thread_rng().gen(),
            //sock: sock,
            tx: tx,
            rx: rx,
            addr: addr,
        }
    }
}*/


enum Client<T, R, A> {
    //Pending(ClientId, T, R, A),
    Pending {
        id: ClientId,
        tx: T,
        rx: R,
        addr: A,
    },
    InMatch,
}

/*fn connection_handler<F, S>(server: &Server<S>, handle: &Handle) -> F
    where F: Future{
    
}*/



fn main() {
    let mut core = Core::new().expect("could not init tokio core");
    let handle = core.handle();

    //let mut pso3p_serv = Rc::new(RefCell::new(PSO3PGameServer::new(handle.clone())));
    let server = Server::bind(format!("127.0.0.1:{}", PORT), &handle).expect("could not bind server");

    let (pending_clients_tx, pending_clients_rx) = mpsc::unbounded();

    let all_clients = Rc::new(RefCell::new(HashMap::new()));
    
    let handle_inner = handle.clone();
    let all_clients_inner = all_clients.clone();
    let incoming_connections = server.incoming()
        .map_err(|InvalidConnection {error, ..}| error)
        .for_each(move |(conn, addr)| {
            println!("connection from {}", addr);

            let all_clients_inner2 = all_clients_inner.clone();
            let pending_clients_tx_inner = pending_clients_tx.clone();
            let handle_inner2 = handle_inner.clone();
            let accept = conn.accept()
                .and_then(move |(sock, _header)| {
                    let (send_tx, send_rx) = mpsc::unbounded();
                    let (recv_tx, recv_rx) = mpsc::unbounded();
                    let (stx, srx) = sock.split();
                    
                    // send from server to client: crx -> stx
                    let csend = send_rx
                        .fold(stx, |mut stx, msg| {
                            //println!("csend! {:?}", msg);
                            //stx.start_send(msg).unwrap();
                            //stx = stx.send(msg).wait().unwrap();
                            stx = stx.send(OwnedMessage::Text(serde_json::to_string(&msg).unwrap())).wait().unwrap();
                            //stx.send_message(msg).unwrap();
                            Ok(stx)
                        })
                        .map(|_| ())
                        .map_err(|_| ());
                    handle_inner2.spawn(csend);

                    // recv'd from client to server
                    // let handle_inner3 = handle_inner2.clone();
                    let crecv = srx
                        .for_each(move |msg| {
                            //println!("crecv! {:?}", msg);
                            let mut recv_tx_inner = recv_tx.clone();
                            recv_tx_inner.start_send(msg).unwrap();
                            Ok(())
                        })
                        .then(|_| Ok(()));
                    handle_inner2.spawn(crecv);

                    send_tx.unbounded_send(StateChange::DebugMsg("waiting for opponent...".to_string())).unwrap();
                    let id = rand::thread_rng().gen();
                    let client = Client::Pending{id,
                                                 tx: send_tx,
                                                 rx: recv_rx,
                                                 addr};
                    //let client = Client::new(send_tx, recv_rx, addr);
                    
                    //let client = Rc::new(RefCell::new(Client::new(sock, addr)));
                    //let id = client.borrow().id;

                    all_clients_inner2.borrow_mut().insert(id, client);
                    //let f = pending_clients_tx_inner.send(Client::new(sock, addr)).then(|_| Ok(()));
                    let f = pending_clients_tx_inner.send(id).then(|_| Ok(()));
                    handle_inner2.spawn(f);
                    Ok(())
                })
                .map_err(|_| ());

            handle_inner.spawn(accept);

            Ok(())
        });
    
    let handle_inner = handle.clone();
    let all_clients_inner = all_clients.clone();
    let pending_clients = Rc::new(RefCell::new(VecDeque::new()));
    let match_pending_clients = pending_clients_rx.for_each(move |client_id| {
        let mut pending_clients = pending_clients.borrow_mut();
        pending_clients.push_back(client_id);
        let all_clients_inner2 = all_clients_inner.clone();
        if pending_clients.len() >= 2 {
            let c1 = pending_clients.pop_front().unwrap();
            let c2 = pending_clients.pop_front().unwrap();


            let (c1_match_tx, c1_match_rx) = mpsc::unbounded();
            let (c2_match_tx, c2_match_rx) = mpsc::unbounded();
            
            let (match_c1_tx, match_c1_rx) = mpsc::unbounded();
            let (match_c2_tx, match_c2_rx) = mpsc::unbounded();

            let handle_inner2 = handle_inner.clone();
            for c in vec![(c1, c1_match_tx, match_c1_rx), (c2, c2_match_tx, match_c2_rx)] {
                let (client_id, tx, rx) = c;
                let mut all_clients = all_clients_inner2.borrow_mut();
                let client = all_clients.remove(&client_id).unwrap();

                if let Client::Pending{id, tx: client_tx, rx: client_rx, ..} = client {
                    
                    //client_tx.unbounded_send(OwnedMessage::Text("\"found opponent...\"".to_string())).unwrap();
                    client_tx.unbounded_send(StateChange::DebugMsg("found opponent".to_string())).unwrap();
                    let handle_inner3 = handle_inner2.clone();
                    let client_handler = client_rx
                        .for_each(move |msg| {
                            println!("[{}] msg: {:?}", id, msg);
                            
                            let tx = tx.clone();
                            //let f = tx.send(OwnedMessage::Text(format!("[{}] {:?}", id, msg))).then(|_| Ok(()));
                            let f = tx.send(msg).then(|_| Ok(()));
                            handle_inner3.spawn(f);
                            
                            Ok(())
                        })
                        .map_err(|_| ());

                    handle_inner2.spawn(client_handler);
                    
                    let client_sender = rx
                        .fold(client_tx, move |client_tx, msg| {
                            //let t = OwnedMessage::Text(format!("[{}] {:?}", id, msg));
                            //client_tx.unbounded_send(t).unwrap();
                            client_tx.unbounded_send(msg).unwrap();
                            Ok(client_tx)
                        })
                        .then(|_| {
                            println!("thened the fuck out");
                            Ok(())
                        });
                    
                    handle_inner2.spawn(client_sender);
                }
            }

            let match_ = Match::new();

            /*match_c1_tx.unbounded_send(OwnedMessage::Text(serde_json::to_string(&StateChange::SetPlayer {
                player: PlayerId::One,
            }).unwrap())).unwrap();
            match_c2_tx.unbounded_send(OwnedMessage::Text(serde_json::to_string(&StateChange::SetPlayer {
                player: PlayerId::Two,
        }).unwrap())).unwrap();*/

            match_c1_tx.unbounded_send(StateChange::SetPlayer {
                player: PlayerId::One,
            }).unwrap();
            match_c2_tx.unbounded_send(StateChange::SetPlayer {
                player: PlayerId::Two,
            }).unwrap();
            
            let mut actions = Vec::new();
            actions.push(StateChange::SetCharacter {
                card: match_.sim.state.boardstate.player1.character
            });
            actions.push(StateChange::SetCharacter {
                card: match_.sim.state.boardstate.player2.character
            });

            for a in actions {
                match_c1_tx.unbounded_send(a.clone()).unwrap();
                match_c2_tx.unbounded_send(a).unwrap();
                //match_c1_tx.unbounded_send(OwnedMessage::Text(serde_json::to_string(&a).unwrap())).unwrap();
                //match_c2_tx.unbounded_send(OwnedMessage::Text(serde_json::to_string(&a).unwrap())).unwrap();
            }

            

            // send clients all the pregame stuff
            //match_c1_tx.unbounded_send(OwnedMessage::Text()).unwrap();
            
            //let match_handler = c1_match_rx.select(c2_match_rx)
            let match_handler = c1_match_rx.merge(c2_match_rx)
                .for_each(move |item| {
                    /*match item {
                        MergedItem::First(m) => {
                            if let OwnedMessage::Close(_) = m {
                                //match_c2_tx.unbounded_send(OwnedMessage::Text("opponent left".to_string())).unwrap();
                                return Err(());
                            }
                            match_c2_tx.unbounded_send(m).unwrap();
                        },
                        MergedItem::Second(m) => {
                            if let OwnedMessage::Close(_) = m {
                                //match_c1_tx.unbounded_send(OwnedMessage::Text("opponent left".to_string())).unwrap();
                                return Err(());
                            }
                            match_c1_tx.unbounded_send(m).unwrap();
                        },
                        _ => panic!("not sure if this will ever be hit")
                    }*/
             
                    Ok(())
                })
                .then(|_| {
                    println!("rip match");
                    Ok(())
                });
                        
            handle_inner.spawn(match_handler);
        }

        Ok(())
    });

    handle.spawn(match_pending_clients);

    core.run(incoming_connections).expect("could not start main loop");

    
}







