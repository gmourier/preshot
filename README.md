Discover the future value of the key field of a Meilisearch API key before its generation to automatize deployments from the ground-up.

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
 d7d30ffe-ec60-484f-84f8-1c8b7d0ac352 | h2mHLjq0eu9N6xO4padE3ZRbToQgi1X7IPb7ePQdvHY
 c5a18797-621c-42b5-81bd-23fbf0202364 | MeU0XSwYywmYnMNUJpg5Degyqk_QqKIWQcyIDf4Z-YM
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
 672383ed-13a2-49b8-85ac-38e4ad0d2e15 | _jCVH0PtEc0peg-Udi4a7kuSQtVb4tSVyAauN7XIrRA
 8e996bfa-e45f-4a3a-8822-32cdba1a4d08 | 2RUx5uZTakePmRIA7jIe8h5JbL3oqxhItt3M5_Br2w8
 3cda0331-08ef-4a86-b3bc-8cdaffaf9536 | 6TNv-vbomWl1Trs35qiOqQzeekhyPncZLnYbwYNq73g
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