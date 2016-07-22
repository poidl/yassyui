use libc;
use lv2;
use std::ptr;
use std::thread;
use websocket::Server;
// use std::net::TcpListener;
use std::sync::mpsc;
use websocket::{Message, Sender, Receiver};

// use websocket::{Server, Message, Sender, Receiver};
// use websocket::server::Connection;
use websocket::server::request::Request;
use websocket::server::response::Response;
use websocket::message::Type;
use websocket::header::WebSocketProtocol;
use websocket::stream::WebSocketStream;
use std::net::TcpStream;
use std::io;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
// use std::sync::mpsc;
use std::net::TcpListener;

#[repr(C)]
pub struct yassyui {
    pub host: *const lv2::LV2UIExternalUIHost,
    pub controller: lv2::LV2UIController,
    pub write: lv2::LV2UIWriteFunction,
    pub extwidget: lv2::LV2UIExternalUIWidget,
    pub showing: bool,
    // pub tcplistener: TcpListener,
    pub sender: mpsc::Sender<f32>,
    pub receiver: mpsc::Receiver<f32>,
}

impl yassyui {
    pub fn new() -> yassyui {
        // println!("address: {}", ipaddr);
        let (tx, rx) = mpsc::channel();
        let ui = yassyui {
            extwidget: lv2::LV2UIExternalUIWidget {
                // Why "None"? Nullable function pointers. See
                // https://doc.rust-lang.org/book/ffi.html
                // https://mail.mozilla.org/pipermail/rust-dev/2014-September/011200.html
                run: None,
                show: None,
                hide: None,
            },
            host: ptr::null(),
            controller: ptr::null(),
            write: None,
            showing: false, /* tcplistener: TcpListener::bind("127.0.0.1:2794").unwrap()
                             * sender: tx, */
            sender: tx,
            receiver: rx,
        };
        ui
    }
    // pub fn receive(&self, val: f32) {
    //     println!("Hello", );
    // }
    pub fn connect(&mut self, write_function: lv2::LV2UIWriteFunction, controller: lv2::LV2UIController) {
        let tcplistener = TcpListener::bind("127.0.0.1:2794").unwrap();
        let result = tcplistener.accept();
        match result {
            Ok(s) => {
                let tcpstream = s.0;
                let wsstream = WebSocketStream::Tcp(tcpstream);
                pub struct Connection<R: Read, W: Write>(R, W);
                let connection = Connection(wsstream.try_clone().unwrap(),
                                            wsstream.try_clone().unwrap());

                let (tx, rx_and_send_to_br) = mpsc::channel();
                let tx_1 = tx.clone();
                let (tx_2, rx_and_send_to_send) = mpsc::channel();
                self.sender = tx_2;
                thread::spawn(move || {
                    loop {
                        let val: f32 = match rx_and_send_to_send.recv() {
                            Ok(v) => v,
                            Err(e) => {
                                println!("Oeha: {:?}", e);
                                return;
                            }
                        };
                        println!("val: {}", val);
                        let message: Message = Message::text(val.to_string());
                        tx_1.send(message);
                    }
                });
                unsafe {
                    // unsafe following line works around calling on_ws_receive()
                    // with raw pointer (raw opinters are not "send")
                    // TODO: dangerous?
                    let ctrl = &*(controller as *const i64);
                    thread::spawn(move || {

                        let request = Request::read(connection.0, connection.1).unwrap();
                        let headers = request.headers.clone(); // Keep the headers so we can check them

                        request.validate().unwrap(); // Validate the request

                        let mut response = request.accept(); // Form a response

                        if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                            if protocols.contains(&("rust-websocket".to_string())) {
                                // We have a protocol we want to use
                                response.headers
                                    .set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
                            }
                        }

                        let mut client = response.send().unwrap(); // Send the response

                        let ip = client.get_mut_sender()
                            .get_mut()
                            .peer_addr()
                            .unwrap();

                        println!("Connection from {}", ip);

                        let message: Message = Message::text("Hello".to_string());
                        client.send_message(&message).unwrap();

                        let (mut sender, mut receiver) = client.split();
                        // send to browser
                        let send_loop = thread::spawn(move || {
                            loop {
                                // Send loop
                                println!("send loop");
                                let message: Message = match rx_and_send_to_br.recv() {
                                    Ok(m) => m,
                                    Err(e) => {
                                        println!("Send Loop: {:?}", e);
                                        return;
                                    }
                                };
                                match message.opcode {
                                    Type::Close => {
                                        let _ = sender.send_message(&message);
                                        // If it's a close message, just send it and then return.
                                        return;
                                    }
                                    _ => (),
                                }
                                // Send the message
                                match sender.send_message(&message) {
                                    Ok(()) => (),
                                    Err(e) => {
                                        println!("Send Loop: {:?}", e);
                                        let _ = sender.send_message(&Message::close());
                                        return;
                                    }
                                }
                            }
                        });

                        // receive from browser
                        for message in receiver.incoming_messages() {
                            let message: Message = message.unwrap();

                            match message.opcode {
                                Type::Close => {
                                    // Got a close message, so send a close message and return
                                    let _ = tx.send(Message::close());
                                    return;
                                }
                                Type::Ping => {
                                    match tx.send(Message::pong(message.payload)) {
                                        // Send a pong in response
                                        Ok(()) => (),
                                        Err(e) => {
                                            println!("Receive Loop: {:?}", e);
                                            return;
                                        }
                                    }
                                }
                                // Say what we received
                                _ => {
                                    let vecu8 = message.payload.into_owned();
                                    let mess = String::from_utf8(vecu8).unwrap();
                                    let myfloat = mess.parse::<f32>();
                                    println!("Receive Loop: {:?}", myfloat);
                                    match myfloat {
                                        Ok(f) => {
                                            on_ws_receive(write_function, ctrl, &f);
                                        }
                                        Err(err) => println!("Err: {}", err),
                                    }


                                }
                            }
                        }
                    });
                }//unsafe
            }
            _ => println!("error"),
        };
    }
}

fn on_ws_receive(write: lv2::LV2UIWriteFunction, controller: &i64, f: &f32) {

    let ctrl = controller as *const i64 as lv2::LV2UIController;
    if let Some(ref func) = write {
        (*func)(ctrl, 2, 4, 0, f as *const f32 as *const libc::c_void);
        println!("bla");
    }
}
