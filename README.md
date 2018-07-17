# CRLF Converter

this is the newline converter tool.

it supports from or to 'CRLF(0x0d0a)','LF(0x0a)','CR(0x0d)'.

# Installation

TODO(cargo install?)

# Build requirements

you must install rust SDK([rustup](https://rustup.rs/) is recommended)

# Basic Usage

execute with "-h" option, you will get the following message.
```
Usage: crlfconv [OPTIONS]

Options:
    -f, --from [newline type]
                        input newline type,possible values: 'crlf'(default),
                        'cr', 'lf'
    -t, --to [newline type]
                        output newline type,possible values: 'crlf'(default),
                        'cr', 'lf'
    -i, --input default: stdin
                        input file path
    -o, --output default: stdout
                        output file path
    -h, --help          output usage

```

# Warning

this tool retrieves file as binary, so UTF-16 encoded file cannot be converted properly.
[recode_rs](https://github.com/hsivonen/recode_rs) is recommended for encoding converter.

```
recode_rs -i u16_encoded.txt -f UTF-16 -t UTF-8 | crlfconv -f crlf -t lf
```