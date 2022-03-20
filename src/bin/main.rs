use std::thread;
use std::time::Duration;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use api::ThreadPool;
// use warp::body::content_length_limit;
// use warp::http::response;
extern crate mysql;
extern crate r2d2_mysql;
extern crate r2d2;


// ctrl+cでサーバの立ち上げをキャンセル
fn main() {
  let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

  // for stream in listener.incoming().take(2) {
  //   let stream = stream.unwrap();

    pool.execute( || {
      handle_connection(stream)
    });

  }

  println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

    
  let (_status_line, filename) =
    if buffer.starts_with(get) {
      ("HTTP/1.1 200 OK", "index.html")

    }else if buffer.starts_with(sleep){
      thread::sleep(Duration::from_secs(4));
      ("HTTP/1.1 200 OK", "index.html")
    }
    else{
      ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!(
      "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
      contents.len(),
      contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

  let status_line = "HTTP/1.1 404 NOT FOUND";
  let contents = fs::read_to_string(filename).unwrap();

  let response = format!(
     "{}\r\nContent-Length: {}\r\n\r\n{}",
       status_line,
      contents.len(),
      contents
     );
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
  

}







