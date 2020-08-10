extern crate rand;
use rand::Rng;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Cursor,Read, Write};


fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];        
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut rng = rand::thread_rng();
            let mut buf = [0u8; 65536];

            //Jeremy's quotes 
            let quotes = ["Just imagine, me in the pub all day, but no one can say a thing because it’s my job and I’ve got to be there. I’ll literally get paid to go to the pub", "Stealing things just makes everything very cheap. Plus, you know how I feel about capitalism", "Are You Happy Now, Bush?", "I Don’t Want To Tempt Fate, But I Think Everything Is Going To Be Totally Great Forever.",
            "I’m A Very Strong Feminist, So I Believe Women Should Have Whatever Mad Thing They Want.", "Love Is All You Need? No, Actually, Beatles, You Also Need A Person To Do It With, Beatles!"];
            let mut cursor = Cursor::new(&mut buf[..]);
            //pick random quote
            let random_phrase = rng.gen_range(0, quotes.len());
            let s = quotes[random_phrase];
            
            write!(cursor, "{}'\n", s).unwrap();
            let len = cursor.position() as usize;
            stream.write(&buf[..len]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}