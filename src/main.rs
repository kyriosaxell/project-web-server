use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use test_voice::ThreadPool;

fn simple_handle_connection(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("src/hello.xhtml").unwrap();
    let contents_length = contents.len();
    let response = format!(
        "{status_line}\r\n\
        Content-Length: {contents_length}\r\n\r\n\
        {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // Ejemplo
    /*    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "src/hello.xhtml")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "src/404.xhtml")
        };
    */
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.xhtml"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(4));
            ("HTTP/1.1 200 OK", "src/hello.xhtml")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.xhtml"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\n\
             Content-Length: {length}\r\n\r\n\
             {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let data = stream.unwrap();
        pool.execute(|| {
            handle_connection(data);
        });
    }
}
