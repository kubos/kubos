# Using NOS3 & KubOS Together

## Setting up the NOS3 vm
1. Pull NOS3 from the NOS3 repo (temporarily the kubos integration NOS3 fork)
2. cd ./support, edit Vagrantfile
3. change `loi` to 'K'; uncomment the associated line
4. run `vagrant up`
5. once finished, do `vagrant reload`
6. The vm should launch, log in with password: `nos3123!`

## Using KubOS in NOS3
The KubOS repo will be installed to `~/kubos`

Run `kubos-nos3-build` script from the desktop.  This builds targets for all NOS3 sims and the 3 KubOS services with feature nos3.
You can specify a fork of KubOS by changing the path `KUBOS` in the `kubos-nos3-run` and `kubos-nos3-run` scripts on the desktop.  This may be necessary as long as the NOS Engine HALs have yet to be merged into KubOS master (As of writing, the NOS3 KubOS fork lives at github.com/Seabass247/kubos)

### Novatel OEM6 GPS
The Novatel OEM6 GPS is the only hardware model that is currently supported.  To issue queries and mutations against the sim, go to "localhost:8123" from a web browser in the vm.  To verify everything is working, issue a query to enable bestxyz log messages.  You should see the sim sending log messages from the DEBUG prints in the GPS sim terminal tab.  If all goes well, `lockinfo` should return good position and velocity data from the sim.

### Other Sims & Future Compatibility
Clyde 3G EPS and ISIS AntS can be interacted with on port `8124` and `8125` respectively.  Currently the open source NOS3 master does not contain these sims; they should come in an August release.