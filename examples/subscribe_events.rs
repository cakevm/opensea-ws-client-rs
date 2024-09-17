use opensea_ws_client_rs::schema::Payload;
use opensea_ws_client_rs::{client, subscribe_to, Collection, Network};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut client = client(Network::Mainnet, "YOUR_API_KEY_HERE").await;

    // Subscribe to a collection. Note that you must all subscribe to all events
    // in the collection; filtering is your responsibility (see below).
    let (handler, mut subscription) = subscribe_to(&mut client, Collection::All).await?;

    // To unsubscribe:
    // handler.close().await?;

    loop {
        // The message received from the channel is a raw message of the Phoenix protocol.
        // It may or may not contain a payload.
        let event = match subscription.recv().await?.into_custom_payload() {
            Some(v) => v,
            None => {
                eprintln!("unexpected message");
                continue;
            }
        };

        // Only print item listing events.
        if let Payload::ItemListed(listing) = event.payload {
            println!("{:?}", listing);
        }
    }
}
