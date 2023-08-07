---
title: Authentication and Authorization
weight: 60
---


Fluvio uses mTLS (mutual TLS) for authentication between client and the fluvio cluster.   The user id (or principle) is extracted from the client certificate then apply to authorization policy.  Currently, Fluvio supports a simple RBAC (role based access control) authorization model but it could be extended to support more complex authorization scheme in the future. 

# Cluster setup TLS

First Fluvio must be running with TLS enabled. Once TLS is enabled, Both SC and SPU will accept mTLS base traffic from the client.  As part of TLS based configuration, admin can also specify authorization policy file.

To create a Fluvio cluster with TLS enabled, you must provide following certs and keys:
- CA cert
- Client cert and key
- Server cert and key
- Authorization policy file

For development and testing, you can use self-signed certs and keys.  For production, you should use certs and keys signed by a trusted CA.  It is important to note that the client cert and key must be signed by the same CA as the server cert and key.  Otherwise, the client will not be able to connect to the server.  In addition, admin must specify domain (DNS name) that cluster will be bind to.  This is required for TLS to work properly.

## Self signed certs

For self-signed certs, you can use sample certs templates in [fluvio](https://github.com/infinyon/fluvio/tree/master/tls).  

## Authorization policy file

Fluvio current implements a simple type level permission model.  The authorization policy file is a JSON file that contains a list of permission entries for the role. Each entry contains a type and a list of permissions.  Tye type can be one of the following object types.  Current list of types are:
-  CustomSpu
-  Spu
-  Topic
-  Partition
-  SpuGroup
-  SmartModule
-  TableFormat

Each object type can have one or more permissions.
- All
- Read
- Write

The `All` permission means the user has all permissions for the object type.   For example, following is a policy file.  It contains a two roles: `Root` and `User`.  The `Root` role has all permissions for all object types.  The `TopicUser` role only has read permission for `Topic` object type.

```json
{
    "Root": {
        "CustomSpu": [
            "All"
        ],
        "Partition": [
            "All"
        ],
        "Spu": [
            "All"
        ],
        "Topic": [
            "All"
        ],
        "SpuGroup": [
            "All"
        ],
        "ManagedConnector": [
            "All"
        ],
        "SmartModule": [
            "All"
        ],
        "TableFormat": [
            "All"
        ]
    },
    "TopicUser": {
        "Topic": [
            "Read"
        ]
    }

}
```

Note that, in order to admin the cluster, "Root" role with full access to all objects is required.

## Authorization Scope file

The scope file contains mapping of users to roles. For example, here is a sample scope file that maps user `user1` and `user2` to `TopicUser` role.  The `Root` role is mapped to user `root`.

```json
{
    "Root": [
        "root"
    ],
    "TopicUser": [
        "user1",
        "user2"
    ]
}
```

The user id comes from the X509 client certs.  It is mapped to `CN`.  For example, this is a simple command to generate a client cert with `CN` set to `user1`.

```bash
openssl req -new -key <client_key> -out <client.csr> -subj "/C=US/ST=CA/O=MyOrg, Inc./CN=user1"
```

The client cert with `CN` set to `user1` can be used to connect to Fluvio cluster.  The Fluvio cluster will map the `user1` to `TopicUser` role.

## Provisioning Fluvio cluster with TLS

Once all the certs and keys are ready, you can provision a Fluvio cluster with TLS enabled.  Note that client certs must have user "root" (default policy mapping) in order to provision the cluster.  Here is syntax to provision a Fluvio cluster with TLS enabled.

```bash
fluvio cluster start --tls --domain <domain> --ca-cert ca_cert --server-cert <server_cert> --server-key <server_key> --client-cert <client_cert> --client-key <client_key> --authorization-policy <policy_file> --authorization-scope <scope_file>
```

The domain must match DNS suffice in order to TLS to work properly.  For example, if you are using self-signed certs, you can use `local` or `localhost` as domain.  If you are using certs signed by a trusted CA, you can use the domain name that is registered with the CA.

# Connecting to Fluvio cluster with TLS

In order to connect to Fluvio cluster with TLS enabled, you must provide client certs and keys. They must be signed by the same CA as the server cert and key.  In addition, it must have CN(user) that is in the authorization scope file.  The client TLS certs and key can be either provided in the fluvio profile file or specified in the env variables.  Here is an section of fluvio profile file that contains client certs and keys.   Currently TLS certs must be manually copied to config file.


```toml
[cluster.mycluster.tls.certs]
domain = "fluvio-cluster.mydomain.com"
key = """

-----BEGIN RSA PRIVATE KEY-----
MIIJKQIBAAKCAgEAtLx58BuGSbwWW6AlNb38965IWQv+IJZGPBzMs9GExohuHOai
...
```




