#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
//extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
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




fn main2() {
    let mut core = Core::new().expect("could not init tokio core");
    let handle = core.handle();

    //let mut pso3p_serv = Rc::new(RefCell::new(PSO3PGameServer::new(handle.clone())));
    let server = Server::bind(format!("127.0.0.1:{}", PORT), &handle).expect("could not bind server");

    //let (pending_clients_tx, pending_clients_rx) = mpsc::unbounded();

    let clients = Rc::new(RefCell::new(HashMap::new()));
    let clients_tx = Rc::new(RefCell::new(HashMap::new()));

    let handle2 = handle.clone();
    let clients2 = clients.clone();
    let clients_tx2 = clients_tx.clone();
    let incoming_connections = server.incoming()
        .map_err(|InvalidConnection {error, ..}| error)
        .for_each(move |(conn, addr)| {
            println!("connection from {}", addr);

            let handle3 = handle2.clone();
            let clients3 = clients2.clone();
            let clients_tx3 = clients_tx2.clone();
            let accept = conn.accept()
                .and_then(move |(sock, _header)| {
                    let id: ClientId = rand::thread_rng().gen();

                    let mut clients = clients3.borrow_mut();
                    clients.insert(id, 5);

                    let (tx, rx) = sock.split();

                    let mut clients_tx = clients_tx3.borrow_mut();
                    clients_tx.insert(id, tx);
                    
                    Ok(())
                })
                .map_err(|_| ());

            handle3.spawn(accept);

            
            Ok(())
        })
        .map_err(|_| ());
    


    core.run(incoming_connections).unwrap();

    
}







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
                    //let f = pending_clients_tx_inner.send(Client::new(sock, addr)).then(|_| Ok(()));

                    // map client actual sockets to unbound channels so we can clone this struct around
                    let (send_tx, send_rx) = mpsc::unbounded();
                    let (recv_tx, recv_rx) = mpsc::unbounded();
                    let (stx, srx) = sock.split();
                    
                    // send from server to client: crx -> stx
                    let csend = send_rx
                        .fold(stx, |mut stx, msg| {
                            println!("csend! {:?}", msg);
                            //stx.start_send(msg).unwrap();
                            stx = stx.send(msg).wait().unwrap();
                            //stx.send_message(msg).unwrap();
                            Ok(stx)
                        })
                        .map(|_| ())
                        .map_err(|_| ());
                    //.then(|_| Ok(()));
                    //let csend = send_rx
                        //.forward(stx)
                    //.then(|_| ());
                    //.map_err(|_| Ok(()));

                    //let newstx = Rc::new(RefCell::new(stx));
                    //let stx = newstx.clone();
                    /*let csend = send_rx
                        .for_each(move |msg | {
                            //let stx_inner = stx.clone();
                            //let stx = stx.into_inner();
                            //stx = stx.send(msg).wait().unwrap();
                            //newstx = f;
                            
                            Ok(())
                        });*/
                        //.then(|_| ());
                        //.map_err(|_| Ok(()));
                    handle_inner2.spawn(csend);

                    // recv'd from client to server
                    // let handle_inner3 = handle_inner2.clone();
                    let crecv = srx
                        /*.fold(recv_tx, |mut recv_tx, msg| {
                            recv_tx.start_send(msg).unwrap();
                            Ok(recv_tx)
                })*/
                        .for_each(move |msg| {
                            println!("crecv! {:?}", msg);
                            let mut recv_tx_inner = recv_tx.clone();
                            recv_tx_inner.start_send(msg).unwrap();
                            //let f = recv_tx.send(msg).wait().unwrap();
                            //handle_inner3.spawn(f);

                            //recv_tx = recv_tx.send(msg).wait().unwrap();
                            
                            Ok(())
                        })
                        .then(|_| Ok(()));
                    handle_inner2.spawn(crecv);

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

            let match_ = Match::new();

            let (c1_match_tx, c1_match_rx) = mpsc::unbounded();
            let (c2_match_tx, c2_match_rx) = mpsc::unbounded();
            
            let (match_c1_tx, match_c1_rx) = mpsc::unbounded();
            let (match_c2_tx, match_c2_rx) = mpsc::unbounded();

            let handle_inner2 = handle_inner.clone();
            for c in vec![(c1, c1_match_tx, match_c1_rx), (c2, c2_match_tx, match_c2_rx)] {
                let (client_id, tx, rx) = c;
                //let id = client.id;
                let mut all_clients = all_clients_inner2.borrow_mut();
                let client = all_clients.remove(&client_id).unwrap();

                if let Client::Pending{id, tx: client_tx, rx: client_rx, ..} = client {
                    
                    //let (client_tx, client_rx) = client.sock.split();
                    let handle_inner3 = handle_inner2.clone();
                    let client_handler = client_rx
                        .for_each(move |msg| {
                            println!("[{}] msg: {:?}", id, msg);
                            
                            let tx = tx.clone();
                            let f = tx.send(OwnedMessage::Text(format!("[{}] {:?}", id, msg))).then(|_| Ok(()));
                            handle_inner3.spawn(f);
                            
                            Ok(())
                        })
                        .map_err(|_| ());

                    handle_inner2.spawn(client_handler);
                    
                    /*let client_sender = rx
                    .map(move |msg| {
                    OwnedMessage::Text(format!("[{}] {:?}", id, msg))
                })
                    .map_err(|_| WebSocketError::NoDataAvailable)
                    .forward(client_tx)
                    .then(|_| {
                    println!("thened the fuck out");
                    Ok(())
                });*/
                    //let handle_inner3 = handle_inner2.clone();
                    let client_sender = rx
                        .fold(client_tx, move |client_tx, msg| {
                            let t = OwnedMessage::Text(format!("[{}] {:?}", id, msg));
                            client_tx.unbounded_send(t).unwrap();
                            Ok(client_tx)
                        })
                        /*.for_each(move |msg| {
                            let t = OwnedMessage::Text(format!("[{}] {:?}", id, msg));
                            client_tx.unbounded_send(t).unwrap();
                            //let client_tx = client_tx;
                            //let f = client_tx.unbounded_send(t)
                                //.map_err(|_| ());
                                //.then(|_| Ok(()));
                            //let f = client.tx.start_send(t).unwrap();
                            //handle_inner3.spawn(f);
                                

                            Ok(())
                        })*/
                        .then(|_| {
                            println!("thened the fuck out");
                            Ok(())
                        });
                    
                    handle_inner2.spawn(client_sender);
                }
            }
            
            //let match_handler = c1_match_rx.into_future().select(c2_match_rx.into_future()).into_stream()
            //let match_handler = c1_match_rx.merge(c2_match_rx)
            let match_handler = c1_match_rx.merge(c2_match_rx)
                .for_each(move |item| {
                    match item {
                        MergedItem::First(m) => match_c2_tx.unbounded_send(m).unwrap(),
                        MergedItem::Second(m) => match_c1_tx.unbounded_send(m).unwrap(),
                        _ => panic!("not sure if this will ever be hit")
                    }
                    
                    //let match_c1_tx = match_c1_tx.clone();
                    /*let f = match_c1_tx.send("asdf")
                        .and_then(|_tx| {
                            Ok(())
                        })
                        .map_err(|_| ());
                    handle_inner2.spawn(f);

                    let match_c2_tx = match_c2_tx.clone();
                    let f = match_c2_tx.send("123")
                        .and_then(|_tx| {
                            Ok(())
                        })
                        .map_err(|_| ());
                    handle_inner2.spawn(f);*/
                    //match_c1_tx.unbounded_send("asdf").unwrap();
                    //match_c2_tx.unbounded_send("123").unwrap();

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







