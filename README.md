# kubos-core

[![Join the chat at https://gitter.im/openkosmosorg/kubos-core](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/openkosmosorg/kubos-core?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
TODO: Write a project description

## Installation
### Pre-Requisites
1. Download RIOT-OS: `git clone git://github.com/RIOT-OS/RIOT.git`. See https://github.com/RIOT-OS/RIOT/wiki/Introduction for more details.
2. gpsd: `sudo apt-get install gpsd:i386 libgps-dev:i386`

### Direwolf Installation (beacon radio)
See https://github.com/wb2osz/direwolf for more details

1. `sudo apt-get install libasound2-dev`
2. `git clone https://www.github.com/wb2osz/direwolf`
3. `make`
4. `sudo make install`
5. `make install-conf` ?? (needed?)

### kubos-core Installation
1. Download kubos-core to same parent directory as RIOT (or change default Makefile): `git clone git://github.com/openkosmosorg/kubos-core.git` 
2. Within kubos-core directory, type `make all term`. Confirm shell is running ">"
3. Type `help`.  Confirm list of available commands.  See "Usage" section for more details.

## Usage
After running `make all term` from kubos-core root directory, a prompt ">" should appear.  Type "help" to see the list of available commands.

The sections below describe how to run a virtual emulator for each available command. 
### Testing Command: "location"
1. Download "gps.log" from github repo "https://github.com/openkosmosorg/vanguard/tree/master/payload/test"
2. In 2nd terminal window (not running kubos-core terminal), navigate to "gps.log" directory. Type: `gpsfake -u -c 0.01 -b gps.log`
3. Type `location` in primary kubos-core terminal
4. Confirm Long, Lat, Alt, Speed and Climb are displayed

### Testing Command: "test_radio"
1. Ensure Direwold is installed (See Installation Section)
1. Download "direwolf.conf" from "https://github.com/openkosmosorg/vanguard/tree/master/payload/config"
2. In new terminal window, navigate to "direwolf.conf" directory. Type: `direwolf -p -t 0 -c "direwolf.conf"`

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
