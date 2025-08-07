use theta_flume;

#[async_std::main]
async fn main() {
    let (tx, rx) = theta_flume::bounded(1);

    // Test that async methods are available by default
    tx.send_async("Test async functionality").await.unwrap();
    
    let result = rx.recv_async().await.unwrap();
    println!("Successfully received: {}", result);
    
    println!("✅ Async functionality is working by default!");
}
