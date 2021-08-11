use std::{collections::HashMap};
use tokio::{net::{TcpListener, TcpStream}};
use mini_redis::{Command, Connection, Frame};
use bytes::Bytes;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let map: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let db = map.clone();

        println!("{:?}", addr);
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("get frame: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Command::Get(cmd) => {
                let map = db.lock().unwrap();
                if let Some(b) = map.get(cmd.key()) {
                    Frame::Bulk(b.clone().into())
                } else {
                    Frame::Null
                }
            },
            Command::Set(cmd) => {
                let mut map = db.lock().unwrap();
                map.insert(cmd.key().to_owned(), cmd.value().clone());
                Frame::Simple("OK".to_owned())
            },
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
