use tracing::{info, instrument, warn, Instrument};

#[instrument]
pub async fn start_proxy(proxy_addr: String, serv_addr: std::net::SocketAddr) {
    let listener = tokio::net::TcpListener::bind(proxy_addr)
        .await
        .expect("Failed to create TcpListener");

    loop {
        // Accept user's connection
        let (mut socket, user_addr) = match listener.accept().await {
            Ok(socket) => {
                info!(target: "Connection", "User {} has connected", socket.1);
                socket
            }
            Err(e) => {
                warn!(target: "Connection", "Failed to connect to proxy: {}", e);
                panic!();
            }
        };

        // Establish connection to dedicated server
        let mut server = match tokio::net::TcpStream::connect(serv_addr).await {
            Ok(socket) => {
                info!(target: "Connection", "User {} has connected to dedicated server", user_addr);
                socket
            }
            Err(e) => {
                warn!(target: "Connection", "User {}: Failed to connect to dedicated server: {}", user_addr, e);
                panic!();
            }
        };

        // Handle user's input
        tokio::spawn(
            async move {
                let task = tokio::io::copy_bidirectional(&mut socket, &mut server);

                if let Err(err) = task.await {
                    tracing::warn!(target: "Data_Transfer", "User {}: Error: {}", user_addr, err);
                };

                tracing::info!(target: "Connections", "User {} has disconnected", user_addr);
            }
            .instrument(tracing::info_span!("Spawned")),
        );
    }
}
