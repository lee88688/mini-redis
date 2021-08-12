use mini_redis::client;
use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    let (tx1, mut rx) = mpsc::channel(32);
    let tx2 = tx1.clone();

    let t1 = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel();
        tx1.send(Command::Get { key: "hello".into(), resp: tx }).await.unwrap();
        let res = rx.await.unwrap();
        println!("got: {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel();
        tx2.send(Command::Set { key: "foo".into(), val: "bar".into(), resp: tx }).await.unwrap();
        let _ = rx.await.unwrap();
        println!("set success!");
    });

    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let val =  client.get(&key).await;
                    resp.send(val).unwrap();
                },
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    resp.send(res).unwrap();
                },
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}