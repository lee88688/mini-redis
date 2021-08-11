use mini_redis::client;
use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get(String),
    Set(String, Bytes),
}

#[tokio::main]
async fn main() {
    // let mut client = client::connect("127.0.0.1:6357").await.unwrap();
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        // let res = client.get("hello").await.unwrap();
        // println!("{:?}", res);
        tx.send("tx1").await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        // let res = client.set("foo", "bar".into());
        tx2.send("tx2").await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("get: {}", msg);
    }
}