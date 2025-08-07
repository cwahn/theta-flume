#[cfg(not(target_os = "unknown"))]
#[async_std::main]
async fn main() {
    let (tx, rx) = theta_flume::bounded(1);

    let t = async_std::task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Received: {}", msg);
        }
    });

    tx.send("Hello, world!").await.unwrap();
    tx.send("How are you today?").await.unwrap();

    drop(tx);

    t.await;
}

#[cfg(target_os = "unknown")]
fn main() {}
