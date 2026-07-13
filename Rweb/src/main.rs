use Rweb::ThreadPool;
use futures::Stream;
use futures::stream::{self, StreamExt};
use std::{fs, io::prelude::*};
use std::{net, thread};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream}; // ← 添加这两行
fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8888").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection2(stream);
        });
    }
}

// #[tokio::main(worker_threads = 4)]
// async fn main() {
//     // println!("当前工作目录: {:?}", std::env::current_dir().unwrap());
//     // let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

//     // listener
//     //     .incoming() // ← 现在这是一个 Stream
//     //     .for_each_concurrent(None, |stream| async move {
//     //         let stream = stream.unwrap();
//     //         handle_connection(stream).await;
//     //     })
//     //     .await;

//     let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

//     stream::unfold(listener, |listener| async {
//         Some((listener.accept().await, listener))
//     })
//     .for_each_concurrent(None, |result| async move {
//         let (stream, _) = result.unwrap();
//         tokio::spawn(async move {
//             handle_connection(stream).await;
//         });
//     })
//     .await;
// }
//
fn handle_connection2(mut stream: std::net::TcpStream) {
    use std::time::Duration;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // tokio::time::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

async fn handle_connection(mut stream: impl AsyncReadExt + AsyncWriteExt + Unpin) {
    use std::time::Duration;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        tokio::time::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

use futures::io::Error;
use futures::task::{Context, Poll};

use std::cmp::min;
use std::pin::Pin;

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl tokio::io::AsyncRead for MockTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<Result<(), Error>> {
        let this = self.get_mut();
        let size = min(buf.remaining(), this.read_data.len());
        buf.put_slice(&this.read_data[..size]);
        this.read_data.drain(..size);
        Poll::Ready(Ok(()))
    }
}

impl tokio::io::AsyncWrite for MockTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context,
        buf: &[u8],
    ) -> Poll<tokio::io::Result<usize>> {
        self.write_data = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}

use std::marker::Unpin;
impl Unpin for MockTcpStream {}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    // let mut buf = [0u8; 1024];
    // stream.read(&mut buf).await.unwrap();
    handle_connection(&mut stream).await;

    let expected_contents = fs::read_to_string("hello.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    println!("yes");
}
