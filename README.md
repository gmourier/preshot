A simple command set to accompany a deterministic use of Meilisearch API keys.

## Overview

```bash
SUBCOMMANDS:
    discover-keys     Discover key fields value for a list of UIDs and a master key
    generate-keys     Generate (uid, key) tuples
    generate-uuids    Generate uuid(s) V4
    help              Print this message or the help of the given subcommand(s)
```

## discover-keys

```bash
blitz discover-keys master_key d7d30ffe-ec60-484f-84f8-1c8b7d0ac352 c5a18797-621c-42b5-81bd-23fbf0202364
```

```bash
 uid                                  | 🔑 key
 d7d30ffe-ec60-484f-84f8-1c8b7d0ac352 | 623359df9ea4d4a6c676c329c793191601ce7dd15541c2394277eae26aeedf1e
 c5a18797-621c-42b5-81bd-23fbf0202364 | 9decc7baffbed2fa9b9cfa599c3d72ecf8db3fad02b65941caa378e824299482
 ```

```bash
Discover key fields value for a list of UIDs and a master key

USAGE:
    blitz discover-keys <MASTER_KEY> [UIDS]...

ARGS:
    <MASTER_KEY>    Meilisearch master key
    <UIDS>...       API key uids
```

## generate-keys

```bash
blitz generate-keys master_key 3
```

```bash
 uid                                  | 🔑 key
 3cc1b0bf-cd20-4a9e-b6b3-f807dc084809 | 573570a095d46fd53ca10419d7ecc01e7247215a2644dbf5d706c0b06eaa02cd
 9d92afc7-cc0c-48d9-80da-eb5af536407b | c6709163458f20017fe085ce8e89053b17a3a8e710eaa203a96dcc83b7228c90
 89292f1b-83ac-4e96-a414-814e85c99b52 | 73491c15fd8dc77ad82a44eee0802c3faf27b639a4c08baf782f366af9f12832
```

```bash
Generate (uid, key) tuples

USAGE:
    blitz generate-keys <MASTER_KEY> [COUNT]

ARGS:
    <MASTER_KEY>    Meilisearch master key
    <COUNT>         Number of (uid, key) tuple to generate [default: 1]
```

## generate-uuids

```bash
blitz generate-uuids 3
```

```bash
8b566165-1cb5-4b8d-890a-23e0c9d670b6
cc339590-86cd-4f42-b06e-faee781cff05
b94505dd-2ed0-4c6c-a079-088e93bceb26
```

```bash
Generate uuid(s) V4

USAGE:
    blitz generate-uuids [COUNT]

ARGS:
    <COUNT>    Number of uuids V4 to generate [default: 1]
```