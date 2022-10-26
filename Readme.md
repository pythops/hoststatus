# hoststatus
**hoststatus** is a simple cli to check website status.

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
|  hostname   | status |     expiration date      | infos |
+-------------+--------+--------------------------+-------+
| pythops.com |   UP   | Nov 26 17:25:25 2022 GMT |       |
+-------------+--------+--------------------------+-------+
```

## Note
The code quality is not optimal and I know it, I'm still learning Rust, so it will be improved at some point.

## License
GPLv3
