KubOS Service Outlines
======================

This guide covers the development of KubOS hardware services and provides an outline for several major types of hardware.
For general information about hardware services, their role, and how they work, check out :doc:`the hardware services documentation. <../services/hardware-services>`
Make sure you reference the general hardware service when looking at the schema for all others, because they simply build on that schema, and each section covers what is added to that base schema.


General Hardware Service
------------------------

A general hardware service is a service for any piece of hardware that does not fit into any of the other categories. These queries/mutations will be expected to be present regardless of the hardware. All other service outlines build on top of what is present here.

GraphQL Schema::

    type Query {
        ack(): String
        power(): PowerState
        config(): String
        errors(): [String] # Error descriptions if there are any, or empty if there aren't
        telemetry(): Telemetry
        testResults(): TestResults
    }
        
    type PowerState {
        state: PowerStateEnum
        uptime: Int
    }
    
    type Telemetry {
        nominal: TelemetryNominal
        debug: TelemetryDebug
    }
    
    type TelemetryNominal {
        # Telemetry items that are required to know the general status of the hardware
        field1: Float
        # field2: whatever type
        # field3: whatever type
        # ...
    }
    
    type TelemetryDebug {
        # Telemetry items that are only useful if actively debugging/diagnosing the system
        field1: Float
        # field2: whatever type
        # field3: whatever type
        # ...
    }
    
    type TestResults {
        # Results of last test performed. success, telemetryNominal, and telemetryDebug are always present 
        # Additional results can be added as indicated 
        success: Boolean
        telemetryNominal: TelemetryNominal
        telemetryDebug: TelemetryDebug
        # results1: any type
        # results2: any type
        # ...
    }

    type Mutation {
        noop(): NoopPayload
        controlPower(
            input: ControlPowerInput!
        ): ControlPowerPayload
        configureHardware(
            input: ConfigureHardwareInput!
        ): ConfigureHardwarePayload
        testHardware(
            input: TestHardwareInput!
        ): TestHardwarePayload
        issueRawCommand(
            input: IssueRawCommandInput!
        ): IssueRawCommandPayload
    }
    
    # Result of an attemped mutation
    interface MutationResult {
        errors: [String]
        success: Boolean
    }
    
    # Simply confirms that the unit is present and talking
    type NoopPayload implements MutationResult {
        errors: [String]
        success: Boolean
    }
    
    type ControlPowerPayload implements MutationResult {
        errors: [String]
        success: Boolean
        power: PowerState
    }
        
    input ControlPowerInput {
        state: PowerStateEnum!
    }
    
    enum PowerStateEnum {
        ON
        OFF
        RESET
    }
    
    type ConfigureHardwarePayload implements MutationResult {
        errors: [String]
        success: Boolean
        config: String
    }
    
    input ConfigureHardwareInput {
        config: String
    }
    
    # Hardware testing has 2 levels: 
    # INTEGRATION is to test the FSW's compatibility with the unit
    # HARDWARE is to test that the hardware itself is functioning
    type TestHardwarePayload implements MutationResult {
        errors: [String]
        success: Boolean
        results: TestResults
    }
    
    input TestHardwareInput {
        testType: TestType
    }
    
    enum TestTypeEnum {
        INTEGRATION
        HARDWARE
        # Add other types as needed
    }
    
    type IssueRawCommandPayload implements MutationResult {
        errors: [String]
        success: Boolean
        ack: String
    }
    
    input IssueRawCommandInput {
        # Input for this is really whatever it needs to be for the specific unit, and can be changed accordingly
        command: String
    }
    

ADCS Service
------------

The ADCS service outline and all following service outlines aim to abstract just the telemetry items and commands that are useful for mission logic. If you need a certain telemetry item for your mission application, please let us know!

Additional GraphQL Schema::

    type Query {
        mode(): String
        orientation(): [Float]
        spin(): [Float]
    }
    
    type Mutation {
        setMode(
            input: SetModeInput!
        ): SetModePayload
        update(
            input: UpdateInput
        ): UpdatePayload
    }
    
    type SetModePayload implements MutationResult {
        errors: [String]
        success: Boolean
    }
        
    input SetModeInput {
        mode: String
        configuration: ModeConfiguration
    }
    
    # Whatever is needed for the ADCS to enter a mode
    type ModeConfiguration { 
        parameter1: Float
        # parameter2: any type
        # parameter3: any type 
        # ...
    }
    
    type UpdatePayload implements MutationResult {
        errors: [String]
        success: Boolean
    } 
    
    input UpdateInput {
        time: Float
        gpsLock: [Float]
        # whatever else needs to be updated for the unit to function properly
    }


GPS Service
-----------

Additional GraphQL Schema::

    type Query {
        lockStatus: LockStatus
        lockTelemetry: LockTelemetry
    }
    
    type LockStatus { 
        time: LockStatusEnum
        position: LockStatusEnum
        velocity: LockStatusEnum
    }
    
    enum LockStatusEnum {
        YES
        NO
    }
    
    # Values from last lock (or current values if currently locked)
    type LockTelemetry {
        time: Float
        position: [Float]
        velocity: [Float]
    }


Battery and EPS Service(s)
--------------------------

These functions are often combined into a single piece of hardware. If so, then the schema holds for that single service. If they are separate pieces of hardware, implement everything possible for each service.

Additional GraphQL Schema::

    type Query {
        solar: SolarStatus
        ports: PortStatus
        power: PowerStatus
        battery: BatteryStatus
    }
    
    type SolarStatus {
        chargingStatus: ChargingEnum
        panelVoltages: [Float]
        panelCurrents: [Float]
        panelTemperatures: [Float]
    }
    
    enum ChargingEnum {
        CHARGING
        DISCHARGING
    }
    
    type PortStatus {
        power: [PowerEnum]
        voltage: [Float]
        current: [Float]
    }
    
    enum PowerEnum {
        ON
        OFF
    }
    
    type PowerStatus {
        voltageLines: [Float] # Available voltages on the bus
        measuredLineVoltage: [Float] # Actual voltages of the available lines
        measuredLineCurrent: [Float] # Current for each voltage line
    }
    
    type BatteryStatus {
        stateOfCharge: [Float]
        chargingStatus: ChargingEnum
        voltage: Float
        current: Float
        temperature: [Float]
        heater: HeaterEnum
        heaterMode: HeaterEnum 
    }
    
    enum HeaterEnum {
        ON
        OFF
        AUTO
    }
    
    type Mutation {
        controlPort(
            input: ControlPortInput!
        ): ControlPortPayload
        controlHeater(
            input: ControlHeaterInput!
        ): ControlHeaterPayload
    }
    
    type ControlPortPayload implements MutationResult {
        errors: [String]
        success: Boolean
    }
    
    input ControlPortInput {
        power: PowerEnum
        port: Int
    }
    
    type ControlHeaterPayload implements MutationResult {
        errors: [String]
        success: Boolean
    }
    
    input ControlHeaterInput {
        status: HeaterEnum
    }


Deployables Service
-------------------

The deployables service covers anything that needs to be deployed. It focuses on abstracting the logic for deploying panels, antenna, etc. from the mission logic to keep it as clean as possible. This schema should be added for any services that control hardware with deployables.

Additional GraphQL Schema::

    type Query {
        armStatus: ArmStatusEnum
        deploymentStatus: DeploymentStatusEnum
    }
    
    enum ArmStatusEnum {
        ARMED
        DISARMED
    }
    
    enum DeploymentStatusEnum {
        DEPLOYED
        STOWED
    }
    
    type Mutation {
        arm(
            input: ArmInput!
        ): ArmPayload
        deploy(
            input: DeployInput!
        ): DeployPayload
    }
    
    type ArmPayload implements MutationResult {
        errors: [String]
        success: Boolean
    }
    
    input ArmInput {
        arm: ArmEnum
    }
    
    enum ArmEnum {
        ARM
        DISARM
    }
    
    type DeployPayload implements MutationResult {
        errors: [String]
        success: Boolean
    }
    
    input DeployInput {
        burntime: Int
    }


Additional Services
-------------------

If there are any major service categories that should be added, or if you feel that any sections are missing Queries or Mutations that would be essential for Mission Applications, please let us know on our `Slack <https://slack.kubos.co/>`__ or open a PR to add them yourself!

