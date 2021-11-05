mod test;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
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
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            // TODO: add logs
            let mut buffer = [0; 1024];

            loop {
                let bytes_read = match reader.read(&mut buffer).await {
                    Ok(n) if n == 0 => {
                        println!("User {} has disconected", _addr);
                        return;
                    }
                    Ok(n) => n,
                    Err(err) => {
                        eprintln!("Failed to read: {}", err);
                        return;
                    }
                };

                if bytes_read == 0 {
                    // close condition
                    println!("User {} has disconected", _addr);
                    break;
                }

                if let Err(e) = writer.write_all(&buffer[..bytes_read]).await {
                    eprintln!("Failed to write: {}", e);
                    return;
                }
            }
        });
    }
}
