Discover the future value of the key field of a Meilisearch API key before its generation to automatize deployments from the ground-up.

## Overview

```bash
SUBCOMMANDS:
    generate-keys     Generate keys field value for a list of UIDs and a master key
    generate-uuids    Generate uuid(s) V4
    help              Print this message or the help of the given subcommand(s)
```

## generate-keys

```bash
blitz generate-keys master_key d7d30ffe-ec60-484f-84f8-1c8b7d0ac352 c5a18797-621c-42b5-81bd-23fbf0202364
```

```bash
 uid                                  | 🔑 key
 d7d30ffe-ec60-484f-84f8-1c8b7d0ac352 | h2mHLjq0eu9N6xO4padE3ZRbToQgi1X7IPb7ePQdvHY
 c5a18797-621c-42b5-81bd-23fbf0202364 | MeU0XSwYywmYnMNUJpg5Degyqk_QqKIWQcyIDf4Z-YM
 ```

```bash
Generate keys field value for a list of UIDs and a master key

USAGE:
    blitz generate-keys <MASTER_KEY> [UIDS]...

ARGS:
    <MASTER_KEY>    Meilisearch master key
    <UIDS>...       API key uids
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