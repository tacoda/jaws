# Jaws 🦈

> A tool to manage AWS resources and run tasks.

[![CircleCI](https://circleci.com/gh/tacoda/jaws/tree/main.svg?style=svg)](https://circleci.com/gh/tacoda/jaws/tree/main)

## Usage

Help

```
jaws --help
```

Standard usage is:

```sh
jaws <service> <command> [OPTIONS]
```

Use help for more documentation with services and commands:

```sh
jaws dynamodb --help
jaws dynamodb create-table --help
```


Services and Commands currently implemented

- `dynamodb`
  - `list-tables`
- `lambda`
- `s3`

| Service | Command |
| --- | --- |
| `dynamodb` | `list-tables` \\ test |