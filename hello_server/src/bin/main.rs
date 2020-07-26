use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use hello_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2){
        let stream = stream.unwrap();
        thread_pool.execute(||{
            handle_stream(stream);
        });
    }

    fn handle_stream(mut stream: TcpStream){
        println!("Connection estblished");
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (html_file, response) = if buffer.starts_with(get){
            ("hello.html", "HTTP/1.1 OK\r\n\r\n")
        }else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("hello.html", "HTTP/1.1 OK \r\n\r\n")
        }else {
            ("404.html", "HTTP/1.1 NOT FOUND\r\n\r\n")
        };
        let html = fs::read_to_string(html_file).unwrap();
        let response = format!("{}{}", response, html);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
