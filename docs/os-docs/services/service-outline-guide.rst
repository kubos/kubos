KubOS Service Outlines
======================

This guide covers an outline for the base Schema that should be implemented in all hardware services.
For general information about hardware services, their role, and how they work, check out :doc:`the hardware services documentation. <../services/hardware-services>`

All hardware services must follow the graphQL spec.
The mutation and query responses are the "data" and "errors" fields, and all errors experienced are returned in that "errors" field, even if they are from the underlying hardware.

.. code-block:: graphql

    type Query {
        ping(): String
        config(): ConfigObject
        power(): PowerState
        telemetry(): Telemetry
    }

    type PowerState {
        state: PowerStateEnum
        uptime: Int
    }

    type Telemetry {
        # All key value telemetry items for the Hardware
        field1: Float
        # field2: whatever type
        # field3: whatever type
        # ...
    }

    type Mutation {
        # When the service receives a noop mutation, it should send a command and make sure it sees the appropriate response from the hardware.
        noop(): Boolean
        controlPower(
            input: ControlPowerInput!
        ): ControlPowerPayload
        # Raw passthrough to the hardware for commands not implemented as mutations directly.
        commandRaw(
            input: CommandRawInput!
        ): IssueRawCommandPayload
    }

    type ControlPowerPayload {
        power: PowerState
    }

    input ControlPowerInput {
        state: PowerStateEnum!
    }

    enum PowerStateEnum {
        # Not all of these make sense for every kind of hardware, but implement to the best of your ability.
        ON
        OFF
        RESET
    }

    input CommandRawInput {
        # "command" must always be a utf-8 string. Use "format" to convert as necessary.
        command: String!
        format: FormatEnum!
        # Number of bytes to be read from the hardware as a response. Can be excluded if the hardware does not support it.
        read: Int
    }

    enum FormatEnum {
        # Necessary for binary data to be passed to the hardware
        HEX
        # For plain text passthrough
        STRING
    }

    type IssueRawCommandPayload {
        response: String
        # Format that the response will be in. Use HEX to pass binary data.
        format: FormatEnum
    }
