//! Crate for getting weather data from a [WeatherFlow Tempest](https://tempest.earth/tempest-home-weather-system/) station
//!
//! ## Getting Started
//! Currently this crate can be used to retrieve weather data from a
//! WeatherFlow Tempest station over your local LAN. It does so by
//! retrieve multicast UDP packets from the station, parsing them,
//! and deserializing them.
//!
//! Check out the examples provided within the crate on how to get started.
//! At the moment you can receive all live weather events, subscribe to
//! specific weather hubs or stations based on serial number, or cache
//! incoming weather data so its easier to retrieve specific data points,
//! like air temperature, station pressure, etc.
//!
//! ## References
//! - [`WeatherFlow UDP`](https://weatherflow.github.io/Tempest/api/udp/v171/)

pub mod data;
pub mod mock;
pub mod test_common;
pub mod udp;
