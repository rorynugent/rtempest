use rtempest::udp::Tempest;

#[tokio::main]
async fn main() {
    let (tempest, mut receiver) = Tempest::listen_udp_with_cache().await;

    while let Some(event) = receiver.recv().await {
        println!("Event: {event:?}");

        println!("Number of hubs cached: {}", tempest.hub_count());
        println!("Number of stations cached: {}", tempest.station_count());
    }

    eprintln!("Channel closed");
}
