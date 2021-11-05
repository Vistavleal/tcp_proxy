#[tokio::test(flavor = "multi_thread")]
async fn test_single_connection() {
    tokio::spawn(crate::run("127.0.0.1:8070"));

    let test_addr = "127.0.0.1:8070".parse().unwrap();
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
