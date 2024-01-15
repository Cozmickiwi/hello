use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use local_ip_address::local_ip;

use hello::ThreadPool;

fn main() {
    let add = local_ip().unwrap().to_string();
    let add_str = format!("{add}:7878");
    println!("{add_str}");
    let listener = TcpListener::bind(&add_str).unwrap();
    let pool = ThreadPool::new(10);
    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        println!("Incoming connection from: {}", peer_addr);
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename, content_type) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "frontend/index.html", "text/html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "frontend/index.html", "text.html")
        }
        "GET /styles.css HTTP/1.1" => ("HTTP/1.1 200 OK", "frontend/styles.css", "text/css"),
        "GET /script.js HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            "frontend/script.js",
            "application/javascript",
        ),
        _ => ("HTTP/1.1 404 NOT FOUND", "frontend/404.html", "text/html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    //let length = contents.len();
    let response = format!(
        "{status_line}\r\n\
        Content-Type: {content_type}\r\n\r\n
        {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
