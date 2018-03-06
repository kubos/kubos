KubOS Service Outlines
=======================

This guide covers the development of KubOS hardware services and provides an outline for several major types of hardware. 
For general information about hardware services, their role, and how they work, check out :doc:`the hardware services documentation. <../services/hardware-services>`


General Hardware Service
-------------------------

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
    
        

Mutations::

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
    
    
    