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

You can specify a fork of KubOS by changing the path `KUBOS` in scripts `kubos-nos3-build` and `kubos-nos3-run` on the desktop.  Run `kubos-nos3-build` script from the desktop.  This builds targets for all NOS3 sims and the 3 KubOS hardware services with feature nos3.  It may be necessary to specify the NOS3 KubOS fork as long as the NOS Engine HALs have yet to be merged into KubOS master (As of writing, the fork lives at github.com/Seabass247/kubos/tree/nos3-demo)

### Novatel OEM6 GPS
The Novatel OEM6 GPS is the only hardware model that is currently supported.  To issue queries and mutations against the sim, go to "localhost:8123" from a web browser in the vm.  To verify everything is working, issue a query to enable bestxyz log messages.  You should see the sim sending log messages from the `[DEBUG]` prints in the GPS sim terminal tab.  If all goes well, querying `lockinfo` should return good position and velocity data from the sim.

Example: 
1. Issue mutation to log bestxyz messages 
```
mutation {
  configureHardware(config: {option: LOG_POSITION_DATA, hold: true, interval: 1, offset: 1}) {
    success
  }
}
```
2. Query `lockinfo`
```
query {
  lockInfo {
    position
    velocity
  }
}
```

### Other Sims & Future Compatibility
Clyde 3G EPS and ISIS AntS can be interacted with on port `8124` and `8125` respectively.  Currently the open source NOS3 repo does not contain these sims;they should come in an August NOS3 open source release.