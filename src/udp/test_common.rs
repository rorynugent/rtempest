//! Common test functions and test payloads

use serde_json::json;

pub fn get_lightning_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
        "serial_number": "ST-00000512",
        "type":"evt_strike",
        "hub_sn": "HB-00000001",
        "evt":[1493322445,27,3848]
      }))
    .expect("Failed to convert JSON to vector")
}

pub fn get_hub_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
      "serial_number":"HB-00013030",
      "type":"hub_status",
      "firmware_revision":"35",
      "uptime":1670133,
      "rssi":-62,
      "timestamp":1495724691,
      "reset_flags": "BOR,PIN,POR",
      "seq": 48,
      "fs": [1, 0, 15675411, 524288],
          "radio_stats": [2, 1, 0, 3, 2839],
          "mqtt_stats": [1, 0]
    }))
    .expect("Failured to convert JSON to vector")
}

pub fn get_device_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
            "serial_number": "AR-00004049",
            "type": "device_status",
            "hub_sn": "HB-00000001",
            "timestamp": 1510855923,
            "uptime": 2189,
            "voltage": 3.50,
            "firmware_revision": 17,
            "rssi": -17,
            "hub_rssi": -87,
            "sensor_status": 0,
            "debug": 0
    }))
    .expect("Failed to convert JSON to vector")
}

pub fn get_station_observation_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
        "serial_number": "ST-00000512",
        "type": "obs_st",
        "hub_sn": "HB-00013030",
        "obs": [
            [1588948614,0.18,0.22,0.27,144,6,1017.57,22.37,50.26,328,0.03,3,0.000000,0,0,0,2.410,1]
        ],
        "firmware_revision": 129
    }))
    .expect("Failured to convert JSON to vector")
}

pub fn get_secondary_station_observation_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
        "serial_number": "ST-00000513",
        "type": "obs_st",
        "hub_sn": "HB-00013030",
        "obs": [
            [1588948614,0.18,0.22,0.27,144,6,1017.57,22.37,50.26,328,0.03,3,0.000000,0,0,0,2.410,1]
        ],
        "firmware_revision": 129
    }))
    .expect("Failured to convert JSON to vector")
}

pub fn get_rain_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
        "serial_number": "ST-00000512",
        "type":"evt_precip",
        "hub_sn": "HB-00000001",
        "evt":[1493322445]
      }))
    .expect("Failed to convert JSON to vector")
}

pub fn get_rapidwind_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
        "serial_number": "ST-00000512",
        "type":"rapid_wind",
        "hub_sn": "HB-00000001",
        "ob":[1493322445,2.3,128]
      }))
    .expect("Failed to convert JSON to vector")
}

pub fn get_air_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
        "serial_number": "ST-00000512",
        "type":"obs_air",
        "hub_sn": "HB-00000001",
        "obs":[[1493164835,835.0,10.0,45,0,0,3.46,1]],
        "firmware_revision": 17
      }))
    .expect("Failed to convert JSON to vector")
}

pub fn get_sky_payload() -> Vec<u8> {
    serde_json::to_vec(&json!(
    {
        "serial_number": "ST-00000512",
        "type":"obs_sky",
        "hub_sn": "HB-00000001",
        "obs":[[1493321340,9000,10,0.0,2.6,4.6,7.4,187,3.12,1,130,null,0,3]],
        "firmware_revision": 29
      }))
    .expect("Failed to convert JSON to vector")
}
