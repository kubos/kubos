KubOS Service Outline
======================

This guide covers an outline for the base Schema that should be implemented in all hardware services.
Your service can and should implement more mutations and queries beyond those specified in the required base.
For general information about hardware services, their role, and how they work, check out :doc:`the hardware services documentation. <../services/hardware-services>`

All hardware services must follow the :url:`GraphQL spec. <https://graphql.github.io/graphql-spec/>`
The mutation and query responses are the "data" and "errors" fields, and all errors experienced are returned in that "errors" field, even if they are from the underlying hardware.
If there is any functionality in the below schema that is unable to be implemented by the service, return an error with the string "Not Implemented".

Queries
-------

The queries in the schema below are intended to give a standard interface for mission applications.
Each query listed is used in the automated systems within KubOS to ease the mission development process.
Omitting any of the following queries will cause compatibility issues with the rest of the KubOS ecosystem.
Adding additional queries is highly encouraged, especially if it will further ease integration with the underlying hardware.
For example, a `currentPosition` query on a gps service could provide faster access to apps that need it.

.. code-block:: graphql

    type Query {
        # Used for doing regular health checks on the service
        ping(): String
        # Used for aggregating current bus configuration
        config(): ConfigObject
        # Used for aggregating current bus state
        power(): PowerState
        # Used for automated telemetry collection and distribution
        telemetry(): Telemetry
    }

    type PowerState {
        state: PowerStateEnum
        uptime: Int
    }

    type Telemetry {
        # All key value telemetry items for the hardware
        field1: Float
        # field2: whatever type
        # field3: whatever type
        # ...
    }

Mutations
---------

Mutations are harder to standardize across all hardware, but make your best attempt to fully implement the short list of mutations below.
As noted before, any that cannot be implement must return a response:: {"errors": ["Not Implemented"], "data": null}

.. code-block:: graphql

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
        STR
    }

    type IssueRawCommandPayload {
        response: String
        # Format that the response will be in. Use HEX to pass binary data.
        format: FormatEnum
    }

commandRaw mutation
___________________

Direct hardware commands are often issued in binary format that does not comply with the utf-8 string requirements.
Since we want to support the passing of raw binary commands to underlying hardware, but do not want to deviate from the GraphQL specification, we've added the FormatEnum to specify how the utf-8 compatible string can be converted to the raw data that must be passed to the hardware. 
The HEX format is for those cases, such that passing a hex string: `"74657374636f6d6d616e64"`, causes the bytearray: `[116,101,115,116,99,111,109,109,97,110,100]` to be passed to the hardware.

Some hardware expects utf-8 compatible string commands, so those services would leverage the STR format to pass the data directly to the hardware. EG: the string `"testcommand"` gets passed to the hardware as the bytearray: `[116,101,115,116,99,111,109,109,97,110,100]`.
