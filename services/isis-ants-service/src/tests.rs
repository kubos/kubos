use double;
//use failure::{Fail, Error};
use std::cell::RefCell;
use isis_ants_api::*;
use model::*;
use objects::*;

macro_rules! mock_new {
    () => (
        MockAntS::new(
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
        Err(AntsError::ConfigError.into()),
    )
    )
}

mock_trait_no_default!(
		MockAntS,
		configure(KANTSController) -> AntSResult<()>,
		reset() -> AntSResult<()>,
		arm() -> AntSResult<()>,
		disarm() -> AntSResult<()>,
		deploy(KANTSAnt, bool, u8) -> AntSResult<()>,
		auto_deploy(u8) -> AntSResult<()>,
		cancel_deploy() -> AntSResult<()>,
		get_deploy() -> AntSResult<DeployStatus>,
		get_uptime() -> AntSResult<u32>,
		get_system_telemetry() -> AntSResult<AntsTelemetry>,
		get_activation_count(KANTSAnt) -> AntSResult<u8>,
		get_activation_time(KANTSAnt) -> AntSResult<u16>,
		watchdog_kick() -> AntSResult<()>,
		watchdog_start() -> AntSResult<()>,
		watchdog_stop() -> AntSResult<()>,
		passthrough(Vec<u8>, Vec<u8>) -> AntSResult<()>
	);

impl IAntS for MockAntS {
    fn new(
        _bus: KI2CNum,
        _primary: u8,
        _secondary: u8,
        _ant_count: u8,
        _timeout: u32,
    ) -> AntSResult<MockAntS> {
        Ok(mock_new!())
    }
    mock_method!(configure(&self, config: KANTSController) -> AntSResult<()>);
    mock_method!(reset(&self) -> AntSResult<()>);
    mock_method!(arm(&self) -> AntSResult<()>);
    mock_method!(disarm(&self) -> AntSResult<()>);
    mock_method!(deploy(&self, antenna: KANTSAnt, force: bool, timeout: u8) -> AntSResult<()>);
    mock_method!(auto_deploy(&self, timeout: u8) -> AntSResult<()>);
    mock_method!(cancel_deploy(&self) -> AntSResult<()>);
    mock_method!(get_deploy(&self) -> AntSResult<DeployStatus>);
    mock_method!(get_uptime(&self) -> AntSResult<u32>);
    mock_method!(get_system_telemetry(&self) -> AntSResult<AntsTelemetry>);
    mock_method!(get_activation_count(&self, antenna: KANTSAnt) -> AntSResult<u8>);
    mock_method!(get_activation_time(&self, antenna: KANTSAnt) -> AntSResult<u16>);
    mock_method!(watchdog_kick(&self) -> AntSResult<()>);
    mock_method!(watchdog_start(&self) -> AntSResult<()>);
    mock_method!(watchdog_stop(&self) -> AntSResult<()>);
    mock_method!(passthrough(&self, tx: &[u8], rx: &mut [u8]) -> AntSResult<()>, self, {
            self.passthrough.call((tx.to_vec(), rx.to_vec()))
        });
}



//TODO: verify master errors matches expected

#[test]
fn arm_good_arm() {
    let mock = mock_new!();

    mock.arm.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Arm).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn arm_error_arm() {
    let mock = mock_new!();

    mock.arm.return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Arm).unwrap();

    assert_eq!(result.errors, "Generic error".to_owned());
    assert_eq!(result.success, false);
}

#[test]
fn arm_good_disarm() {
    let mock = mock_new!();

    mock.disarm.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Disarm).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn arm_error_disarm() {
    let mock = mock_new!();

    mock.disarm.return_value(
        Err(AntsError::GenericError.into()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Disarm).unwrap();

    assert_eq!(result.errors, "Generic error".to_owned());
    assert_eq!(result.success, false);
}

#[test]
fn configure_good_primary() {
    let mock = mock_new!();

    mock.configure.return_value_for(
        (KANTSController::Primary),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.configure_hardware(ConfigureController::Primary)
        .unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.config, ConfigureController::Primary);
}

#[test]
fn configure_good_secondary() {
    let mock = mock_new!();

    mock.configure.return_value_for(
        (KANTSController::Secondary),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.configure_hardware(ConfigureController::Secondary)
        .unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.config, ConfigureController::Secondary);
}

#[test]
fn configure_bad() {
    let mock = mock_new!();

    mock.configure.return_value(
        Err(AntsError::GenericError.into()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.configure_hardware(ConfigureController::Primary)
        .unwrap();

    assert_eq!(result.errors, "Generic error".to_owned());
    assert_eq!(result.success, false);
    assert_eq!(result.config, ConfigureController::Primary);
}

#[test]
fn control_power_good() {
    let mock = mock_new!();

    mock.reset.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.control_power(PowerState::Reset).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.power, PowerState::Reset);
}

#[test]
fn control_power_bad() {
    let mock = mock_new!();

    mock.reset.return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.control_power(PowerState::Reset).unwrap();

    assert_eq!(result.errors, "Generic error".to_owned());
    assert_eq!(result.success, false);
    assert_eq!(result.power, PowerState::Reset);
}

#[test]
fn control_power_invalid() {
    let mock = mock_new!();

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.control_power(PowerState::On).unwrap();

    assert_eq!(result.errors, "Invalid power state".to_owned());
    assert_eq!(result.success, false);
    assert_eq!(result.power, PowerState::On);
}

#[test]
fn deploy_good_all() {
    let mock = mock_new!();

    mock.auto_deploy.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::All, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant1() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant1, false, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna1, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant1_override() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant1, true, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna1, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant2() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant2, false, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna2, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant2_override() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant2, true, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna2, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant3() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant3, false, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna3, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant3_override() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant3, true, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna3, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant4() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant4, false, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna4, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_good_ant4_override() {
    let mock = mock_new!();

    mock.deploy.return_value_for(
        (KANTSAnt::Ant4, true, 5),
        Ok(()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna4, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn deploy_bad() {
    let mock = mock_new!();

    mock.deploy.return_value(
        Err(AntsError::GenericError.into()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna1, true, 5).unwrap();

    assert_eq!(result.errors, "Generic error".to_owned());
    assert_eq!(result.success, false);
}

//TODO: Integration test

#[test]
fn noop_good() {
    let mock = mock_new!();

    mock.watchdog_kick.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.noop().unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
}

#[test]
fn noop_error() {
    let mock = mock_new!();

    mock.watchdog_kick.return_value(
        Err(AntsError::GenericError.into()),
    );

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.noop().unwrap();

    assert_eq!(result.errors, "Generic error".to_owned());
    assert_eq!(result.success, false);
}
