use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line.is_empty() {
        return;
    }

    let (status_line, filename) = handle_response(&request_line);
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}", status_line = status_line, length = contents.len(), contents = contents);
    
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_response(request_line: &str) -> (&str, &str) {
    return match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "pages/hello.html"), 
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10)); ("HTTP/1.1 200 OK", "pages/hello.html") 
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "pages/404.html"),
    };
}