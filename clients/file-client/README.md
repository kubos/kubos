# Kubos File Client

This client is implemented as a [luvi](https://github.com/luvit/luvi)
application.

The single-file executable is literally a Unix
[hashbang](https://en.wikipedia.org/wiki/Shebang_(Unix)) line containing
`#!/home/system/usr/bin/luvi-tiny --\n` with a zip-file of
[Lua](https://www.lua.org/) scripts and assets appended.

## Building and Installing `luvi-tiny` and Installing Lit

See [shell-service](..../services/shell-service) for docs on building
`luvi-tiny` and `lit`.

## Building the Service

Assuming `lit` is in your path we can now build from this folder.

```sh
# Make sure you're in the kubos/clients/file-client/ folder
lit make .
# Install the generated file
sudo mv ./kubos-file-service /home/system/usr/bin/
```

*Note*: this binary is portable and can run unmodified on any device that
has a local native version of `/home/system/usr/bin/luvi-tiny`.

If you don't want to build the latest version from git, you can also build the
latest published version of the service by running:

```sh
lit make lit://kubos/kubos-file-client
sudo mv ./kubos-file-client /home/system/usr/bin/
```

## Running the Client

Assuming the file is installed to `/home/system/usr/bin` which is in `$PATH`:

```sh
# Specify the UDP port to listen on as an environment variable.
PORT=7000 kubos-file-service upload path/to/file [optional/remote/path]
PORT=7000 kubos-file-service download remote/path [optional/local/path]
```

The `upload` command will import a file into the local `./storage` folder, issue
a remote export command and send any chunks the remote side is missing.

The `download` command will request a remote import into remote `./storage` and
check the local `./storage` to see what chunks are missing and request them.  
Once all chunks are received, it will export the file to the file-system.

## Running the File Service Client

See the README for the [kubos-file-service](../services/file-service).
