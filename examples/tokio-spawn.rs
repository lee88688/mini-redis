#[tokio::main]
pub async fn main() {
    let handle = tokio::spawn(async {
        "return value"
    });

    let value = handle.await.unwrap();
    println!("{}", value);
}