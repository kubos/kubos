## Building

This service can easily be built as a single luvi bundle.  Simpe install latest
luvit and lit and run:

```sh
lit make lit://kubos/kubos-shell-service
```

And you will have a portable binary `kubos-shell-service` that depends on only
`/home/system/usr/local/bin/luvi-tiny` to run (which in turn only depends on
your system libc in most builds.)

## Running

I recommend uploading `kubos-shell-service` to `/home/system/usr/local/bin/`
which should be in your path.  Then you can run it as simply:

```sh
PORT=6000 kubos-shell-service
```

It will listen for shell service commands over loopback UDP on the given port.

## Commands

This service is a simple UDP process that allows remote control of a
linux box using simple commands.  All datagrams are
[cbor](http://cbor.io/) serialized arrays.  The first value is a string of
the command/function to call.  Everything else is the arguments to pass to
the function.  Return values are ignored in functions and responses must be
sent back using a new command from flight back to the ground.  All state
must be encoded in the parameters.  Typically a single opaque id chosen by
the ground is used for this purpose.

For example, here is what a conversation asking for uname would look like. The  
`<-` prefix means message is received in flight from ground `->` means flight
back down to ground.  Messages are shown as JS literals, but would be cbor in
the actual protocol:

```js
'<-'	[ 'spawn', '/bin/uname', { args: [ '-a' ] } ]
'->'	[ 's-pid', 26319 ]
'->'	[ 's-out', 'Linux t580 4.16.0-041600rc4-generic #201803041930 SMP Mon Mar 5 00:32:34 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux\n' ]
'->'	[ 's-out' ]
'->'	[ 's-err' ]
'->'	[ 's-exit', 0, 0 ]
```

We asked to spawn `/bin/uname` with arguments passed in as an `args` array.  We
got out 5 events:

- process was created with a pid
- stdout emitted the result we were looking for
- stdout was closed
- stderr was closed
- the process exited with code 0 and no signal.

### Process Spawn

The spawn command is used to manage child processes.

There are 5 ground-to-flight messages:

 - `spawn(path, options)` - spawn a child process - make sure id is unique.
 - `s-in(data)` - write to child's stdin
 - `s-in()` - close child's stdin
 - `s-kill(signal)` - send signal to child, defaults to SIGTERM
 - `s-resize(cols, rows)` - if child has a pty, resize it

The `options` argument in `spawn()` is an object that accepts:

 - `args` - an array of arguments to pass to the child process.
 - `pty` - an array containing `[cols, rows]` if you wish to create a pty
   for this process.
 - `env` - an array of environment variable entries in the form `"KEY=val"`.
 - `cwd` - set the current working directory of the child process
 - `uid`, `gid` - set the uid or gid of the process.
 - `detached` - set if you want the process detached from this service.

There are 3 different flight-to-ground messages (though 2 are dual purpose)

 - `s-pid(pid)` - process was created, this is the pid
 - `s-out(data)` - data came out of stdout
 - `s-out()` - stdout closed
 - `s-err(data)` - data came out of stderr
 - `s-err()` - stderr closed
 - `s-exit(code, signal)` - process exited with signal and/or code
