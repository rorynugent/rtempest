//! Primary interface for WeatherFlow Tempest weather data over UDP

use crate::data::*;
use log::trace;
use serde_json::{Error, Value};
use std::net::Ipv4Addr;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, mpsc::Receiver};

/// Default Tempest UDP port
const DEFAULT_PORT: u16 = 50222;

/// Default UDP buffer sized used in this crate
const DEFAULT_BUFFER_SIZE: usize = 4096;

/// Inner data structure of `Tempest` containing cached hubs and stations
#[derive(Clone)]
pub struct Inner {
    hubs_cached: Vec<Hub>,
    stations_cached: Vec<Station>,
}

impl Inner {
    fn new() -> Self {
        Inner {
            hubs_cached: Vec::new(),
            stations_cached: Vec::new(),
        }
    }
}

/// Tempest hub and station interface
#[derive(Clone)]
pub struct Tempest {
    /// Thread safe receiver for UDP socket data
    recv: Arc<UdpSocket>,
    /// Thread safe read-write lock on inner data (cached data)
    inner: Arc<RwLock<Inner>>,
}

impl Tempest {
    async fn bind(ip: Option<Ipv4Addr>, port: Option<u16>) -> Self {
        let ip = ip.unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
        let port = port.unwrap_or(DEFAULT_PORT);

        let sock = UdpSocket::bind(format!("{ip}:{port}"))
            .await
            .expect("Error binding to socket");
        let arc_socket = Arc::new(sock);

        Self {
            recv: arc_socket,
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    /// Grabs the shared read lock of the inner
    fn read_inner(&self) -> RwLockReadGuard<Inner> {
        self.inner.read().expect("Unable to acquire read lock")
    }

    /// Grabs the exclusive write lock of the inner
    fn write_inner(&self) -> RwLockWriteGuard<Inner> {
        self.inner.write().expect("Unable to acquire write lock")
    }

    /// Returns a count of the number of stations cached
    pub fn station_count(&self) -> usize {
        self.read_inner().stations_cached.len()
    }

    /// Returns a count of the number of hubs cached
    pub fn hub_count(&self) -> usize {
        self.read_inner().hubs_cached.len()
    }

    /// Insert or replace the provided hub into the hub cache
    fn hub_upsert(&mut self, hub_data: Hub) {
        let index = self.get_hub_index(&hub_data.serial_number);

        if let Some(index) = index {
            trace!("Removing existing hub record");
            self.write_inner().hubs_cached.swap_remove(index);
        }

        self.write_inner().hubs_cached.push(hub_data);
    }

    /// Cache a ObservationEvent into the station cache
    fn cache_station_observation(&mut self, observation: ObservationEvent) {
        let index = self.get_station_index(&observation.get_serial_number());

        if let Some(index) = index {
            // general station info
            self.write_inner().stations_cached[index].firmware_revision =
                Some(observation.get_firmware_revision());

            self.write_inner().stations_cached[index].hub_sn = observation.get_hub_sn();

            self.write_inner().stations_cached[index].serial_number =
                observation.get_serial_number();

            self.write_inner().stations_cached[index].battery_voltage =
                observation.get_battery_voltage().ok();

            // common weather data
            self.write_inner().stations_cached[index].station_pressure =
                observation.get_station_pressure().ok();

            self.write_inner().stations_cached[index].air_temperature =
                observation.get_air_temperature().ok();

            self.write_inner().stations_cached[index].relative_humidity = observation.get_rh().ok();

            self.write_inner().stations_cached[index].lightning_strike_count =
                observation.get_lightning_strike_count().ok();

            self.write_inner().stations_cached[index].lightning_strike_avg_distance =
                observation.get_lightning_avg_distance().ok();

            self.write_inner().stations_cached[index].illuminance =
                observation.get_illuminance().ok();

            self.write_inner().stations_cached[index].uv = observation.get_uv().ok();

            self.write_inner().stations_cached[index].rain_amount_prev_minute =
                observation.get_rain_amount_prev_min().ok();

            self.write_inner().stations_cached[index].wind_lull = observation.get_wind_lull().ok();

            self.write_inner().stations_cached[index].wind_avg = observation.get_wind_avg().ok();

            self.write_inner().stations_cached[index].wind_gust = observation.get_wind_gust().ok();

            self.write_inner().stations_cached[index].wind_direction =
                observation.get_wind_direction().ok();

            self.write_inner().stations_cached[index].solar_radiation =
                observation.get_solar_radiation().ok();

            self.write_inner().stations_cached[index].precipitation_type =
                observation.get_precip_type().ok();

            // cache event
            self.write_inner().stations_cached[index]
                .observation
                .replace(observation);
        } else {
            self.write_inner().stations_cached.push(observation.into());
        }
    }

    /// Cache a RapidWindEvent into the station cache
    fn cache_station_wind_event(&mut self, event: RapidWindEvent) {
        let index = self.get_station_index(&event.get_serial_number());

        if let Some(index) = index {
            self.write_inner().stations_cached[index]
                .wind_event
                .replace(event);
        } else {
            self.write_inner().stations_cached.push(event.into());
        }
    }

    /// Cache a RainStartEvent into the station cache
    fn cache_station_rain_event(&mut self, event: RainStartEvent) {
        let index = self.get_station_index(&event.get_serial_number());

        if let Some(index) = index {
            self.write_inner().stations_cached[index]
                .rain_event
                .replace(event);
        } else {
            self.write_inner().stations_cached.push(event.into());
        }
    }

    /// Cache a LightningStrikeEvent into the station cache
    fn cache_station_lightning_event(&mut self, event: LightningStrikeEvent) {
        let index = self.get_station_index(&event.get_serial_number());

        if let Some(index) = index {
            self.write_inner().stations_cached[index]
                .lightning_event
                .replace(event);
        } else {
            self.write_inner().stations_cached.push(event.into());
        }
    }

    /// Cache a ObservationAirEvent into the station cache
    fn cache_station_air_event(&mut self, event: ObservationAirEvent) {
        let index = self.get_station_index(&event.get_serial_number());

        if let Some(index) = index {
            // general station info
            self.write_inner().stations_cached[index].serial_number = event.get_serial_number();

            self.write_inner().stations_cached[index].hub_sn = event.get_hub_sn();

            self.write_inner().stations_cached[index].firmware_revision =
                Some(event.get_firmware_revision());

            self.write_inner().stations_cached[index].battery_voltage =
                event.get_battery_voltage().ok();

            // common weather data
            self.write_inner().stations_cached[index].station_pressure =
                event.get_station_pressure().ok();

            self.write_inner().stations_cached[index].air_temperature =
                event.get_air_temperature().ok();

            self.write_inner().stations_cached[index].relative_humidity =
                event.get_relative_humidity().ok();

            self.write_inner().stations_cached[index].lightning_strike_count =
                event.get_lightning_count().ok();

            self.write_inner().stations_cached[index].lightning_strike_avg_distance =
                event.get_lightning_avg_distance().ok();

            // cache event
            self.write_inner().stations_cached[index]
                .air_event
                .replace(event);
        } else {
            self.write_inner().stations_cached.push(event.into());
        }
    }

    /// Cache a ObservationSkyEvent into the station cache
    fn cache_station_sky_event(&mut self, event: ObservationSkyEvent) {
        let index = self.get_station_index(&event.get_serial_number());

        if let Some(index) = index {
            // general station info
            self.write_inner().stations_cached[index].serial_number = event.get_serial_number();

            self.write_inner().stations_cached[index].hub_sn = event.get_hub_sn();

            self.write_inner().stations_cached[index].firmware_revision =
                Some(event.get_firmware_revision());

            self.write_inner().stations_cached[index].battery_voltage =
                event.get_battery_voltage().unwrap_or_default();

            // common weather data
            self.write_inner().stations_cached[index].illuminance =
                event.get_illuminance().unwrap_or_default();

            self.write_inner().stations_cached[index].uv = event.get_uv().unwrap_or_default();

            self.write_inner().stations_cached[index].rain_amount_prev_minute =
                event.get_rain_prev_min().unwrap_or_default();

            self.write_inner().stations_cached[index].wind_lull =
                event.get_wind_lull().unwrap_or_default();

            self.write_inner().stations_cached[index].wind_avg =
                event.get_wind_avg().unwrap_or_default();

            self.write_inner().stations_cached[index].wind_gust =
                event.get_wind_gust().unwrap_or_default();

            self.write_inner().stations_cached[index].wind_direction =
                event.get_wind_direction().unwrap_or_default();

            self.write_inner().stations_cached[index].solar_radiation =
                event.get_solar_radiation().unwrap_or_default();

            self.write_inner().stations_cached[index].precipitation_type =
                event.get_precip_type().ok();

            // cache event
            self.write_inner().stations_cached[index]
                .sky_event
                .replace(event);
        } else {
            self.write_inner().stations_cached.push(event.into());
        }
    }

    /// Cache a DeviceStatusEvent into the station cache
    fn cache_station_device_status(&mut self, event: DeviceStatusEvent) {
        let index = self.get_station_index(&event.get_serial_number());

        if let Some(index) = index {
            // general station info
            self.write_inner().stations_cached[index].serial_number = event.get_serial_number();

            self.write_inner().stations_cached[index].hub_sn = event.get_hub_sn();

            self.write_inner().stations_cached[index].firmware_revision =
                Some(event.get_firmware_revision());

            self.write_inner().stations_cached[index].battery_voltage =
                Some(event.get_battery_voltage());

            // cache event
            self.write_inner().stations_cached[index]
                .device_status
                .replace(event);
        } else {
            self.write_inner().stations_cached.push(event.into());
        }
    }

    /// Retrieve a hub from the cache based on the provided serial number
    ///
    /// Returns Some(Hub) if the hub is present in the cache, otherwise None
    pub fn get_hub_by_sn(&self, serial_number: &str) -> Option<Hub> {
        for hub in self.read_inner().hubs_cached.iter() {
            if hub.serial_number == serial_number {
                return Some(hub.clone());
            }
        }

        None
    }

    /// Retrieve a hub from the cache associated with the provided station
    ///
    /// If the hub is in the cache then Some(Hub) is returned, otherwise None if not present
    pub fn get_hub_from_station(&self, station: Station) -> Option<Hub> {
        self.get_hub_by_sn(&station.hub_sn)
    }

    /// Get the vector index of a cached hub based on the provided hub serial number
    ///
    /// If station is in the cache then Some(index) is returned, otherwise None if not present.
    fn get_hub_index(&self, serial_number: &str) -> Option<usize> {
        for (index, hub) in self.read_inner().hubs_cached.iter().enumerate() {
            if hub.serial_number == serial_number {
                return Some(index);
            }
        }

        None
    }

    /// Get the vector index of a cached station based on the provided hub serial number
    ///
    /// If station is in the cache then Some(index) is returned, otherwise None is not present.
    fn get_station_index(&self, serial_number: &str) -> Option<usize> {
        for (index, station) in self.read_inner().stations_cached.iter().enumerate() {
            if station.serial_number == serial_number {
                return Some(index);
            }
        }

        None
    }

    /// Retrieve a station from the cache based on the provided serial number
    pub fn get_station_by_sn(&self, serial_number: &str) -> Option<Station> {
        for station in self.read_inner().stations_cached.iter() {
            if station.serial_number == serial_number {
                return Some(station.clone());
            }
        }

        None
    }

    /// Retrieve a vector of stations from the cache based on the associated hub's serial number
    pub fn get_stations_by_hub_sn(&self, serial_number: &str) -> Vec<Station> {
        let mut stations: Vec<Station> = Vec::new();

        for station in self.read_inner().stations_cached.iter() {
            if station.hub_sn == serial_number {
                stations.push(station.clone());
            }
        }

        stations
    }

    /// Retrieve the most recent battery voltage of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_battery_voltage(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.battery_voltage)?
    }

    /// Retrieve the most recent wind speed lull of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_wind_lull(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.wind_lull)?
    }

    /// Retrieve the most recent wind speed average of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_wind_avg(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.wind_avg)?
    }

    /// Retrieve the most recent wind speed gust of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_wind_gust(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.wind_gust)?
    }

    /// Retrieve the most recent wind direction of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_wind_direction(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.wind_direction)?
    }

    /// Retrieve the most recent wind speed of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_wind_speed(&self, serial_number: &str) -> Option<f32> {
        Some(
            self.get_station_by_sn(serial_number)?
                .wind_event?
                .get_wind_speed_mps(),
        )
    }

    /// Retrieve the most recent station pressure (MB, millibars) of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_station_pressure(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.station_pressure)?
    }

    /// Retrieve the most recent air temperature (C, celsius) of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_air_temperature(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.air_temperature)?
    }

    /// Retrieve the most recent illuminance (lux) of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_lux(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.illuminance)?
    }

    /// Retrieve the most recent UV Index of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_uv(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.uv)?
    }

    /// Retrieve the most recent solar radiation (W/m^2) of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_solar_radiation(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.solar_radiation)?
    }

    /// Retrieve the most recent measurement of rain (mm) in the previous minute of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_rain_prev_min(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.rain_amount_prev_minute)?
    }

    /// Retrieve the timestamp of the previous rain start from a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_prev_rain_start(&self, serial_number: &str) -> Option<u64> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.prev_rain_timestamp)?
    }

    /// Retrieve the most recent precipitation type of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_precipitation_type(&self, serial_number: &str) -> Option<PrecipitationType> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.precipitation_type)?
    }

    /// Retrieve the most recent measurement of lightning strike average distance (km) of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_lightning_avg_distance(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.lightning_strike_avg_distance)?
    }

    /// Retrieve the most recent lightning strike count of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_lightning_count(&self, serial_number: &str) -> Option<f32> {
        self.get_station_by_sn(serial_number)
            .map(|station| station.lightning_strike_count)?
    }

    /// Retrieve the most recent lightning strike timestamp of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_lightning_timestamp(&self, serial_number: &str) -> Option<u64> {
        Some(
            self.get_station_by_sn(serial_number)?
                .lightning_event?
                .get_timestamp(),
        )
    }

    /// Retrieve the most recent lightning strike distance (km, kilometers) of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_lightning_distance(&self, serial_number: &str) -> Option<u64> {
        Some(
            self.get_station_by_sn(serial_number)?
                .lightning_event?
                .get_strike_distance(),
        )
    }

    /// Retrieve the most recent lightning strike energy (J, joules) of a cached station based on the provided station's serial number
    ///
    /// Returns the value as a Some(..) if present otherwise returns a None
    pub fn get_lightning_energy(&self, serial_number: &str) -> Option<u64> {
        Some(
            self.get_station_by_sn(serial_number)?
                .lightning_event?
                .get_strike_energy(),
        )
    }

    /// Listen to UDP packets sent from the WeatherFlow Tempest hub
    ///
    /// Returns a Tokio receiver containing a weather event as an `EventType`.
    /// The `Tempest` instance is disregarded in this use case.
    pub async fn listen_udp() -> Receiver<EventType> {
        let (_, rx) = Tempest::listen_udp_internal(None, None, false, None).await;
        rx
    }

    /// Listen to UDP packets sent from the WeatherFlow Tempest hub and cache data about hubs and stations reporting events
    ///
    /// Returns a `Tempest` instance along with a Tokio receiver containining a weather event as an `EventType`
    pub async fn listen_udp_with_cache() -> (Tempest, Receiver<EventType>) {
        Tempest::listen_udp_internal(None, None, true, None).await
    }

    /// Listen to UDP packets sent from the WeatherFlow Tempest hub and only share events that match the provided serial number.
    ///
    /// Returns a Tokio receiver accepting weather events as an `EventType`.
    /// The `Tempest` instance is disregarded in this use case.
    pub async fn listen_udp_subscribe(station_filter: Vec<&str>) -> Receiver<EventType> {
        let station_filter = station_filter
            .iter()
            .map(|&station| station.to_string())
            .collect();

        let (_, rx) = Tempest::listen_udp_internal(None, None, false, Some(station_filter)).await;
        rx
    }

    /// Internal function used for parsing UDP packets containing JSON weather data.
    ///
    /// When a weather event is received, a few things can happen depending on the parameters passed into this function.
    ///
    /// If `caching` is set to true then the weather event will be saved to cache.
    ///
    /// If `station_filter` is Some(..) and contains station serial numbers then it will only send the weather event
    /// back over the mpsc channel if the weather event's serial number matches the provided serial number.
    /// This acts like a form of filtering.
    ///
    /// This function returns both an instance of `Tempest` for further weather data retrieval (air temperature, wind, etc)
    /// and `rx` is an mpsc receiver for accepting weather event data as it arrives.
    async fn listen_udp_internal(
        address: Option<Ipv4Addr>,
        port: Option<u16>,
        caching: bool,
        station_filter: Option<Vec<String>>,
    ) -> (Tempest, Receiver<EventType>) {
        let mut tempest = Tempest::bind(address, port).await;
        let (tx, rx) = mpsc::channel(16);

        let tempest_clone: Tempest = tempest.clone();

        tokio::spawn(async move {
            loop {
                let mut recv_buffer: Vec<u8> = vec![0; DEFAULT_BUFFER_SIZE];

                // receive udp packet into buffer
                let len = match tempest.recv.recv_from(&mut recv_buffer).await {
                    Ok((len, _addr)) => len,
                    Err(e) => {
                        eprintln!("Failed to receive UDP packet: {e}");
                        continue;
                    }
                };

                // deserialize buffer contents into json value
                let json: Value = match serde_json::from_slice(&recv_buffer[0..len]) {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!(
                            "Failed to deserialize packet contents into serde JSON value: {e}"
                        );
                        continue;
                    }
                };

                match json["type"].as_str() {
                    // Station observation event
                    Some("obs_st") => {
                        let evt: Result<ObservationEvent, Error> = serde_json::from_value(json);

                        match evt {
                            Ok(event) => {
                                if caching {
                                    tempest.cache_station_observation(event.clone());
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ = tx
                                        .send(EventType::Observation(event))
                                        .await
                                        .inspect_err(|e| eprintln!("Unable to send {e:?}"));
                                }
                            }
                            Err(e) => eprintln!("Error : {e}"),
                        }
                    }
                    // Air observation event
                    Some("obs_air") => {
                        let evt: Result<ObservationAirEvent, Error> = serde_json::from_value(json);

                        match evt {
                            Ok(event) => {
                                if caching {
                                    tempest.cache_station_air_event(event.clone());
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ = tx.send(EventType::Air(event)).await.inspect_err(|e| {
                                        eprintln!("Unable to send {e:?}");
                                    });
                                }
                            }
                            Err(e) => eprintln!("Error : {e}"),
                        }
                    }
                    // Sky observation event
                    Some("obs_sky") => {
                        println!("Converting JSON to serde value");
                        let evt: Result<ObservationSkyEvent, Error> = serde_json::from_value(json);

                        println!("Converted");

                        match evt {
                            Ok(event) => {
                                if caching {
                                    println!("Caching");
                                    tempest.cache_station_sky_event(event.clone());
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ = tx.send(EventType::Sky(event)).await.inspect_err(|e| {
                                        eprintln!("Unable to send {e:?}");
                                    });
                                }
                            }
                            Err(e) => eprintln!("Error: {e}"),
                        }
                    }
                    // Hub Status Event
                    Some("hub_status") => {
                        let evt: Result<HubStatusEvent, Error> = serde_json::from_value(json);

                        match evt {
                            Ok(event) => {
                                if caching {
                                    tempest.hub_upsert(Hub::from(event.clone()));
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ = tx
                                        .send(EventType::HubStatus(event))
                                        .await
                                        .inspect_err(|e| eprintln!("Unable to send {e:?}"));
                                }
                            }
                            Err(e) => eprintln!("Error : {e}"),
                        }
                    }
                    //  Rapid wind event
                    Some("rapid_wind") => {
                        let evt: Result<RapidWindEvent, Error> = serde_json::from_value(json);

                        match evt {
                            Ok(event) => {
                                if caching {
                                    tempest.cache_station_wind_event(event.clone());
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ = tx.send(EventType::RapidWind(event)).await.inspect_err(
                                        |e| {
                                            eprintln!("Unable to send {e:?}");
                                        },
                                    );
                                }
                            }
                            Err(e) => eprintln!("Error : {e}"),
                        }
                    }
                    // Precipitation event
                    Some("evt_precip") => {
                        let evt: Result<RainStartEvent, Error> = serde_json::from_value(json);

                        match evt {
                            Ok(event) => {
                                if caching {
                                    tempest.cache_station_rain_event(event.clone());
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ =
                                        tx.send(EventType::Rain(event)).await.inspect_err(|e| {
                                            eprintln!("Unable to send {e:?}");
                                        });
                                }
                            }
                            Err(e) => eprintln!("Error : {e}"),
                        }
                    }
                    // Lightning strike event
                    Some("evt_strike") => {
                        let evt: Result<LightningStrikeEvent, Error> = serde_json::from_value(json);

                        match evt {
                            Ok(event) => {
                                if caching {
                                    tempest.cache_station_lightning_event(event.clone());
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ = tx.send(EventType::Lightning(event)).await.inspect_err(
                                        |e| {
                                            eprintln!("Unable to send {e:?}");
                                        },
                                    );
                                }
                            }
                            Err(e) => eprintln!("Error : {e}"),
                        }
                    }
                    // Device status event
                    Some("device_status") => {
                        let evt: Result<DeviceStatusEvent, Error> = serde_json::from_value(json);

                        match evt {
                            Ok(event) => {
                                if caching {
                                    tempest.cache_station_device_status(event.clone());
                                }

                                // send event if no serial number provided or on a match
                                if station_filter.clone().map_or(true, |stations| {
                                    stations.contains(&event.get_serial_number())
                                }) {
                                    let _ = tx
                                        .send(EventType::DeviceStatus(event))
                                        .await
                                        .inspect_err(|e| {
                                            eprintln!("Unable to send {e:?}");
                                        });
                                }
                            }
                            Err(e) => eprintln!("Error : {e}"),
                        }
                    }
                    _ => {
                        eprintln!("Unknown event type received");
                    }
                };
            }
        });

        (tempest_clone, rx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock::MockSender;
    use crate::test_common::*;

    async fn test_setup(caching: bool) -> (MockSender, Tempest, Receiver<EventType>, u16) {
        let mock = MockSender::bind();

        let (tempest, receiver) =
            Tempest::listen_udp_internal(Some(Ipv4Addr::new(127, 0, 0, 1)), Some(0), caching, None)
                .await;

        let port: u16 = tempest
            .recv
            .local_addr()
            .expect("Unable to retrieve local address of listener")
            .port();

        (mock, tempest, receiver, port)
    }

    #[tokio::test]
    async fn station_count() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();

        // check cached station count while empty
        assert_eq!(0, tempest.station_count());

        // check cached station count after receiving a station observation event
        mock.send(payload.clone(), port);
        receiver.recv().await;
        assert_eq!(1, tempest.station_count());

        // check cached station count after receiving a station observation event
        mock.send(payload.clone(), port);
        receiver.recv().await;
        assert_eq!(1, tempest.station_count());
    }

    #[tokio::test]
    async fn hub_count() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_hub_payload();

        // check cached hub count while empty
        assert_eq!(0, tempest.hub_count());

        // check cached hub count after receiving a hub status event
        mock.send(payload.clone(), port);
        receiver.recv().await;
        assert_eq!(1, tempest.hub_count());

        // check cached hub count after receiving a hub status event for the same hub
        mock.send(payload.clone(), port);
        receiver.recv().await;
        assert_eq!(1, tempest.hub_count());
    }

    #[tokio::test]
    async fn get_hub_by_sn() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_hub_payload();

        mock.send(payload.clone(), port);
        receiver.recv().await;

        // try to retrieve hub with correct SN
        let hub = tempest.get_hub_by_sn("HB-00013030");

        assert!(hub.is_some());

        // try to retrieve hub with incorrect SN
        let hub = tempest.get_hub_by_sn("HB-00000000");

        assert!(hub.is_none())
    }

    #[tokio::test]
    async fn get_hub_from_station() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_hub_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        let station = tempest
            .get_station_by_sn("ST-00000512")
            .expect("Unable to retrieve station");

        let hub = tempest.get_hub_from_station(station);

        assert!(hub.is_some());
    }

    #[tokio::test]
    async fn get_station_by_sn() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        // try to retrieve station with correct SN
        let station = tempest.get_station_by_sn("ST-00000512");

        assert!(station.is_some());

        // try to retrieve hub with incorrect SN
        let station = tempest.get_station_by_sn("ST-00000513");

        assert!(station.is_none())
    }

    #[tokio::test]
    async fn get_stations_by_hub_sn() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        // cache hub
        let payload = get_hub_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        // cache station 1
        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        // cache station 2
        let payload = get_secondary_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        let stations = tempest.get_stations_by_hub_sn("HB-00013030");

        assert_eq!(stations.len(), 2);
    }

    #[tokio::test]
    async fn cache_rain_event_only() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_rain_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_prev_rain_start("ST-00000512"), Some(1493322445));
    }

    #[tokio::test]
    async fn cache_air_event_only() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_air_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_air_temperature("ST-00000512"), Some(10.0));
    }

    #[tokio::test]
    async fn cache_sky_event_only() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_sky_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        println!("Assert");
        assert_eq!(tempest.get_lux("ST-00000512"), Some(9000.0));
    }

    #[tokio::test]
    async fn cache_wind_event_only() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_rapidwind_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_wind_speed("ST-00000512"), Some(2.3));
    }

    #[tokio::test]
    async fn cache_lightning_event_only() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_lightning_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_lightning_energy("ST-00000512"), Some(3848));
    }

    #[tokio::test]
    async fn get_battery_voltage() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_battery_voltage("ST-00000512"), Some(2.410));
    }

    #[tokio::test]
    async fn get_wind_lull() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_wind_lull("ST-00000512"), Some(0.18));
    }

    #[tokio::test]
    async fn get_wind_avg() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_wind_avg("ST-00000512"), Some(0.27));
    }

    #[tokio::test]
    async fn get_wind_gust() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_wind_gust("ST-00000512"), Some(0.27));
    }

    #[tokio::test]
    async fn get_wind_direction() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_wind_direction("ST-00000512"), Some(3.0));
    }

    #[tokio::test]
    async fn get_wind_speed() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        let payload = get_rapidwind_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_wind_speed("ST-00000512"), Some(2.3));
    }

    #[tokio::test]
    async fn get_station_pressure() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_station_pressure("ST-00000512"), Some(1017.57));
    }

    #[tokio::test]
    async fn get_air_temperature() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_air_temperature("ST-00000512"), Some(22.37));
    }

    #[tokio::test]
    async fn get_lux() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_lux("ST-00000512"), Some(328.0));
    }

    #[tokio::test]
    async fn get_uv() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_uv("ST-00000512"), Some(0.03));
    }

    #[tokio::test]
    async fn get_solar_radiation() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_solar_radiation("ST-00000512"), Some(3.0));
    }

    #[tokio::test]
    async fn get_rain_prev_min() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_rain_prev_min("ST-00000512"), Some(0.0));
    }

    #[tokio::test]
    async fn get_precip_type() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(
            tempest.get_precipitation_type("ST-00000512"),
            Some(PrecipitationType::None)
        );
    }

    #[tokio::test]
    async fn get_lightning_avg_distance() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_lightning_avg_distance("ST-00000512"), Some(0.0));
    }

    #[tokio::test]
    async fn get_lightning_count() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_lightning_count("ST-00000512"), Some(0.0));
    }

    #[tokio::test]
    async fn get_lightning_timestamp() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        let payload = get_lightning_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(
            tempest.get_lightning_timestamp("ST-00000512"),
            Some(1493322445)
        );
    }

    #[tokio::test]
    async fn get_lightning_distance() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        let payload = get_lightning_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_lightning_distance("ST-00000512"), Some(27));
    }

    #[tokio::test]
    async fn get_lightning_energy() {
        let (mock, tempest, mut receiver, port) = test_setup(true).await;

        let payload = get_station_observation_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        let payload = get_lightning_payload();
        mock.send(payload.clone(), port);
        receiver.recv().await;

        assert_eq!(tempest.get_lightning_energy("ST-00000512"), Some(3848));
    }
}
