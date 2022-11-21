#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

pub fn run(ip: IpAddr, port: u16) {
    let mutex_names: Arc<Mutex<HashMap<Vec<u8>, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    let mutex_adresses: Arc<Mutex<HashMap<SocketAddr, Vec<u8>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let address = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mutex_names_copy = mutex_names.clone();
                let mutex_adress_copy = mutex_adresses.clone();

                let _thread = thread::spawn(move || {
                    let mut buf: Vec<u8> = vec![];
                    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());

                    let name = BufReader::read_until(&mut buf_reader, 1u8, &mut buf);

                    let mut guard_name = mutex_names_copy.lock().unwrap();
                    let mut guard_address = mutex_adress_copy.lock().unwrap();

                    match name {
                        Ok(res) => {
                            if res == 0 {
                                return;
                            }

                            guard_name.insert(buf.clone(), stream.try_clone().unwrap());
                            guard_address.insert(stream.peer_addr().unwrap(), buf.clone());
                        }
                        Err(_) => {
                            return;
                        }
                    }

                    std::mem::drop(guard_name);
                    std::mem::drop(guard_address);

                    loop {
                        buf.clear();
                        let to = BufReader::read_until(&mut buf_reader, 1u8, &mut buf);

                        let mut adress_to: TcpStream;
                        let mut reply: Vec<u8> = vec![];

                        match to {
                            Ok(res) => {
                                if res == 0 {
                                    let mut guard_address = mutex_adress_copy.lock().unwrap();
                                    let cur_name =
                                        guard_address.remove(&stream.peer_addr().unwrap()).unwrap();

                                    let mut guard_name = mutex_names_copy.lock().unwrap();
                                    guard_name.remove(&cur_name);

                                    break;
                                }

                                let guard_address = mutex_adress_copy.lock().unwrap();
                                let mut cur_name: Vec<u8> =
                                    (*guard_address.get(&stream.peer_addr().unwrap()).unwrap())
                                        .clone();

                                if buf == cur_name {
                                    continue;
                                }

                                let guard_name = mutex_names_copy.lock().unwrap();
                                let tmp_adress_to = guard_name.get(&buf);

                                if tmp_adress_to.is_none() {
                                    continue;
                                }

                                adress_to = tmp_adress_to.unwrap().try_clone().unwrap();

                                reply.append(&mut cur_name);
                            }
                            Err(_) => {
                                let mut guard_address = mutex_adress_copy.lock().unwrap();
                                let cur_name =
                                    guard_address.remove(&stream.peer_addr().unwrap()).unwrap();

                                let mut guard_name = mutex_names_copy.lock().unwrap();
                                guard_name.remove(&cur_name);
                                break;
                            }
                        }

                        buf.clear();
                        let message = BufReader::read_until(&mut buf_reader, 1u8, &mut buf);

                        match message {
                            Ok(res) => {
                                if res == 0 {
                                    let mut guard_address = mutex_adress_copy.lock().unwrap();
                                    let cur_name =
                                        guard_address.remove(&stream.peer_addr().unwrap()).unwrap();

                                    let mut guard_name = mutex_names_copy.lock().unwrap();
                                    guard_name.remove(&cur_name);

                                    break;
                                }

                                if *buf.last().unwrap() != 1u8 {
                                    let mut guard_address = mutex_adress_copy.lock().unwrap();
                                    let cur_name =
                                        guard_address.remove(&stream.peer_addr().unwrap()).unwrap();

                                    let mut guard_name = mutex_names_copy.lock().unwrap();
                                    guard_name.remove(&cur_name);

                                    break;
                                }

                                reply.append(&mut buf);
                            }
                            Err(_) => {
                                let mut guard_address = mutex_adress_copy.lock().unwrap();
                                let cur_name =
                                    guard_address.remove(&stream.peer_addr().unwrap()).unwrap();

                                let mut guard_name = mutex_names_copy.lock().unwrap();
                                guard_name.remove(&cur_name);
                                break;
                            }
                        }

                        let res = adress_to.write(&reply);

                        if res.is_err() {
                            println!("Error!");
                        }
                    }
                });
            }
            Err(_) => {
                println!("An error occured!");
            }
        }
    }
}
