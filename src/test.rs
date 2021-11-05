use std::{fmt::Result, net::SocketAddr};

#[tokio::test(flavor = "multi_thread")]
async fn test_single_connection() {
    tokio::spawn(crate::run("127.0.0.1:8080".to_string()));

    let test_addr = "127.0.0.1:8080".parse().unwrap();
    let socket = tokio::net::TcpSocket::new_v4().unwrap();
    let stream = match socket.connect(test_addr).await {
        Ok(_socket) => true,
        Err(err) => {
            println!("Error: {}", err);
            false
        }
    };
    assert_eq!(stream, true);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_multiple_connections() {
    tokio::spawn(crate::run("127.0.0.1:8080".to_string()));
    let test_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    // Add 100 connections
    let handles: Vec<_> = (0..100)
        .map(|_| tokio::spawn(async move { connect_user(&test_addr).await }))
        .collect();

    let mut error_happened = false;
    for handle in handles {
        if let Err(e) = handle.await.expect("Task panicked") {
            eprintln!("Task terminated with error: {:#}", e);
            error_happened = true;
        };
    }

    assert!(!error_happened);
}

async fn connect_user(addr: &SocketAddr) -> std::io::Result<()> {
    let socket = tokio::net::TcpSocket::new_v4().unwrap();
    let _stream = match socket.connect(*addr).await {
        Ok(val) => val,
        Err(e) => {
            panic!("Failed to connect to {}: Error: {}", addr, e);
        }
    };
    Ok(())
}
