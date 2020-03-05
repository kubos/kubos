use failure::{bail, Error};
use getopts::Options;
use kubos_app::*;
use log::*;
use serde_derive::Serialize;
use serde_json;
use std::net::UdpSocket;
use std::time::Duration;

#[derive(Serialize, Clone, Debug, Default)]
pub struct Beacon {
    mem: Option<u64>,
    la1: Option<f64>,
    la5: Option<f64>,
    la15: Option<f64>,
}

fn main() -> Result<(), Error> {
    logging_setup!("rust-beacon-app")?;
    let mut opts = Options::new();

    // Standard app args:
    // This option will be processed by the system-api crate when a service query is run
    opts.optflagopt(
        "c",
        "config",
        "System config file which should be used",
        "CONFIG",
    );
    opts.optflag("h", "help", "Print this help menu");

    let beacon = get_beacon_information()?;
    info!("Current beacon info: {:?}", beacon);
    send_beacon(&beacon)?;

    Ok(())
}

fn get_beacon_information() -> Result<Beacon, Error> {
    let monitor_service = ServiceConfig::new("monitor-service")?;

    let request = "{ memInfo { available }, loadAverage { one, five, fifteen } }";
    let response = match query(&monitor_service, request, Some(Duration::from_secs(1))) {
        Ok(msg) => msg,
        Err(err) => {
            error!("Monitor service query failed: {}", err);
            bail!("Monitor service query failed: {}", err);
        }
    };

    let memory = response.get("memInfo").and_then(|msg| msg.get("available"));
    let load_average = response.get("loadAverage");
    let la1 = load_average.and_then(|msg| msg.get("one"));
    let la5 = load_average.and_then(|msg| msg.get("five"));
    let la15 = load_average.and_then(|msg| msg.get("fifteen"));

    let mut beacon: Beacon = Default::default();
    beacon.mem = memory.and_then(|v| v.as_u64());
    beacon.la1 = la1.and_then(|v| v.as_f64());
    beacon.la5 = la5.and_then(|v| v.as_f64());
    beacon.la15 = la15.and_then(|v| v.as_f64());

    Ok(beacon)
}

fn send_beacon(beacon: &Beacon) -> Result<(), Error> {
    // Get the receiver IP address. FIXME: this should be the comms service
    let config = ServiceConfig::new("rust-beacon-app")?;
    let host = config
        .hosturl()
        .ok_or_else(|| failure::format_err!("Unable to fetch addr for beacon downlink host"))?;
    let mut host_parts = host.split(':').map(|val| val.to_owned());
    let beacon_ip = host_parts.next().unwrap_or_else(|| {
        error!("Failed to lookup beacon host IP. Using default 0.0.0.0");
        "0.0.0.0".to_owned()
    });
    let beacon_port = host_parts.next().unwrap_or_else(|| {
        error!("Failed to lookup beacon host port. Using default 8500");
        "8500".to_owned()
    });
    let downlink = format!("{}:{}", beacon_ip, beacon_port);

    // Bind a socket for the host IP
    let local_socket = UdpSocket::bind("0.0.0.0:0")?;

    // Send our distress message
    match local_socket.send_to(serde_json::to_string(beacon)?.as_ref(), downlink) {
        Ok(_) => debug!("Sent beacon"),
        Err(err) => error!("Failed to send beacon: {:?}", err),
    }

    Ok(())
}
