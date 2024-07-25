use rtempest::{data::EventType, udp::Tempest};

#[tokio::main]
async fn main() {
    let mut receiver = Tempest::listen_udp().await;

    while let Some(event) = receiver.recv().await {
        match &event {
            EventType::Rain(event_data) => {
                println!("{event_data}");
            }
            EventType::Lightning(event_data) => {
                println!("{event_data}");
            }
            EventType::RapidWind(event_data) => {
                println!("{event_data}");
            }
            EventType::Air(event_data) => {
                println!("{event_data}");
            }
            EventType::Sky(event_data) => {
                println!("{event_data}");
            }
            EventType::Observation(event_data) => {
                println!("{event_data}");
            }
            EventType::DeviceStatus(event_data) => {
                println!("{event_data}");
            }
            EventType::HubStatus(event_data) => {
                println!("{event_data}");
            }
        }
    }

    eprintln!("Channel closed");
}
