mod test;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let server_conf = args.get(1);
    let server_conf = match server_conf {
        None => "127.0.0.1:8080",
        Some(conf) => conf,
    };

    run(server_conf).await;
}

async fn run(config: &str) {
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
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    // close condition
                    println!("User {} has disconected", _addr);
                    break;
                }

                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
