use isis_ants_api::*;

fn main() {
    let mut ants = AntS::new("/dev/i2c-1", 0x31, 0x32, 4, 10).unwrap();

    ants.configure(KANTSController::Secondary);

    //ants.reset();
    ants.arm();
    //ants.disarm();
    ants.deploy(KANTSAnt::Ant3, true, 10);
    //ants.auto_deploy(0x05);
    //ants.cancel_deploy();
    //ants.get_deploy();
    //println!("Telemetry: {:?}", ants.get_system_telemetry());
    // println!("Deployment Status: {:?}", ants.get_deploy());
    //ants.get_activation_count(KANTSAnt::Ant1);
    //ants.get_activation_time(KANTSAnt::Ant1);

    println!("End of test");
}
