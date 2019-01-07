#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate log4rs_syslog;

use comms_service::*;
use failure::Error;
use serial;
use std::cell::RefCell;
use std::io::prelude::*;
use serial::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

const CONFIG_PATH: &'static str = "comms.toml";
const MAX_READ: usize = 48;
const TIMEOUT: Duration = Duration::from_millis(100);

// Return type for the ethernet service.
type ServiceResult<T> = Result<T, Error>;

fn log_init() -> ServiceResult<()> {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs_syslog::SyslogAppender;
    // Use custom PatternEncoder to avoid duplicate timestamps in logs.
    let syslog_encoder = Box::new(PatternEncoder::new("{m}"));
    // Set up logging which will be routed to syslog for processing
    let syslog = Box::new(
        SyslogAppender::builder()
            .encoder(syslog_encoder)
            .openlog(
                "uart-comms-service",
                log4rs_syslog::LogOption::LOG_PID | log4rs_syslog::LogOption::LOG_CONS,
                log4rs_syslog::Facility::User,
            )
            .build(),
    );

    // Set up logging which will be routed to stdout
    let stdout = Box::new(ConsoleAppender::builder().build());

    // Combine the loggers into one master config
    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("syslog", syslog))
        .appender(log4rs::config::Appender::builder().build("stdout", stdout))
        .build(
            log4rs::config::Root::builder()
                .appender("syslog")
                .appender("stdout")
                // Set the minimum logging level to record
                .build(log::LevelFilter::Debug),
        )?;

    // Start the logger
    log4rs::init_config(config)?;
    
    Ok(())
}

fn serial_init(bus: &str) -> RefCell<serial::SystemPort> {
    let settings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };
    
    let mut port = serial::open(bus).unwrap();

    port.configure(&settings).unwrap();
    port.set_timeout(TIMEOUT).unwrap();
    
    RefCell::new(port)
}

fn read(conn: Arc<Mutex<RefCell<serial::SystemPort>>>) -> CommsResult<Vec<u8>> {
    // Take the stream connection mutex
    // If the lock() call fails, it means that a different thread poisoned
    // the mutex. We want to maintain our ability to read messages from the
    // device for as long as possible, so we'll go ahead and just ignore the
    // poisoned status. Ideally, the master thread will have detected whatever
    // error caused the problem and will take error handling measures.
    // TODO: decide if this is still the thing to do
    loop {
        {
            let conn = conn.lock().unwrap_or_else(|err| err.into_inner());
            let mut conn = conn.try_borrow_mut().unwrap();
            
            
            let mut packet = vec![];
        
            loop {
                let mut buffer: Vec<u8> = vec![0; MAX_READ];
                match conn.read(buffer.as_mut_slice()) {
                    Ok(num) => {
                        buffer.resize(num, 0);
                        packet.append(&mut buffer);
        
                        if num < MAX_READ {
                            debug!("Radio read {} bytes", packet.len());
    
                            return Ok(packet);
                        }
                    },
                    Err(ref err) => match err.kind() {
                        ::std::io::ErrorKind::TimedOut => break,
                        _ => panic!()
                    }
                };
            }
            
        }
        
        thread::sleep(Duration::from_millis(10));
    }

}

fn write(conn: Arc<Mutex<RefCell<serial::SystemPort>>>, msg: &[u8]) -> CommsResult<()> {
    debug!("Radio write: {:?}", msg);

    let conn = match conn.lock() {
        Ok(val) => val,
        Err(e) => bail!("Failed to take mutex: {:?}", e)    
    };
    let mut conn = match conn.try_borrow_mut() {
        Ok(val) => val,
        Err(e) => bail!("Failed to borrow mut: {:?}", e)
    };
    
    match conn.write(msg) {
        Ok(num) => {
            debug!("Radio wrote {} bytes", num);
            Ok(())
        },
        Err(err) => {error!("Write failed: {:?}", err); bail!(err)}
    }
}

fn main() -> ServiceResult<()> {
    
    log_init()?;
    
    let config = CommsConfig::new("uart-comms-service", CONFIG_PATH.to_string());

    let raw_conn = serial_init("/dev/ttyS2");
    let conn = Arc::new(Mutex::new(raw_conn));
    
    // Set up the comms configuration
    // In this instance, reading and writing are done over the same connection,
    // so we'll just clone the UART port connection
    let read_conn = conn.clone();
    let write_conn = conn;
    
    let control = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        read_conn,
        write_conn,
        config
    );
    
    /*
    let config = CommsControlBlock {
        read: Some(Arc::new(read)),
        write: vec![Arc::new(write)],
        read_conn,
        write_conn,
        handler_port_min: 9000,
        handler_port_max: 9100,
        timeout: 500,
        ground_ip: Ipv4Addr::new(192, 168, 8, 40),
        satellite_ip: Ipv4Addr::new(0, 0, 0, 0),
        downlink_ports: None,
        ground_port: Some(9001)
    };
    */
    
    // Start the comms service thread
    CommsService::start(control, Arc::new(Mutex::new(CommsTelemetry::default()))).unwrap();
    
    // Start the GraphQL service
    loop {}
    
    Ok(())
}
