# kubos-core
TODO: Write a project description

## Installation
### Pre-Requisites
1. Download RIOT-OS: `git clone git://github.com/RIOT-OS/RIOT.git`. See https://github.com/RIOT-OS/RIOT/wiki/Introduction for more details.
2. gpsd: `sudo apt-get install gpsd:i386 libgps-dev:i386`
3. 

### kubos-core Installation
1. Download kubos-core to same parent directory as RIOT (or change default Makefile): `git clone git://github.com/openkosmosorg/kubos-core.git` 
2. Within kubos-core directory, type `make all term`. Confirm shell is running ">"
3. Type `help`.  Confirm list of available commands.  See "Usage" section for more details.

## Usage
After running `make all term` from kubos-core root directory, a prompt ">" should appear.  Type "help" to see the list of available commands.

The sections below describe how to run a virtual emulator for each available command. 
### Testing Command: "get_gps"
1. Download "gps.log" from github repo "https://github.com/openkosmosorg/vanguard/tree/master/payload/test"
2. In 2nd terminal window (not running kubos-core terminal), navigate to "gps.log" directory. Type: `gpsfake -u -c 0.01`
3. Type `gps_get` in primary kubos-core terminal
4. Confirm Long, Lat, Alt, and Accuracy displayed

## Contributing
1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request

## History
TODO: Write history

## Credits
TODO: Write credits

## License
TODO: Write license