---
title: SysDB and gNMI
description: streaming EOS SysDB data over gNMI
date: 2024-01-20
tags:
    - eos
    - terminattr
    - gnmi
    - openconfig
---
### Exploring Sysdb with TerminAttr and REST API

1. **Enable TerminAttr**: Begin by enabling the TerminAttr daemon on your device. This can be done with the following configuration commands:

```
spine1#config
spine1(config)#daemon TerminAttr
spine1(config-daemon-TerminAttr)#exec /usr/bin/TerminAttr -grpcaddr 0.0.0.0:6042
spine1(config-daemon-TerminAttr)#no shutdown
spine1(config-daemon-TerminAttr)#bash
```

This configuration initializes the TerminAttr service, which provides a REST API to explore its data, including Sysdb.

2. **Access Sysdb via REST API**: Once TerminAttr is running, use the REST API to explore Sysdb. Start by checking available paths:

```
bash-4.2# curl localhost:6060/rest/Sysdb
{
    "config": {
        "_ptr": "/Sysdb/sys/net/config"
    }
}
```

This command will return a JSON structure with various Sysdb paths, each having a `_ptr` reference.

3. **Browse Sysdb Paths**: Use the REST API to navigate through the Sysdb paths. For example, to access network configuration data, you might use:

```
bash-4.2# curl localhost:6060/rest/Sysdb/sys/net/config
{
    "dnsCacheCountersUpdateInterval": 3,
    "dnsCacheSize": 4096,
    "domainList": {},
    "domainListMetadata": {
        "head": 0,
        "tail": 0
    },
    "domainName": "",
    "dscpValue": 0,
    "externalDnsProxy": false,
    "hostAddr": {},
    "hostname": "n1",
    "hostnameTimeout": 20,
    "name": "config",
    "nameServer": {
        "1.1.1.1_default": {
            "priority": 0,
            "vrfIpPair": {
                "ip": "1.1.1.1",
                "vrfName": {
                    "value": "default"
                }
            }
        }
    },
    "sourceIntf": {},
    "v6NameServer": {}
}
```

You will receive a JSON response with the relevant Sysdb data.

### Streaming Sysdb Data Over gNMI

After identifying the desired Sysdb paths using TerminAttr and the REST API, you can proceed to stream this data over gNMI. For non-openconfig paths, use the `eos_native` prefix and enable it on the device.

```
management api gnmi
   transport grpc default
       no shutdown
   provider eos-native
```

If we try to get the hostname directly, we get an empty response:
```
bash-4.2# gnmic -a 0.0.0.0:6030 -u admin -p admin --insecure --gzip get --path "eos_native:/Sysdb/sys/net/config/hostname"
[]
```

This is because when pulling from Sysdb via gNMI, we need to use the closest `_ptr` reference, as only entire objects can be pulled. In this case, we can use the last `_ptr` we saw in the REST API output:
```
bash-4.2# gnmic -a 0.0.0.0:6030 -u admin -p admin --insecure --gzip get --path "eos_native:/Sysdb/sys/net/config" --format flat
dnsCacheCountersUpdateInterval: {}
eos_native:Sysdb/sys/net/config/dnsCacheSize: 4096
eos_native:Sysdb/sys/net/config/domainListMetadata/head: 0
eos_native:Sysdb/sys/net/config/domainListMetadata/tail: 0
eos_native:Sysdb/sys/net/config/domainName:
eos_native:Sysdb/sys/net/config/dscpValue: 0
eos_native:Sysdb/sys/net/config/externalDnsProxy: false
eos_native:Sysdb/sys/net/config/hostname: spine1
eos_native:Sysdb/sys/net/config/name: config
hostnameTimeout: {}
```