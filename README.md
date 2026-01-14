# rTempest 🫖

[![Crates.io Version][crates-badge]][crates-url]
![CI](https://github.com/rorynugent/rtempest/actions/workflows/ci.yml/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/rtempest
[crates-url]: https://crates.io/crates/rtempest

#### [Changelog](https://github.com/rorynugent/rtempest/blob/main/CHANGELOG.md) | [Docs](https://rorynugent.github.io/rtempest/)

## Overview

Retrieves and parses weather data from a WeatherFlow Tempest station.

Supports Tempest UDP reference [v171](https://weatherflow.github.io/Tempest/api/udp/v171/).

## Features
 - Live notification of weather events
 - Caching of the most recent weather event
 - Get hub information
 - Retrieval of specific station or hub data, e.g. last temperature reading, average wind speed, station battery voltage, etc.
 - Subscribe to events for specific stations
 - Asynchronous implementation using [Tokio](https://tokio.rs/)

 ## Examples
 - Receive UDP data as structured event data via a channel
 - Receive UDP data as structured event data via a channel as well as caching of most recent data points
 - Receive UDP data as structured event data via a channel for specific device serial numbers (subscribe)