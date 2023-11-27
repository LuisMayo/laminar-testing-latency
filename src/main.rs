use std::{thread, time::{Instant, Duration}, env::args_os};

use laminar::{Socket, Packet};

fn serve() {
    let mut socket = Socket::bind(format!("127.0.0.1:9999")).unwrap();
    let rec = socket.get_event_receiver();
    thread::spawn(move || socket.start_polling());
    loop {
        let instant = Instant::now();
        let _msg = rec.recv();
        println!("{:?}", instant.elapsed().as_millis());
    }
}

fn client() {
    let mut socket = Socket::bind(format!("127.0.0.1:9998")).unwrap();
    let send = socket.get_packet_sender();
    thread::spawn(move || socket.start_polling());
    let mut instant = Instant::now();
    loop {
        send.send(Packet::unreliable_sequenced("127.0.0.1:9999".parse().unwrap(), vec![], None));
        println!("{:?}", instant.elapsed().as_millis());
        instant = Instant::now();
        thread::sleep(Duration::from_millis(16));
    }
}

fn main() {
    if args_os().any(|element| element == "--client") {
        println!("Client!");
        client();
    } else {
        println!("Server");
        serve();
    }
}
