# Setup

This service is implemented as a [luvi](https://github.com/luvit/luvi)
application.

The single-file executable is literally a Unix
[hashbang](https://en.wikipedia.org/wiki/Shebang_(Unix)) line containing
`#!/home/system/usr/bin/luvi-tiny --\n` with a zip-file of
[Lua](https://www.lua.org/) scripts and assets appended.

## Building and Installing `luvi-tiny`

If you don't already have `luvi-tiny` you'll need to build it.

Here are instructions for building on a typical Ubuntu or Debian system:

```sh
# Install required build tools.
sudo apt install build-essential cmake git
# Checkout the source to luvi.
git clone --recursive https://github.com/luvit/luvi.git
cd luvi
# Configure and built the tiny flavor.
make tiny
make
make test
# Install the binary to the expected location.
sudo mkdir -p /home/system/usr/bin
sudo cp build/luvi /home/system/usr/bin/
```

## Installing Lit

Now that we have the native code built and installed as `luvi-tiny`, everything
else is just lua code and zip files.  The [Luvit Invention
Toolkit](https://github.com/luvit/lit) makes is much easier.

The official [luvit install instructions](https://luvit.io/install.html) will
download prebuilt `luvi` and build `luvit` and `lit` for you.

In short, run the following:
```sh
# Get `luvit`, `luvi`, and `lit`
curl -L https://github.com/luvit/lit/raw/master/get-lit.sh | sh
# Make sure we have latest `lit`
./lit update
# Move these to the same bin folder as `luvi-tiny`
sudo mv lit luvit luvi /home/system/usr/bin/
```

## Building the Service

Assuming `lit` is in your path we can now build from this folder.

```sh
# Make sure you're in the kubos/services/shell-service/ folder
lit make .
# Install the generated file
sudo mv ./kubos-shell-service /home/system/usr/bin/
```

*Note*: this binary is portable and can run unmodified on any device that
has a local native version of `/home/system/usr/bin/luvi-tiny`.

If you don't want to build the latest version from git, you can also build the
latest published version of the service by running:

```sh
lit make lit://kubos/kubos-shell-service
sudo mv ./kubos-shell-service /home/system/usr/bin/
```

# Running the Service

Assuming the file is installed to `/home/system/usr/bin` which is in `$PATH`:

```sh
# Specify the UDP port to listen on as an environment variable.
PORT=6000 kubos-shell-service
```

It will listen for shell service commands over loopback UDP on the given port.

# Running the CLI Client

A simple CLI client is provided for convenience for testing the service and
providing basic access to devices that have this service installed.  The client
has the same install instructions as the service, except it's source is located
in the [clients/shell-client folder](../../clients/shell-client/) in this git
repo or published as `lit:kubos/kubos/shell-client`.

```sh
lit make lit://kubos/kubos-shell-client
./kubos-shell-client
```

This client assumes the shell service is running on `UDP://127.0.0.1:6000`  This
will work for local testing services as well as remote services who's UDP port
is forwarded by the communications service.

# Service Protocol

This service is a simple UDP process that allows remote control of a Linux box
using simple commands.  

## Message Format

All datagrams are [CBOR](http://cbor.io/) serialized
arrays.  A message is typically encoded as the following:

```js
[channel_id, command, parameters...]
```

The `channel_id` is a random integer chosen by the client and represents a
virtual communications channel.  Since our transport is stateless UDP this is
needed to associate I/O streams and commands with various processes.

The `command` is a short string of the command or event being communicated and
`parameters` is zero or more values that are passed to the command handler as
function arguments.

## Spawn Messages

The main job of this service is to spawn and control remote processes.

### Spawn Commands

The service handles the following commands:

- `spawn(command, options)` - Spawn a child process. The `command` can be
  absolute path or something in system `$PATH`.
- `stdin(data)` - Write to child's stdin.
- `stdin()` - Close child's stdin.
- `kill(signal)` - Send signal to child; defaults to SIGTERM.
- `resize(cols, rows)` - If child has a pseudo terminal, resize it.

The `options` argument in `spawn()` is an object that accepts:

- `args` - an array of arguments to pass to the child process.
- `pty` - a boolean specifying we want a new pty for this process.
- `env` - an array of environment variable entries in the form `"KEY=val"`.
- `cwd` - set the current working directory of the child process
- `uid`, `gid` - set the uid or gid of the process.
- `detached` - set if you want the process detached from this service.

### Spawn Events

The service will emit the following events that need to be handled by the
client:

 - `pid(pid)` - Process was created; this is the pid.
 - `stdout(data)` - Data came out of stdout.
 - `stdout()` - Stdout was closed.
 - `stderr(data)` - Data came out of stderr.
 - `stderr()` - Stderr was closed.
 - `exit(code, signal)` - Process exited with signal and/or code.

## List Command

In addition to spawning and managing individual processes, there is a `list`
command that lists currently managed processes.  This is used to allow clients
to take over or resume lost shell sessions.

 - `list()` - Client sends `list` command to server.
 - `list(processes)` - Service sends `list` event to client.

The `processes` object is a map from `channel_id` to `{path, pid}`

## Example Short-Lived Command

I'm showing messages here as JavaScript literals, but remember it's send over
the wire as binary CBOR.

The goal is to run `uname -a` on the remote machine and see the output.

The client randomly chooses `42` as it's `channel_id` and sends a `spawn`
command with the arguments.

```js
Client: [ 42, 'spawn', '/bin/uname', { args: [ '-a' ] } ]
```

The server emits a several events since it's a short-lived process.

```js
Server: [ 42, 'pid', 26319]
Server: [ 42, 'stdout', 'Linux t580 4.16.0-041600rc4-generic #201803041930 SMP Mon Mar 5 00:32:34 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux\n' ]
Server: [ 42, 'stdout' ]
Server: [ 42, 'stderr' ]
Server: [ 42, 'exit', 0, 0 ]
```

Translated this means the following happened:

- Process was created with PID of `26319`.
- Some data came out of stdout.
- The stdout stream was closed.
- The stderr stream was closed.
- The process exited with code 0 and/or signal 0.

## Example Long-Lived Command

If a process doesn't exit on it's own, you can query for running processes,
write to their stdin or send them signals.

In these examples commands are shown as Lua literals.  They are still sent on
the wire as CBOR.

### Spawning Sleep

```lua
Client: { 1337, 'spawn', 'sleep', { args = { '1000' } } }
Server: { 1337, 'pid', 16337 }
```

We created a very long-lived `sleep 1000` command and it told us the PID, but
nothing more since it didn't do anything else.

### Finding Sleep

Let's pretend we forgot about the command and came back later to see what's running:

```lua
Client: { 2000, 'list' }
Server: { 2000, 'list', { [1337] = { path = 'sleep', pid = 16337 } } }
```

We used the list command with a new `channel_id` and it told us the `sleep`
command is still running with `channel_id` of 1337 and PID of 16337.

Remember that if you want multiple concurrent processes or request name-spaces,
you need a unique `channel_id` for each so the streams don't get crossed.

### Killing Sleep

Let's send it the default signal of `SIGTERM`

```lua
Client: { 1337, 'kill' }
Server: { 1337, 'stderr' }
Server: { 1337, 'stdout' }
Server: { 1337, 'exit', 0, 15 }
```

We used the `channel_id` given in the listing to send a `kill()` command to the
process.  By default it will send signal 15 which is `SIGTERM`.  We can see the
stdout and stderr both closed and the process exited with signal 15 as the
reason.

### Writing to Input

Let's do one more example writing to stdin.

```lua
Client: { 9000, 'spawn', 'sort' }
Server: { 9000, 'pid', 16742 }
Client: { 9000, 'stdin', 'Apple\n' }
Client: { 9000, 'stdin', 'Grape\n' }
Client: { 9000, 'stdin', 'Banana\n' }
```

We spawned an instance of `sort` and wrote 3 lines to it's stdin stream.  Sort
doesn't output anything till it's stdin is closed, so let's do that and see the
output:

```lua
Client: { 9000, 'stdin' }
Server: { 9000, 'stdout', 'Apple\nBanana\nGrape\n' }
Server: { 9000, 'stdout' }
Server: { 9000, 'stderr' }
Server: { 9000, 'exit', 0, 0 }
```

Perfect!  We get the result in stdout and the process exits cleanly.

### Full Terminal

See the [kubos-shell-client](../../clients/shell-client) source code for a more
involved example of spawning a persistent `sh -l` session with allocated pseudo
terminal as well as using `list` to resume shell sessions.
