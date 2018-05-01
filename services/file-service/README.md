# Kubos File Service

This service is implemented as a [luvi](https://github.com/luvit/luvi)
application.

The single-file executable is literally a Unix
[hashbang](https://en.wikipedia.org/wiki/Shebang_(Unix)) line containing
`#!/home/system/usr/bin/luvi-tiny --\n` with a zip-file of
[Lua](https://www.lua.org/) scripts and assets appended.

## Building and Installing `luvi-tiny` and Installing Lit

See [shell-service](../shell-service) for docs on building `luvi-tiny` and `lit`.

## Building the Service

Assuming `lit` is in your path we can now build from this folder.

```sh
# Make sure you're in the kubos/services/file-service/ folder
lit make .
# Install the generated file
sudo mv ./kubos-file-service /home/system/usr/bin/
```

*Note*: this binary is portable and can run unmodified on any device that
has a local native version of `/home/system/usr/bin/luvi-tiny`.

If you don't want to build the latest version from git, you can also build the
latest published version of the service by running:

```sh
lit make lit://kubos/kubos-file-service
sudo mv ./kubos-file-service /home/system/usr/bin/
```

## Running the Service

Assuming the file is installed to `/home/system/usr/bin` which is in `$PATH`:

```sh
# Specify the UDP port to listen on as an environment variable.
PORT=7000 kubos-file-service [optional-nak-timeout]
```

It will listen for file service commands over loopback UDP on the given port.

The default nak timeout is 1000ms, but if you have a very slow connection such
as 9600 baud serial, then it should be longer such as 5000ms.  The goal is to
make it long enough that it won't send naks between each message, but only after
the last one.

## Running the CLI Client

See the README for the [kubos-file-client](../clients/file-client).

##  File Protocol

All commands are sent using the `kubos/cbor-message-protocol` helper library
that encodes all messages as [CBOR](http://cbor.io/).  The first value in the
encoded list is the `channel_id` for request/response type commands and the
`hash` for content-addressable based messages.

### `{ hash, chunk_index, data }` - Send Chunk

Files are sent to the remote side in 4k chunks.  This is the payload message
containing the chunks indexed by file hash and chunk index.

The protocol won't typically reply to these unless there is a long pause in
sending (such as after sending the last chunk).  In this case the receiver will
send either an Ack or Nak depending on if all chunks are received.

### `{ hash, num_chunks }` - Sync

To query is the other side has all chunks for a given file, send a sync message.
The `num_chunks` is optional, but should be sent for new files so the other side
knows how many chunks to expect.

Expect an Ack or Nak in response.

### `{ hash, true, num_chunks }` - Acknowledge

This message tells the other side that the sender has all chunks for a given
hash file.

### `{ hash, false, 1, 4, 6, 7 }`- Negative Acknowledge

This message tells the other side that the sender is missing chunks.  The
numbers after the `false` are pairs of ranges where the first number is
inclusive and the second is exclusive.  For example `0, 2` means the first two
chunks are missing.

### `{ channel_id, "export", hash, path, mode }` - Export

The sender wishes that the remote side exports a file from content addressable
storage to somewhere on the normal file-system.  Typically this is the first
command then uploading a file to a remote service.  

If the chunks aren't there yet, the export will wait for them to be uploaded.

### `{ channel_id, "import", path }` - Import

Import is used to tell a remote side to import a file from the normal
file-system into the managed content addressable storage.  This will respond
with the hash and number of chunks.

### `{ channel_id, true, ...values }` - Success

When an import or export command finishes with success, this will be received
with the results.

### `{ channel_id, false, error_message}` - Failure

If there is an error in the import or export, this will be received with an
error message.
