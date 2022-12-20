use crate::types::{Cache, Pilot, SerialNumber, Violation};
use serde::Deserialize;
use std::time::Duration;

const NO_DRONE_ZONE_CENTER: f64 = 250000.0;
const NO_DRONE_ZONE_RADIUS: f64 = 100000.0;
const TTL: Duration = Duration::from_secs(60 * 10);
const SERVER_URL: &str = "https://assignments.reaktor.com/birdnest";
const UPDATE_PERIOD: Duration = Duration::from_secs(2);

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
    pub position_y: f64,
    pub position_x: f64,
}

impl Drone {
    pub fn is_in_no_drone_zone(&self) -> bool {
        self.distance_to_nest() < NO_DRONE_ZONE_RADIUS
    }

    pub fn distance_to_nest(&self) -> f64 {
        ((self.position_x - NO_DRONE_ZONE_CENTER).powf(2.0)
            + (self.position_y - NO_DRONE_ZONE_CENTER).powf(2.0))
        .sqrt()
    }
}

pub async fn update_cache(cache: Cache) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(UPDATE_PERIOD);
    loop {
        interval.tick().await;

        let body = reqwest::get(format!("{SERVER_URL}/drones"))
            .await?
            .text()
            .await?;
        let report: Report = quick_xml::de::from_str(&body)?;
        let drones = report.capture.drones;

        futures_util::future::try_join_all(
            drones
                .into_iter()
                .map(|drone| handle_violations(drone, cache.clone())),
        )
        .await?;
    }
}

pub async fn handle_violations(
    drone: Drone,
    cache: Cache,
) -> anyhow::Result<()> {
    let mut cache_lock = cache.write().await;
    // If drone has previously violated NDZ (i.e., is already cached), update TTL and distance if necessary
    if let Some(violation) = cache_lock.remove(&drone.serial_number) {
        let violation = Violation {
            distance: f64::min(drone.distance_to_nest(), violation.distance),
            ..violation
        };
        cache_lock.insert(drone.serial_number, violation, TTL);
    // If drone has not previously violated NDZ but is currently in violation of NDZ
    } else if drone.is_in_no_drone_zone() {
        drop(cache_lock);
        let url = format!("{SERVER_URL}/pilots/{}", &drone.serial_number);

        if let Ok(response) = reqwest::get(&url).await {
            let body = response.text().await?;

            let pilot: Pilot = serde_json::from_str(&body)?;
            let distance = drone.distance_to_nest();
            let id = drone.serial_number.clone();
            let violation = Violation {
                pilot,
                distance,
                id,
            };

            let mut cache_lock = cache.write().await;
            cache_lock.insert(drone.serial_number, violation, TTL);
        }
    }
    Ok(())
}
