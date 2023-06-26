use futures::{stream::StreamExt, SinkExt};
use std::{collections::HashMap, error::Error, ffi::CString};
use xpc_connection::{Message, XpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mach_port_name = CString::new("com.example.echo")?;

    println!("Attempting to connect to {:?}", mach_port_name);
    let client = XpcClient::connect(&mach_port_name);

    let (mut tx, mut rx) = client.split();

    tokio::spawn(async move {
        while let Some(message) = rx.next().await {
            println!("Client received message {:?}", message);
        }
    });

    loop {
        // Send message per second
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Sending a message");

        let mut dictionary = HashMap::new();
        dictionary.insert(CString::new("hello")?, Message::Int64(2));

        let _ = tx.send(Message::Dictionary(dictionary)).await;
    }
}
