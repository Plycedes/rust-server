use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use rust_server::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listner.incoming(){
        let stream = stream.unwrap();

        pool.execute(|| {
            connection_handler(stream);
        });        
    }
}

fn connection_handler(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();   

    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    // ex: HTTP/1.1 200 OK\r\n\r\n

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = 
    if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "./html/index.html")
    } 
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./html/index.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "./html/404.html")
    };
    let content = fs::read_to_string(filename).unwrap();    
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",status, content.len(), content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    
}