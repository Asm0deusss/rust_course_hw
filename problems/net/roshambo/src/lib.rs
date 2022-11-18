#![forbid(unsafe_code)]

use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

use std::collections::HashMap;

pub fn run(ip: IpAddr, port: u16) {
    let address = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(address).unwrap();

    let mut clients: HashMap<usize, TcpStream> = HashMap::new();
    let mut cur_client_counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                cur_client_counter += 1;
                clients.insert(cur_client_counter, stream);
            }
            Err(_e) => {
                println!("Error");
            }
        }

        if cur_client_counter == 2 {
            break;
        }
    }

    let mut first_player = &clients[&1];
    let mut second_player = &clients[&2];

    first_player
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .unwrap();
    second_player
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .unwrap();

    loop {
        let mut is_bad = false;
        let mut first_move = 0_u8;
        let mut second_move = 0_u8;

        let mut buf_1 = [0_u8];
        let result = first_player.read(&mut buf_1);

        match result {
            Ok(_) => {
                let ans = buf_1[0];
                match ans {
                    b'R' => {
                        first_move = b'R';
                    }
                    b'P' => {
                        first_move = b'P';
                    }
                    b'S' => {
                        first_move = b'S';
                    }
                    _ => {
                        is_bad = true;
                    }
                }
            }
            Err(_) => {
                is_bad = true;
            }
        }

        let mut buf_2 = [0_u8];
        let result = second_player.read(&mut buf_2);

        match result {
            Ok(_) => {
                let ans = buf_2[0];
                match ans as u8 {
                    b'R' => {
                        second_move = b'R';
                    }
                    b'P' => {
                        second_move = b'P';
                    }
                    b'S' => {
                        second_move = b'S';
                    }
                    _ => {
                        is_bad = true;
                    }
                }
            }
            Err(_) => {
                is_bad = true;
            }
        }

        if is_bad {
            println!("Something bad happend");
            return;
        }

        let mut write_first: [u8; 2] = [second_move, b'W'];
        let mut write_second: [u8; 2] = [first_move, b'L'];

        if first_move == second_move {
            write_first[1] = b'D';
            write_second[1] = b'D';
        }

        if first_move == b'R' && second_move == b'P' {
            write_first[1] = b'L';
            write_second[1] = b'W';
        }

        if first_move == b'P' && second_move == b'S' {
            write_first[1] = b'L';
            write_second[1] = b'W';
        }

        if first_move == b'S' && second_move == b'R' {
            write_first[1] = b'L';
            write_second[1] = b'W';
        }

        let try_write_first = first_player.write(&write_first);

        if try_write_first.is_err() {
            break;
        }

        let try_write_second = second_player.write(&write_second);

        if try_write_second.is_err() {
            break;
        }
    }
}
