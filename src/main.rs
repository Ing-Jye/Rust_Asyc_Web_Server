use std::{
    net::{TcpListener, TcpStream}, 
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
};
use std::fs;
use tokio;


fn main() {
    println!("Web Server Start!");
    
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let _guard = rt.enter();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &rt);
    }

}

fn handle_connection(mut stream: TcpStream, rt: &tokio::runtime::Runtime) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    rt.spawn(async move {

        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
 
        let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
        
    });

}
