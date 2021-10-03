# Jaws 🦈

> A tool to manage AWS resources and run tasks.

[![CircleCI](https://circleci.com/gh/tacoda/jaws/tree/main.svg?style=svg)](https://circleci.com/gh/tacoda/jaws/tree/main)

## Usage

Standard usage is:

```sh
jaws <service> <command> [OPTIONS]
```

Use help for more documentation with services and commands:

```sh
jaws --help
jaws dynamodb --help
jaws dynamodb create-table --help
```


**Services and Commands currently implemented**

| Service | Command |
| --- | --- |
| `dynamodb` | `list-tables` `create-table` `delete-table` `put-item` |
| `s3` | `list-buckets` |
| `lambda` | `list-functions` |