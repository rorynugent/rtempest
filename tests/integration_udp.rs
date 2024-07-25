use rtempest::mock::MockSender;
use rtempest::test_common::*;
use rtempest::{data::EventType, udp::Tempest};

const PORT: u16 = 50222;

#[tokio::test]
async fn udp() {
    let mock = MockSender::bind();
    let mut receiver = Tempest::listen_udp().await;

    mock.send(get_rain_payload(), PORT);
    mock.send(get_lightning_payload(), PORT);
    mock.send(get_rapidwind_payload(), PORT);
    mock.send(get_air_payload(), PORT);
    mock.send(get_sky_payload(), PORT);
    mock.send(get_station_observation_payload(), PORT);
    mock.send(get_device_payload(), PORT);
    mock.send(get_hub_payload(), PORT);

    let mut success = vec![false; 8];

    while let Some(event) = receiver.recv().await {
        match &event {
            EventType::Rain(event_data) => {
                println!("{event_data}");

                if event_data.get_timestamp() == 1493322445 {
                    success[0] = true;
                    println!("rain");
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
            EventType::Lightning(event_data) => {
                println!("{event_data}");

                if event_data.get_strike_energy() == 3848 {
                    success[1] = true;
                    println!("lightning");
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
            EventType::RapidWind(event_data) => {
                println!("{event_data}");

                if event_data.get_wind_direction() == 128 {
                    println!("wind");
                    success[2] = true;
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
            EventType::Air(event_data) => {
                println!("{event_data}");

                if event_data.get_hub_sn() == "HB-00000001" {
                    success[3] = true;
                    println!("air");
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
            EventType::Sky(event_data) => {
                println!("{event_data}");

                if event_data.get_hub_sn() == "HB-00000001" {
                    success[4] = true;
                    println!("sky");
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
            EventType::Observation(event_data) => {
                println!("{event_data}");

                if event_data.get_hub_sn() == "HB-00013030" {
                    success[5] = true;
                    println!("observation");
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
            EventType::DeviceStatus(event_data) => {
                println!("{event_data}");

                if event_data.get_rssi() == -17 {
                    success[6] = true;
                    println!("device");
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
            EventType::HubStatus(event_data) => {
                println!("{event_data}");

                if event_data.get_uptime() == 1670133 {
                    success[7] = true;
                    println!("hub");
                }

                if finished(&success) {
                    println!("success");
                    return;
                }
            }
        }
    }
}

fn finished(success: &Vec<bool>) -> bool {
    !success.contains(&false)
}
