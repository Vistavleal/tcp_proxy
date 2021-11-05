mod test;

use tokio::{
    // io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let args = match std::env::args().nth(1) {
        None => "127.0.0.1:8080".to_string(),
        Some(adrr) => adrr,
    };

    run(args).await;
}

async fn run(config: String) {
    let listener = TcpListener::bind(config).await.unwrap();
    println!("Server has started");

    loop {
        // Allow multiple connections
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("User {} has connected!", _addr);
            let (mut reader, mut writer) = socket.split();


            loop {

                if tokio::io::copy(&mut reader, &mut writer).await.is_err() {
                    eprintln!("Failed to copy data from reader to writer");
                }
            }
        });
    }
}
