# hoststatus

**hoststatus** is a simple cli to check hostname status.

## Usage

```
$ cargo run -- --help
Usage: hoststatus [--host <host>] [--path <path>]

Simple cli to check hosts status

Options:
  --host            hostname (example: google.com)
  --path            path to file containing hostnames
  --help            display usage information

```

## Example

```
$ cargo run -- --host pythops.com
+-------------+--------+--------------------------+-------+
|  hostname   | status | TLS cert expiration date | infos |
+-------------+--------+--------------------------+-------+
| pythops.com |   UP   | Nov 26 17:25:25 2022 GMT |       |
+-------------+--------+--------------------------+-------+
```

## License

GPLv3
