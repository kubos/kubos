# Monitor Service

Service for monitoring KubOS processes, memory, and CPU usage

# Running the Service

The service should be started automatically by its init script, but may also be started manually:

```bash
$ monitor-service
Listening on: 127.0.0.1:8089
```

If no config file is specified, then the service will look at `/etc/kubos-config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ monitor-service -c config.toml
```

# GraphQL Schema

```graphql
schema {
    query: Query
}

type Query {
    ping: String!
    memInfo: MemInfo!
    loadAverage: LoadAverage!
    logFiles: LogFiles!
    uptime: Float!
    mounts: [Mount!]!
    ps(pids: [Int!] = null): [ProcInfo!]!
}

type MemInfo {
    total: Int
    free: Int
    available: Int
    lowFree: Int
}

type LoadAverage {
    one: Float!
    five: Float!
    fifteen: Float!
}

type LogFiles {
    kubosModTime: Float
    appModTime: Float
}

type Mount {
    free: Float!
    avail: Float!
    total: Float!
    files: Float!
    filesTotal: Float!
    filesAvail: Float!
    fsType: String!
    fsMountedOn: String!
    fsMountedFrom: String!
}

type ProcInfo {
    pid: Int!
    uid: Int
    gid: Int
    usr: String
    grp: String
    state: String
    ppid: Int
    mem: Int
    rss: Int
    threads: Int
    cmd: String
}
```
