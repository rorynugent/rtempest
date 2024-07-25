//! Data structures for managing WeatherFlow Tempest weather data

use serde::{Deserialize, Serialize};
use std::fmt;

/// Weather event types
#[derive(Debug, Clone)]
pub enum EventType {
    Rain(RainStartEvent),
    Lightning(LightningStrikeEvent),
    RapidWind(RapidWindEvent),
    Observation(ObservationEvent),
    Air(ObservationAirEvent),
    Sky(ObservationSkyEvent),
    DeviceStatus(DeviceStatusEvent),
    HubStatus(HubStatusEvent),
}

impl From<HubStatusEvent> for Hub {
    /// Returns a `Hub` created from `HubStatusEvent`
    fn from(evt: HubStatusEvent) -> Self {
        Self {
            serial_number: evt.serial_number,
            firmware_revision: evt.firmware_revision,
            uptime: evt.uptime,
            rssi: evt.rssi,
            timestamp: evt.timestamp,
            reset_flags: evt.reset_flags.split(',').map(|s| s.to_string()).collect(),
            seq: evt.seq,
            fs: evt.fs,
            radio_stats: RadioStats {
                version: *evt.radio_stats.first().unwrap_or(&0),
                reboot_count: *evt.radio_stats.get(1).unwrap_or(&0),
                i2c_bus_error_count: *evt.radio_stats.get(2).unwrap_or(&0),
                radio_status: match *evt.radio_stats.get(3).unwrap_or(&0) {
                    1 => RadioStatus::RadioOn,
                    2 => RadioStatus::RadioActive,
                    3 => RadioStatus::RadioActive,
                    7 => RadioStatus::BLEConnected,
                    _ => RadioStatus::RadioOff,
                },
                radio_network_id: *evt.radio_stats.get(4).unwrap_or(&0),
            },
            mqtt_stats: evt.mqtt_stats,
        }
    }
}

/// General cached hub related information
#[derive(Debug, Clone)]
pub struct Hub {
    pub serial_number: String,
    pub firmware_revision: String,
    pub uptime: u64,
    pub rssi: i16,
    pub timestamp: u64,
    pub reset_flags: Vec<String>,
    pub seq: u32,
    pub fs: Option<Vec<u32>>,
    pub radio_stats: RadioStats,
    pub mqtt_stats: Vec<u8>,
}

impl fmt::Display for Hub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( Serial Number: {}, Firmware Revision: {}, Uptime: {}, RSSI: {}, Timestamp: {}, Reset Flags: {:?}, Seq: {}, Fs: {:?}, {:?}, MQTT Stats: {:?} )",
            self.serial_number,
            self.firmware_revision,
            self.uptime,
            self.rssi,
            self.timestamp,
            self.reset_flags,
            self.seq,
            self.fs,
            self.radio_stats,
            self.mqtt_stats,
        )
    }
}

/// General cached hub information
#[derive(Debug, Clone)]
pub struct Station {
    // general station info
    pub hub_sn: String,
    pub firmware_revision: Option<u16>,
    pub serial_number: String,
    pub battery_voltage: Option<f32>,
    // common weather data
    pub air_temperature: Option<f32>,
    pub station_pressure: Option<f32>,
    pub relative_humidity: Option<f32>,
    pub lightning_strike_count: Option<f32>,
    pub lightning_strike_avg_distance: Option<f32>,
    pub illuminance: Option<f32>,
    pub uv: Option<f32>,
    pub rain_amount_prev_minute: Option<f32>,
    pub prev_rain_timestamp: Option<u64>,
    pub wind_lull: Option<f32>,
    pub wind_avg: Option<f32>,
    pub wind_gust: Option<f32>,
    pub wind_direction: Option<f32>,
    pub solar_radiation: Option<f32>,
    pub precipitation_type: Option<PrecipitationType>,
    // events
    pub observation: Option<ObservationEvent>,
    pub wind_event: Option<RapidWindEvent>,
    pub rain_event: Option<RainStartEvent>,
    pub lightning_event: Option<LightningStrikeEvent>,
    pub air_event: Option<ObservationAirEvent>,
    pub sky_event: Option<ObservationSkyEvent>,
    pub device_status: Option<DeviceStatusEvent>,
}

impl From<ObservationEvent> for Station {
    /// Retuns a `Station` created from an `ObservationEvent`
    fn from(event: ObservationEvent) -> Self {
        Self {
            // general station info
            hub_sn: event.get_hub_sn(),
            firmware_revision: Some(event.get_firmware_revision()),
            serial_number: event.get_serial_number(),
            battery_voltage: event.get_battery_voltage().ok(),
            // common weather data
            air_temperature: event.get_air_temperature().ok(),
            station_pressure: event.get_station_pressure().ok(),
            relative_humidity: event.get_rh().ok(),
            lightning_strike_count: event.get_lightning_avg_distance().ok(),
            lightning_strike_avg_distance: event.get_lightning_strike_count().ok(),
            illuminance: event.get_illuminance().ok(),
            uv: event.get_uv().ok(),
            rain_amount_prev_minute: event.get_rain_amount_prev_min().ok(),
            prev_rain_timestamp: None,
            wind_lull: event.get_wind_lull().ok(),
            wind_avg: event.get_wind_gust().ok(),
            wind_gust: event.get_wind_gust().ok(),
            wind_direction: event.get_solar_radiation().ok(),
            solar_radiation: event.get_solar_radiation().ok(),
            precipitation_type: event.get_precip_type().ok(),
            // events
            observation: Some(event),
            wind_event: None,
            rain_event: None,
            lightning_event: None,
            air_event: None,
            sky_event: None,
            device_status: None,
        }
    }
}

impl From<RapidWindEvent> for Station {
    /// Retuns a `Station` created from an `RapidWindEvent`
    fn from(event: RapidWindEvent) -> Self {
        Self {
            // general station info
            hub_sn: event.get_hub_sn(),
            firmware_revision: None,
            serial_number: event.get_serial_number(),
            battery_voltage: None,
            // common weather data
            air_temperature: None,
            station_pressure: None,
            relative_humidity: None,
            lightning_strike_count: None,
            lightning_strike_avg_distance: None,
            illuminance: None,
            uv: None,
            rain_amount_prev_minute: None,
            prev_rain_timestamp: None,
            wind_lull: None,
            wind_avg: None,
            wind_gust: None,
            wind_direction: None,
            solar_radiation: None,
            precipitation_type: None,
            // events
            observation: None,
            wind_event: Some(event),
            rain_event: None,
            lightning_event: None,
            air_event: None,
            sky_event: None,
            device_status: None,
        }
    }
}

impl From<RainStartEvent> for Station {
    /// Retuns a `Station` created from an `RainStartEvent`
    fn from(event: RainStartEvent) -> Self {
        Self {
            // general station info
            hub_sn: event.get_serial_number(),
            firmware_revision: None,
            serial_number: event.get_serial_number(),
            battery_voltage: None,
            // common weather data
            air_temperature: None,
            station_pressure: None,
            relative_humidity: None,
            lightning_strike_count: None,
            lightning_strike_avg_distance: None,
            illuminance: None,
            uv: None,
            rain_amount_prev_minute: None,
            prev_rain_timestamp: Some(event.get_timestamp()),
            wind_lull: None,
            wind_avg: None,
            wind_gust: None,
            wind_direction: None,
            solar_radiation: None,
            precipitation_type: None,
            // events
            observation: None,
            wind_event: None,
            rain_event: Some(event),
            lightning_event: None,
            air_event: None,
            sky_event: None,
            device_status: None,
        }
    }
}

impl From<LightningStrikeEvent> for Station {
    /// Retuns a `Station` created from an `LightningStrikeEvent`
    fn from(event: LightningStrikeEvent) -> Self {
        Self {
            // general station info
            hub_sn: event.get_hub_sn(),
            firmware_revision: None,
            serial_number: event.get_serial_number(),
            battery_voltage: None,
            // common weather data
            air_temperature: None,
            station_pressure: None,
            relative_humidity: None,
            lightning_strike_count: None,
            lightning_strike_avg_distance: None,
            illuminance: None,
            uv: None,
            rain_amount_prev_minute: None,
            prev_rain_timestamp: None,
            wind_lull: None,
            wind_avg: None,
            wind_gust: None,
            wind_direction: None,
            solar_radiation: None,
            precipitation_type: None,
            // events
            observation: None,
            wind_event: None,
            rain_event: None,
            lightning_event: Some(event),
            air_event: None,
            sky_event: None,
            device_status: None,
        }
    }
}

impl From<ObservationAirEvent> for Station {
    /// Retuns a `Station` created from an `ObservationAirEvent`
    fn from(event: ObservationAirEvent) -> Self {
        Self {
            // general station info
            hub_sn: event.get_hub_sn(),
            firmware_revision: Some(event.get_firmware_revision()),
            serial_number: event.get_serial_number(),
            battery_voltage: event.get_battery_voltage().ok(),
            // common weather data
            air_temperature: event.get_air_temperature().ok(),
            station_pressure: event.get_station_pressure().ok(),
            relative_humidity: event.get_relative_humidity().ok(),
            lightning_strike_count: event.get_lightning_count().ok(),
            lightning_strike_avg_distance: event.get_lightning_avg_distance().ok(),
            illuminance: None,
            uv: None,
            rain_amount_prev_minute: None,
            prev_rain_timestamp: None,
            wind_lull: None,
            wind_avg: None,
            wind_gust: None,
            wind_direction: None,
            solar_radiation: None,
            precipitation_type: None,
            // events
            observation: None,
            wind_event: None,
            rain_event: None,
            lightning_event: None,
            air_event: Some(event),
            sky_event: None,
            device_status: None,
        }
    }
}

impl From<ObservationSkyEvent> for Station {
    /// Retuns a `Station` created from an `ObservationSkyEvent`
    fn from(event: ObservationSkyEvent) -> Self {
        Self {
            // general station info
            hub_sn: event.get_hub_sn(),
            firmware_revision: Some(event.get_firmware_revision()),
            serial_number: event.get_serial_number(),
            battery_voltage: event.get_battery_voltage().ok().unwrap_or_default(),
            // common weather data
            air_temperature: None,
            station_pressure: None,
            relative_humidity: None,
            lightning_strike_count: None,
            lightning_strike_avg_distance: None,
            illuminance: event.get_illuminance().unwrap_or_default(),
            uv: event.get_uv().ok().unwrap_or_default(),
            rain_amount_prev_minute: event.get_rain_prev_min().ok().unwrap_or_default(),
            prev_rain_timestamp: None,
            wind_lull: event.get_wind_lull().ok().unwrap_or_default(),
            wind_avg: event.get_wind_avg().ok().unwrap_or_default(),
            wind_gust: event.get_wind_gust().ok().unwrap_or_default(),
            wind_direction: event.get_wind_direction().ok().unwrap_or_default(),
            solar_radiation: event.get_solar_radiation().ok().unwrap_or_default(),
            precipitation_type: event.get_precip_type().ok(),
            // events
            observation: None,
            wind_event: None,
            rain_event: None,
            lightning_event: None,
            air_event: None,
            sky_event: Some(event),
            device_status: None,
        }
    }
}

impl From<DeviceStatusEvent> for Station {
    /// Retuns a `Station` created from an `DeviceStatusEvent`
    fn from(event: DeviceStatusEvent) -> Self {
        Self {
            // general station info
            hub_sn: event.get_hub_sn(),
            firmware_revision: Some(event.get_firmware_revision()),
            serial_number: event.get_serial_number(),
            battery_voltage: Some(event.get_battery_voltage()),
            // common weather data
            air_temperature: None,
            station_pressure: None,
            relative_humidity: None,
            lightning_strike_count: None,
            lightning_strike_avg_distance: None,
            illuminance: None,
            uv: None,
            rain_amount_prev_minute: None,
            prev_rain_timestamp: None,
            wind_lull: None,
            wind_avg: None,
            wind_gust: None,
            wind_direction: None,
            solar_radiation: None,
            precipitation_type: None,
            // events
            observation: None,
            wind_event: None,
            rain_event: None,
            lightning_event: None,
            air_event: None,
            sky_event: None,
            device_status: Some(event),
        }
    }
}

/// Preciptation types
#[derive(Debug, Clone, PartialEq)]
pub enum PrecipitationType {
    None,
    Rain,
    Hail,
    RainHail, /* Experimental */
}

impl fmt::Display for PrecipitationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrecipitationType::None => "None",
                PrecipitationType::Rain => "Rain",
                PrecipitationType::Hail => "Hail",
                PrecipitationType::RainHail => "Rain + Hail (experimental)",
            }
        )
    }
}

/// Radio statuses
#[derive(Debug, Clone, PartialEq)]
pub enum RadioStatus {
    RadioOff,
    RadioOn,
    RadioActive,
    BLEConnected,
    Unknown,
}

impl fmt::Display for RadioStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RadioStatus::RadioOff => "Radio Off",
                RadioStatus::RadioOn => "Radio On",
                RadioStatus::RadioActive => "Radio Active",
                RadioStatus::BLEConnected => "BLE Connected",
                RadioStatus::Unknown => "Unknown",
            }
        )
    }
}

/// Event error codes
#[derive(Debug, PartialEq)]
pub enum EventError {
    ParseError,
    UnexpectedValue,
}

/// Rain start event for a station
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RainStartEvent {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    evt: Vec<u64>,
}

impl fmt::Display for RainStartEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RainStartEvent Data (Timestamp: {}, Serial Number: {}, Hub Serial Number: {})",
            self.get_timestamp(),
            self.get_serial_number(),
            self.get_hub_sn(),
        )
    }
}

impl RainStartEvent {
    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_hub_sn(&self) -> String {
        self.hub_sn.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.evt[0]
    }
}

/// Lightning strike event for a station
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightningStrikeEvent {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    evt: Vec<u64>,
}

impl fmt::Display for LightningStrikeEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LightningStrikeEvent Data (Timestamp: {}, Serial Number: {}, Hub Serial Number: {}, Strike Distance: {} km, Energy: {})",
        self.get_timestamp(),
        self.get_serial_number(),
        self.get_hub_sn(),
        self.get_strike_distance(),
        self.get_strike_energy())
    }
}

impl LightningStrikeEvent {
    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_hub_sn(&self) -> String {
        self.hub_sn.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.evt[0]
    }

    pub fn get_strike_distance(&self) -> u64 {
        self.evt[1]
    }

    pub fn get_strike_energy(&self) -> u64 {
        self.evt[2]
    }
}

/// Rapid wind event for a station
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RapidWindEvent {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    ob: Vec<f64>,
}

impl fmt::Display for RapidWindEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RapidWindEvent Data (Timestamp: {}, Serial Number: {}, Hub Serial Number: {}, Wind Speed: {} m/s, Wind Direction: {}°)", 
        self.get_timestamp(),
        self.get_serial_number(),
        self.get_hub_sn(),
        self.get_wind_speed_mps(),
        self.get_wind_direction())
    }
}

impl RapidWindEvent {
    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }
    pub fn get_hub_sn(&self) -> String {
        self.hub_sn.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.ob[0] as u64
    }

    pub fn get_wind_speed_mps(&self) -> f32 {
        self.ob[1] as f32
    }

    pub fn get_wind_direction(&self) -> u16 {
        self.ob[2] as u16
    }
}

/// Observation air event for a station
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationAirEvent {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    obs: Vec<Vec<f32>>,
    firmware_revision: u16,
}

impl fmt::Display for ObservationAirEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObservationAirEvent Data (Timestamp: {}, Serial Number: {}, Hub SN: {}, Firmware Revision: {}, Station Pressure: {}, Air Temperature: {}, Relative Humidity: {}%, Lightning Strike Count: {}, Lightning Strike Avg Distance: {} km, Battery Voltage: {}V, Report Interval: {})",
        self.get_timestamp().unwrap_or(0.0),
        self.get_serial_number(),
        self.get_hub_sn(),
        self.get_firmware_revision(),
        self.get_station_pressure().unwrap_or(0.0),
        self.get_air_temperature().unwrap_or(0.0),
        self.get_relative_humidity().unwrap_or(0.0),
        self.get_lightning_count().unwrap_or(0.0),
        self.get_lightning_avg_distance().unwrap_or(0.0),
        self.get_battery_voltage().unwrap_or(0.0),
        self.get_report_interval().unwrap_or(0.0),
        )
    }
}

impl ObservationAirEvent {
    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_hub_sn(&self) -> String {
        self.hub_sn.clone()
    }

    pub fn get_firmware_revision(&self) -> u16 {
        self.firmware_revision
    }

    pub fn get_timestamp(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve timestamp from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[0];

        Ok(data)
    }

    pub fn get_station_pressure(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve station pressure from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[1];

        Ok(data)
    }

    pub fn get_air_temperature(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve air temperature from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[2];

        Ok(data)
    }

    pub fn get_relative_humidity(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve relative humidity from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[3];

        Ok(data)
    }

    pub fn get_lightning_count(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve lightning strike count from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[4];

        Ok(data)
    }

    pub fn get_lightning_avg_distance(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve lightning avg distance from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[5];

        Ok(data)
    }

    pub fn get_battery_voltage(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve battery voltage from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[6];

        Ok(data)
    }

    pub fn get_report_interval(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve report interval from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[7];

        Ok(data)
    }
}

/// Observation sky event for a station
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationSkyEvent {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    obs: Vec<Vec<Option<f32>>>,
    firmware_revision: u16,
}

impl fmt::Display for ObservationSkyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ObservationSkyEvent Data (Timestamp: {:?}, Serial Number: {}, Hub Serial Number: {}, Firmware Revision: {})",
            self.get_timestamp().unwrap_or_default(),
            self.get_serial_number(),
            self.get_hub_sn(),
            self.get_firmware_revision(),
        )
    }
}

impl ObservationSkyEvent {
    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_hub_sn(&self) -> String {
        self.hub_sn.clone()
    }

    pub fn get_firmware_revision(&self) -> u16 {
        self.firmware_revision
    }

    pub fn get_timestamp(&self) -> Result<Option<f32>, EventError> {
        match self.obs.first() {
            Some(obs) => Ok(obs[0]),
            None => {
                eprintln!(
                    "Unable to retrieve timestamp from {}",
                    std::any::type_name::<Self>()
                );
                Err(EventError::ParseError)
            }
        }
    }

    pub fn get_illuminance(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(1).copied()).ok_or({
            eprintln!(
                "Unable to retrieve illuminance from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_uv(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(2).copied()).ok_or({
            eprintln!(
                "Unable to retrieve UV from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_rain_prev_min(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(3).copied()).ok_or({
            eprintln!(
                "Unable to retrieve rain previous minute from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_wind_lull(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(4).copied()).ok_or({
            eprintln!(
                "Unable to retrieve wind lull from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_wind_avg(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(5).copied()).ok_or({
            eprintln!(
                "Unable to retrieve wind avg from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_wind_gust(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(6).copied()).ok_or({
            eprintln!(
                "Unable to retrieve wind gust from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_wind_direction(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(7).copied()).ok_or({
            eprintln!(
                "Unable to retrieve wind direction from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_battery_voltage(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(8).copied()).ok_or({
            eprintln!(
                "Unable to retrieve battery voltage from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_report_interval(&self) -> Result<Option<f32>, EventError> {
        self.obs.first().and_then(|vec| vec.get(9).copied()).ok_or({
            eprintln!(
                "Unable to retrieve report interval from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })
    }

    pub fn get_solar_radiation(&self) -> Result<Option<f32>, EventError> {
        self.obs
            .first()
            .and_then(|vec| vec.get(10).copied())
            .ok_or({
                eprintln!(
                    "Unable to retrieve solar radiation from {}",
                    std::any::type_name::<Self>()
                );
                EventError::ParseError
            })
    }

    pub fn get_local_day_rain_accum(&self) -> Result<Option<f32>, EventError> {
        self.obs
            .first()
            .and_then(|vec| vec.get(11).copied())
            .ok_or({
                eprintln!(
                    "Unable to retrieve local day rain accumulation from {}",
                    std::any::type_name::<Self>()
                );
                EventError::ParseError
            })
    }

    pub fn get_precip_type(&self) -> Result<PrecipitationType, EventError> {
        match self
            .obs
            .first()
            .and_then(|vec| vec.get(12).copied())
            .unwrap_or_default()
        {
            Some(precip) => match precip as u16 {
                0 => Ok(PrecipitationType::None),
                1 => Ok(PrecipitationType::Rain),
                2 => Ok(PrecipitationType::Hail),
                3 => Ok(PrecipitationType::RainHail),
                _ => {
                    eprintln!("Unknown precipitation type");
                    Err(EventError::UnexpectedValue)
                }
            },
            None => {
                eprintln!(
                    "Unable to retrieve precipitation type from {}",
                    std::any::type_name::<Self>()
                );
                Err(EventError::ParseError)
            }
        }
    }

    pub fn get_wind_sample_interval(&self) -> Result<Option<f32>, EventError> {
        match self.obs.first() {
            Some(obs) => Ok(obs[13]),
            None => {
                eprintln!(
                    "Unable to retrieve wind sample interval from {}",
                    std::any::type_name::<Self>()
                );
                Err(EventError::ParseError)
            }
        }
    }
}

/// Observation event for a station
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationEvent {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    obs: Vec<Vec<f32>>,
    firmware_revision: u16,
}

impl fmt::Display for ObservationEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObservationEvent Data (Timestamp: {}, Serial Number: {}, Hub SN: {}, Firmware Revision: {}, Wind Lull: {} m/s, Wind Avg: {} m/s, Wind Gust: {} m/s, Wind Direction: {}°, Wind Sample Interval: {}s, Station Pressure: {} MB, Air Temperature: {}°C, Relative Humidity: {}%, Illuminance: {} lux, UV: {} Index, Solar Radiation: {} W/m^2, Rain Previous Minute: {} mm, Precipitation: {}, Lightning Strike Avg Distance: {} km, Lightning Strike Count: {}, Battery: {}V, Report Interval: {} min(s)",
            self.get_timestamp().unwrap_or(0.0),
            self.get_serial_number(),
            self.get_hub_sn(),
            self.get_firmware_revision(),
            self.get_wind_lull().unwrap_or(0.0),
            self.get_wind_avg().unwrap_or(0.0),
            self.get_wind_gust().unwrap_or(0.0),
            self.get_wind_direction().unwrap_or(0.0),
            self.get_wind_sample_interval().unwrap_or(0.0),
            self.get_station_pressure().unwrap_or(0.0),
            self.get_air_temperature().unwrap_or(0.0),
            self.get_rh().unwrap_or(0.0),
            self.get_illuminance().unwrap_or(0.0),
            self.get_uv().unwrap_or(0.0),
            self.get_solar_radiation().unwrap_or(0.0),
            self.get_rain_amount_prev_min().unwrap_or(0.0),
            self.get_precip_type().unwrap_or(PrecipitationType::None),
            self.get_lightning_avg_distance().unwrap_or(0.0),
            self.get_lightning_strike_count().unwrap_or(0.0),
            self.get_battery_voltage().unwrap_or(0.0),
            self.get_report_interval().unwrap_or(0.0),
        )
    }
}

impl ObservationEvent {
    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_hub_sn(&self) -> String {
        self.hub_sn.clone()
    }

    pub fn get_firmware_revision(&self) -> u16 {
        self.firmware_revision
    }

    pub fn get_timestamp(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve timestamp from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[0];

        Ok(data)
    }

    pub fn get_wind_lull(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve wind lull from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[1];

        Ok(data)
    }

    pub fn get_wind_avg(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve wind average from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[2];

        Ok(data)
    }

    pub fn get_wind_gust(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve wind gust from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[3];

        Ok(data)
    }

    pub fn get_wind_direction(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve wind direction from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[4];

        Ok(data)
    }

    pub fn get_wind_sample_interval(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve wind sample interval from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[5];

        Ok(data)
    }

    pub fn get_station_pressure(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve station pressure from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[6];

        Ok(data)
    }

    pub fn get_air_temperature(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve air temperature from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[7];

        Ok(data)
    }

    pub fn get_rh(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve R/H from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[8];

        Ok(data)
    }

    pub fn get_illuminance(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve illuminance from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[9];

        Ok(data)
    }

    pub fn get_uv(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve UV from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[10];

        Ok(data)
    }

    pub fn get_solar_radiation(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve solar radiation from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[11];

        Ok(data)
    }

    pub fn get_rain_amount_prev_min(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve previous minute's rain amount from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[12];

        Ok(data)
    }

    pub fn get_precip_type(&self) -> Result<PrecipitationType, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve precipitation type from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[13];

        match data as u16 {
            0 => Ok(PrecipitationType::None),
            1 => Ok(PrecipitationType::Rain),
            2 => Ok(PrecipitationType::Hail),
            3 => Ok(PrecipitationType::RainHail),
            _ => {
                eprintln!("Unknown precipitation type");
                Err(EventError::UnexpectedValue)
            }
        }
    }

    pub fn get_lightning_avg_distance(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve average distance of lighting strike from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[14];

        Ok(data)
    }

    pub fn get_lightning_strike_count(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve lightning strike count from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[15];

        Ok(data)
    }

    pub fn get_battery_voltage(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve battery voltage from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[16];

        Ok(data)
    }

    pub fn get_report_interval(&self) -> Result<f32, EventError> {
        let data = self.obs.first().ok_or_else(|| {
            eprintln!(
                "Unable to retrieve report interval from {}",
                std::any::type_name::<Self>()
            );
            EventError::ParseError
        })?[17];

        Ok(data)
    }
}

/// Device status event for a station
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceStatusEvent {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    timestamp: u64,
    uptime: u64,
    voltage: f32,
    firmware_revision: u16,
    rssi: i16,
    hub_rssi: i16,
    sensor_status: u32,
    debug: u8,
}

impl fmt::Display for DeviceStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceStatusEvent Data (Timestamp: {}, Serial Number: {}, Hub SN: {}, Firmware Revision: {}, Uptime: {}, Battery Voltage: {}V, RSSI: {}, Hub RSSI: {}, Debug Enabled: {})",
        self.get_timestamp(),
        self.get_serial_number(),
        self.get_hub_sn(),
        self.get_firmware_revision(),
        self.get_uptime(),
        self.get_battery_voltage(),
        self.get_rssi(),
        self.get_hub_rssi(),
        self.debugging_enabled())
    }
}

impl DeviceStatusEvent {
    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_hub_sn(&self) -> String {
        self.hub_sn.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_uptime(&self) -> u64 {
        self.uptime
    }

    pub fn get_battery_voltage(&self) -> f32 {
        self.voltage
    }

    pub fn get_firmware_revision(&self) -> u16 {
        self.firmware_revision
    }

    pub fn get_rssi(&self) -> i16 {
        self.rssi
    }

    pub fn get_hub_rssi(&self) -> i16 {
        self.hub_rssi
    }

    pub fn debugging_enabled(&self) -> bool {
        self.debug != 0
    }
}

/// Hub status event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubStatusEvent {
    serial_number: String,
    r#type: String,
    firmware_revision: String,
    uptime: u64,
    rssi: i16,
    timestamp: u64,
    reset_flags: String,
    seq: u32,
    fs: Option<Vec<u32>>,
    radio_stats: Vec<u16>,
    mqtt_stats: Vec<u8>,
}

impl fmt::Display for HubStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HubStatusEvent Data (Timestamp: {}, Serial Number: {}, Firmware Revision: {}, Uptime: {}, RSSI: {}, Reset Flags: \"{}\", Radio Status: {}, Radio Network ID: {})",
        self.get_timestamp(),
        self.get_serial_number(),
        self.get_firmware_revision(),
        self.get_uptime(),
        self.get_rssi(),
        self.get_reset_flags(),
        self.get_radio_status(),
        self.get_radio_network_id())
    }
}

impl HubStatusEvent {
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }

    pub fn get_firmware_revision(&self) -> String {
        self.firmware_revision.clone()
    }

    pub fn get_uptime(&self) -> u64 {
        self.uptime
    }

    pub fn get_rssi(&self) -> i16 {
        self.rssi
    }

    pub fn get_reset_flags(&self) -> String {
        self.reset_flags.clone()
    }

    pub fn get_radio_version(&self) -> u16 {
        self.radio_stats[0]
    }

    pub fn get_radio_reboot_count(&self) -> u16 {
        self.radio_stats[1]
    }

    pub fn get_radio_i2c_error_count(&self) -> u16 {
        self.radio_stats[2]
    }

    pub fn get_radio_status(&self) -> RadioStatus {
        match self.radio_stats[3] {
            0 => RadioStatus::RadioOff,
            1 => RadioStatus::RadioOn,
            3 => RadioStatus::RadioActive,
            7 => RadioStatus::BLEConnected,
            _ => RadioStatus::Unknown,
        }
    }

    pub fn get_radio_network_id(&self) -> u16 {
        self.radio_stats[4]
    }
}

/// Radio stats from a hub status event
#[derive(Debug, Clone)]
pub struct RadioStats {
    pub version: u16,
    pub reboot_count: u16,
    pub i2c_bus_error_count: u16,
    pub radio_status: RadioStatus,
    pub radio_network_id: u16,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_to_observation() {
        let json = b"{
            \"serial_number\": \"ST-00000512\",
            \"type\": \"obs_st\" ,
            \"hub_sn\": \"HB-00013030\",
            \"obs\": [
                [1588948614,0.18,0.22,0.27,144,6,1017.57,22.37,50.26,328,0.03,3,0.000000,0,0,0,2.410,1]
            ],
            \"firmware_revision\": 129
        }";

        let observation: ObservationEvent =
            serde_json::from_slice(json).expect("Unable to convert JSON to ObservationEvent");

        assert_eq!(observation.serial_number, "ST-00000512");
    }

    #[test]
    fn hubstatus_into_hub() {
        let hub_status = HubStatusEvent {
            serial_number: "HB-00000001".to_string(),
            r#type: "hub_status".to_string(),
            firmware_revision: "35".to_string(),
            uptime: 1670133,
            rssi: -62,
            timestamp: 1495724691,
            reset_flags: "BOR,PIN,POR".to_string(),
            seq: 48,
            fs: Some(vec![1, 0, 15675411, 524288]),
            radio_stats: vec![2, 1, 0, 3, 2839],
            mqtt_stats: vec![1, 0],
        };

        let hub: Hub = hub_status.clone().into();

        assert_eq!(hub.serial_number, "HB-00000001");
    }

    #[test]
    fn observation_into_station() {
        let observation = ObservationEvent {
            serial_number: "ST-00000512".to_string(),
            hub_sn: "HB-00013030".to_string(),
            firmware_revision: 129,
            r#type: "obs_st".to_string(),
            obs: vec![vec![
                1588948614.0,
                0.18,
                0.22,
                0.27,
                144.0,
                6.0,
                1017.57,
                22.37,
                50.26,
                328.0,
                0.03,
                3.0,
                0.000000,
                0.0,
                0.0,
                0.0,
                2.410,
                1.0,
            ]],
        };

        let station: Station = observation.clone().into();

        assert_eq!(station.serial_number, "ST-00000512");

        assert_eq!(station.observation, Some(observation));
    }

    #[test]
    fn rapidwind_into_station() {
        let rapidwind = RapidWindEvent {
            serial_number: "SK-00008453".to_string(),
            r#type: "rapid_wind".to_string(),
            hub_sn: "HB-00000001".to_string(),
            ob: vec![1493322445.0, 2.3, 128.0],
        };

        let station: Station = rapidwind.clone().into();

        assert_eq!(station.serial_number, "SK-00008453");

        assert_eq!(station.wind_event, Some(rapidwind));
    }

    #[test]
    fn rain_into_station() {
        let rain = RainStartEvent {
            serial_number: "SK-00008453".to_string(),
            r#type: "evt_precip".to_string(),
            hub_sn: "HB-00000001".to_string(),
            evt: vec![1493322445],
        };

        let station: Station = rain.clone().into();

        assert_eq!(station.serial_number, "SK-00008453");

        assert_eq!(station.rain_event, Some(rain));
    }

    #[test]
    fn lightning_into_station() {
        let lightning = LightningStrikeEvent {
            serial_number: "AR-00004049".to_string(),
            r#type: "evt_strike".to_string(),
            hub_sn: "HB-00000001".to_string(),
            evt: vec![1493322445, 27, 3848],
        };

        let station: Station = lightning.clone().into();

        assert_eq!(station.serial_number, "AR-00004049");

        assert_eq!(station.lightning_event, Some(lightning));
    }

    #[test]
    fn air_into_station() {
        let air = ObservationAirEvent {
            serial_number: "AR-00004049".to_string(),
            r#type: "obs_air".to_string(),
            hub_sn: "HB-00000001".to_string(),
            firmware_revision: 17,
            obs: vec![vec![1493164835.0, 835.0, 10.0, 45.0, 0.0, 0.0, 3.46, 1.0]],
        };

        let station: Station = air.clone().into();

        assert_eq!(station.serial_number, "AR-00004049");

        assert_eq!(station.air_event, Some(air));
    }

    #[test]
    fn sky_into_station() {
        let sky = ObservationSkyEvent {
            serial_number: "SK-00008453".to_string(),
            r#type: "obs_sky".to_string(),
            hub_sn: "HB-00000001".to_string(),
            firmware_revision: 29,
            obs: vec![vec![
                Some(1493321340.0),
                Some(9000.0),
                Some(10.0),
                Some(0.0),
                Some(2.6),
                Some(4.6),
                Some(7.4),
                Some(187.0),
                Some(3.12),
                Some(1.0),
                Some(130.0),
                None,
                Some(0.0),
                Some(3.0),
            ]],
        };

        let station: Station = sky.clone().into();

        assert_eq!(station.serial_number, "SK-00008453");

        assert_eq!(station.sky_event, Some(sky));
    }

    #[test]
    fn devicestatus_into_station() {
        let device = DeviceStatusEvent {
            serial_number: "AR-00004049".to_string(),
            r#type: "device_status".to_string(),
            hub_sn: "HB-00000001".to_string(),
            timestamp: 1510855923,
            uptime: 2189,
            voltage: 3.50,
            firmware_revision: 17,
            rssi: -17,
            hub_rssi: -87,
            sensor_status: 0,
            debug: 0,
        };

        let station: Station = device.clone().into();

        assert_eq!(station.serial_number, "AR-00004049");

        assert_eq!(station.device_status, Some(device));
    }

    #[test]
    fn get_data_from_rainstart_event() {
        let rain = RainStartEvent {
            serial_number: "SK-00008453".to_string(),
            r#type: "evt_precip".to_string(),
            hub_sn: "HB-00000001".to_string(),
            evt: vec![1493322445],
        };

        assert_eq!(rain.get_serial_number(), "SK-00008453");
        assert_eq!(rain.get_hub_sn(), "HB-00000001");
        assert_eq!(rain.get_timestamp(), 1493322445);
    }

    #[test]
    fn get_data_from_lightning_event() {
        let lightning = LightningStrikeEvent {
            serial_number: "AR-00004049".to_string(),
            r#type: "evt_strike".to_string(),
            hub_sn: "HB-00000001".to_string(),
            evt: vec![1493322445, 27, 3848],
        };

        assert_eq!(lightning.get_serial_number(), "AR-00004049");
        assert_eq!(lightning.get_hub_sn(), "HB-00000001");
        assert_eq!(lightning.get_timestamp(), 1493322445);
        assert_eq!(lightning.get_strike_distance(), 27);
        assert_eq!(lightning.get_strike_energy(), 3848);
    }

    #[test]
    fn get_data_from_rapidwind_event() {
        let rapidwind = RapidWindEvent {
            serial_number: "SK-00008453".to_string(),
            r#type: "rapid_wind".to_string(),
            hub_sn: "HB-00000001".to_string(),
            ob: vec![1493322445.0, 2.3, 128.0],
        };

        assert_eq!(rapidwind.get_serial_number(), "SK-00008453");
        assert_eq!(rapidwind.get_hub_sn(), "HB-00000001");
        assert_eq!(rapidwind.get_timestamp(), 1493322445);
        assert_eq!(rapidwind.get_wind_speed_mps(), 2.3);
        assert_eq!(rapidwind.get_wind_direction(), 128);
    }

    #[test]
    fn get_data_from_observationair_event() {
        let air = ObservationAirEvent {
            serial_number: "AR-00004049".to_string(),
            r#type: "obs_air".to_string(),
            hub_sn: "HB-00000001".to_string(),
            firmware_revision: 17,
            obs: vec![vec![1493164835.0, 835.0, 10.0, 45.0, 0.0, 0.0, 3.46, 1.0]],
        };

        assert_eq!(air.get_serial_number(), "AR-00004049");
        assert_eq!(air.get_hub_sn(), "HB-00000001");
        assert_eq!(air.get_firmware_revision(), 17);
        assert_eq!(air.get_timestamp(), Ok(1493164835.0));
        assert_eq!(air.get_station_pressure(), Ok(835.0));
        assert_eq!(air.get_air_temperature(), Ok(10.0));
        assert_eq!(air.get_relative_humidity(), Ok(45.0));
        assert_eq!(air.get_lightning_count(), Ok(0.0));
        assert_eq!(air.get_lightning_avg_distance(), Ok(0.0));
        assert_eq!(air.get_battery_voltage(), Ok(3.46));
        assert_eq!(air.get_report_interval(), Ok(1.0));
    }

    #[test]
    fn get_data_from_observationsky_event() {
        let sky = ObservationSkyEvent {
            serial_number: "SK-00008453".to_string(),
            r#type: "obs_sky".to_string(),
            hub_sn: "HB-00000001".to_string(),
            firmware_revision: 29,
            obs: vec![vec![
                Some(1493321340.0),
                Some(9000.0),
                Some(10.0),
                Some(0.0),
                Some(2.6),
                Some(4.6),
                Some(7.4),
                Some(187.0),
                Some(3.12),
                Some(1.0),
                Some(130.0),
                Some(0.0),
                Some(0.0),
                Some(3.0),
            ]],
        };

        assert_eq!(sky.get_serial_number(), "SK-00008453");
        assert_eq!(sky.get_hub_sn(), "HB-00000001");
        assert_eq!(sky.get_firmware_revision(), 29);
        assert_eq!(sky.get_timestamp(), Ok(Some(1493321340.0)));
        assert_eq!(sky.get_illuminance(), Ok(Some(9000.0)));
        assert_eq!(sky.get_uv(), Ok(Some(10.0)));
        assert_eq!(sky.get_rain_prev_min(), Ok(Some(0.0)));
        assert_eq!(sky.get_wind_lull(), Ok(Some(2.6)));
        assert_eq!(sky.get_wind_avg(), Ok(Some(4.6)));
        assert_eq!(sky.get_wind_gust(), Ok(Some(7.4)));
        assert_eq!(sky.get_wind_direction(), Ok(Some(187.0)));
        assert_eq!(sky.get_battery_voltage(), Ok(Some(3.12)));
        assert_eq!(sky.get_report_interval(), Ok(Some(1.0)));
        assert_eq!(sky.get_solar_radiation(), Ok(Some(130.0)));
        assert_eq!(sky.get_local_day_rain_accum(), Ok(Some(0.0)));
        assert_eq!(sky.get_precip_type(), Ok(PrecipitationType::None));
        assert_eq!(sky.get_wind_sample_interval(), Ok(Some(3.0)));
    }

    #[test]
    fn get_data_from_observationevent() {
        let observation = ObservationEvent {
            serial_number: "ST-00000512".to_string(),
            hub_sn: "HB-00013030".to_string(),
            firmware_revision: 129,
            r#type: "obs_st".to_string(),
            obs: vec![vec![
                1588948614.0,
                0.18,
                0.22,
                0.27,
                144.0,
                6.0,
                1017.57,
                22.37,
                50.26,
                328.0,
                0.03,
                3.0,
                0.000000,
                0.0,
                0.0,
                0.0,
                2.410,
                1.0,
            ]],
        };

        assert_eq!(observation.get_serial_number(), "ST-00000512");
        assert_eq!(observation.get_hub_sn(), "HB-00013030");
        assert_eq!(observation.get_firmware_revision(), 129);
        assert_eq!(observation.get_timestamp(), Ok(1588948614.0));
        assert_eq!(observation.get_wind_lull(), Ok(0.18));
        assert_eq!(observation.get_wind_avg(), Ok(0.22));
        assert_eq!(observation.get_wind_gust(), Ok(0.27));
        assert_eq!(observation.get_wind_direction(), Ok(144.0));
        assert_eq!(observation.get_wind_sample_interval(), Ok(6.0));
        assert_eq!(observation.get_station_pressure(), Ok(1017.57));
        assert_eq!(observation.get_air_temperature(), Ok(22.37));
        assert_eq!(observation.get_rh(), Ok(50.26));
        assert_eq!(observation.get_illuminance(), Ok(328.0));
        assert_eq!(observation.get_uv(), Ok(0.03));
        assert_eq!(observation.get_solar_radiation(), Ok(3.0));
        assert_eq!(observation.get_rain_amount_prev_min(), Ok(0.000000));
        assert_eq!(observation.get_precip_type(), Ok(PrecipitationType::None));
        assert_eq!(observation.get_lightning_avg_distance(), Ok(0.0));
        assert_eq!(observation.get_lightning_strike_count(), Ok(0.0));
        assert_eq!(observation.get_battery_voltage(), Ok(2.410));
        assert_eq!(observation.get_report_interval(), Ok(1.0));
    }

    #[test]
    fn get_data_from_devicestatusevent() {
        let device = DeviceStatusEvent {
            serial_number: "AR-00004049".to_string(),
            r#type: "device_status".to_string(),
            hub_sn: "HB-00000001".to_string(),
            timestamp: 1510855923,
            uptime: 2189,
            voltage: 3.50,
            firmware_revision: 17,
            rssi: -17,
            hub_rssi: -87,
            sensor_status: 0,
            debug: 0,
        };

        assert_eq!(device.get_serial_number(), "AR-00004049");
        assert_eq!(device.get_hub_sn(), "HB-00000001");
        assert_eq!(device.get_timestamp(), 1510855923);
        assert_eq!(device.get_uptime(), 2189);
        assert_eq!(device.get_battery_voltage(), 3.50);
        assert_eq!(device.get_firmware_revision(), 17);
        assert_eq!(device.get_rssi(), -17);
        assert_eq!(device.get_hub_rssi(), -87);
        assert!(!device.debugging_enabled());
    }

    #[test]
    fn get_data_from_hubstatusevent() {
        let hub_status = HubStatusEvent {
            serial_number: "HB-00000001".to_string(),
            r#type: "hub_status".to_string(),
            firmware_revision: "35".to_string(),
            uptime: 1670133,
            rssi: -62,
            timestamp: 1495724691,
            reset_flags: "BOR,PIN,POR".to_string(),
            seq: 48,
            fs: Some(vec![1, 0, 15675411, 524288]),
            radio_stats: vec![2, 1, 0, 3, 2839],
            mqtt_stats: vec![1, 0],
        };

        assert_eq!(hub_status.get_serial_number(), "HB-00000001");
        assert_eq!(hub_status.get_firmware_revision(), "35");
        assert_eq!(hub_status.get_uptime(), 1670133);
        assert_eq!(hub_status.get_rssi(), -62);
        assert_eq!(hub_status.get_timestamp(), 1495724691);
        assert_eq!(hub_status.get_reset_flags(), "BOR,PIN,POR");
        assert_eq!(hub_status.get_radio_version(), 2);
        assert_eq!(hub_status.get_radio_reboot_count(), 1);
        assert_eq!(hub_status.get_radio_status(), RadioStatus::RadioActive);
        assert_eq!(hub_status.get_radio_network_id(), 2839);
    }
}
