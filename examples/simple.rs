use std::thread;

fn main() {
    let (tx, rx) = theta_flume::unbounded();

    let t = thread::spawn(move || {
        for msg in rx.iter() {
            println!("Received: {}", msg);
        }
    });

    tx.send_blocking("Hello, world!").unwrap();
    tx.send_blocking("How are you today?").unwrap();

    drop(tx);

    t.join().unwrap();
}
