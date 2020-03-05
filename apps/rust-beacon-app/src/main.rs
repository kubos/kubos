use failure::{bail, Error};
use getopts::Options;
use kubos_app::*;
use log::*;
use serde_derive::Serialize;
use serde_json;
use std::net::UdpSocket;
use std::time::Duration;

#[derive(Serialize, Clone, Debug)]
pub struct Beacon {
    mem: Option<u64>,
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

    // Get the amount of memory currently available on the OBC
    let mem = get_available_memory()?;
    info!("Current memory value: {:?}", mem);

    send_beacon(&Beacon { mem })?;

    Ok(())
}

fn get_available_memory() -> Result<Option<u64>, Error> {
    let monitor_service = ServiceConfig::new("monitor-service")?;

    let request = "{memInfo{available}}";
    let response = match query(&monitor_service, request, Some(Duration::from_secs(1))) {
        Ok(msg) => msg,
        Err(err) => {
            error!("Monitor service query failed: {}", err);
            bail!("Monitor service query failed: {}", err);
        }
    };

    let memory = response.get("memInfo").and_then(|msg| msg.get("available"));
    if let Some(mem) = memory {
        Ok(mem.as_u64())
    } else {
        Ok(None)
    }
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
