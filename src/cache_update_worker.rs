use crate::{
    config::CacheConfig,
    types::{Cache, Pilot, SerialNumber, Violation},
};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub capture: Capture,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Capture {
    #[serde(rename = "drone")]
    pub drones: Vec<Drone>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Drone {
    pub serial_number: SerialNumber,
    pub position_x: f64,
    pub position_y: f64,
}

impl Drone {
    pub fn is_in_no_drone_zone(&self, config: &CacheConfig) -> bool {
        self.distance_to_nest(config) < config.ndz_radius
    }

    pub fn distance_to_nest(&self, config: &CacheConfig) -> f64 {
        ((self.position_x - config.ndz_center_x).powf(2.0)
            + (self.position_y - config.ndz_center_y).powf(2.0))
        .sqrt()
    }
}

pub async fn run_worker(
    cache: Cache,
    config: &CacheConfig,
) -> anyhow::Result<()> {
    let mut interval =
        tokio::time::interval(Duration::from_secs(config.update_interval_secs));
    loop {
        interval.tick().await;

        let body = reqwest::get(format!("{}/drones", config.server_url))
            .await?
            .text()
            .await?;
        let report: Report = quick_xml::de::from_str(&body)?;
        let drones = report.capture.drones;

        futures_util::future::try_join_all(
            drones
                .into_iter()
                .map(|drone| handle_violations(drone, cache.clone(), config)),
        )
        .await?;
    }
}

pub async fn handle_violations(
    drone: Drone,
    cache: Cache,
    config: &CacheConfig,
) -> anyhow::Result<()> {
    let mut cache_lock = cache.write().await;
    let ttl = Duration::from_secs(config.violation_ttl_secs);
    // If drone has previously violated NDZ (i.e., is already cached), update TTL and distance if necessary
    if let Some(violation) = cache_lock.remove(&drone.serial_number) {
        let violation = Violation {
            distance: f64::min(
                drone.distance_to_nest(config),
                violation.distance,
            ),
            ..violation
        };
        cache_lock.insert(drone.serial_number, violation, ttl);
    // If drone has not previously violated NDZ but is currently in violation of NDZ
    } else if drone.is_in_no_drone_zone(config) {
        drop(cache_lock);
        let url =
            format!("{}/pilots/{}", config.server_url, &drone.serial_number);

        if let Ok(response) = reqwest::get(&url).await {
            let body = response.text().await?;

            let pilot: Pilot = serde_json::from_str(&body)?;
            let distance = drone.distance_to_nest(config);
            let id = drone.serial_number.clone();
            let violation = Violation {
                pilot,
                distance,
                id,
            };

            let mut cache_lock = cache.write().await;
            cache_lock.insert(drone.serial_number, violation, ttl);
        }
    }
    Ok(())
}
