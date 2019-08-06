# Channel Protocol

This protocol is used to send and receive CBOR-encoded channel based
messages over UDP. Each message consists of three parts: channel ID,
message name, message payload, all contained in a CBOR array.

    { channel_id, name, payload.. }

The channel ID is typically used to group all messages related to a transaction
or logical action. An example would be all messages related to uploading
a single file.
The message name is used to determine the type of message.
The message payload contains any other message data.