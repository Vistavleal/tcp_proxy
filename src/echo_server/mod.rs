pub async fn start_server(config: String) {
    let listener = tokio::net::TcpListener::bind(config).await.unwrap();

    loop {
        // Allow multiple connections
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();

            loop {
                let bytes = tokio::io::copy(&mut reader, &mut writer).await.unwrap();
                if bytes == 0 {
                    break;
                }
            }
        });
    }
}
