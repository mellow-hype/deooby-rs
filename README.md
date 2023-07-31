
# deooby

Simple program to remove OOB blocks from NAND dumps (or anything else that uses a repeated pattern of [x-byte block][y-byte block], where the x-byte blocks are the ones to be extracted.
You must provide proper OOB and page size values; we don't validate all that.

```

Usage: deooby [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>            path to the target file
  -o, --oob-size <OOB_SIZE>    OOB data size [default: 64]
  -p, --page-size <PAGE_SIZE>  Page size [default: 2048]
  -h, --help                   Print help
  -V, --version                Print version
```
