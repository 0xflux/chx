# chx - (C)lipboard (H)ex(D)umper

## Introduction 

CHX Clipboard HexDumper is a command-line tool written in Rust that allows users to read binary data of a file on disk, 
convert it to a hex dump or a base64 encoded string, and copy it to the clipboard. This tool is designed for developers 
and security experts who need to quickly transfer binary data in a readable or encoded format without manual conversion.

Works cross-platform for Windows, Linux, and MacOS.

You can either build from source, or downloads available https://github.com/0xflux/chx/releases/ - why not add it to your PATH for easy usage!

If there are any features you would like to request, please submit an Issue and I will take a look! Or, feel free
to contribute to the project!

If you like this, please reach out to me on [twitter](https://twitter.com/0xfluxsec). I also have a [Youtube](https://www.youtube.com/@FluxSec) channel and a [blog](https://fluxsec.red/) so please go check those out!

## Features

- Read binary data from specified file paths.
- Convert binary data into hex dump format.
- Optionally convert hex dump to base64 encoding.
- Copy the resulting string directly to the system clipboard.

## Example result:

Here you can see a base64 encoded string as an input, copied to the clipboard by chx. When reversing the base64 string in Cyber Chef, you can clearly see it is in fact a Windows binary.

![image](https://github.com/0xflux/chx/assets/49762827/fb135c64-4a09-48a8-850c-f9074c2bbf96)


# Getting started

## Usage

### As raw hexdump: 

For copying a direct hexdump (formatted C friendly) to your clipboard:

```shell
./chx -f /path/to/your/binary
```

An example of the output:

```C
\x4d\x5a\x90\x00\x03\x00\x00\x00\x04\x00\x00...
```

### As base64 encoded string

To copy as a base64 encoded string, add the `-b` flag:

```shell
./chx -f /path/to/your/binary -b
```

An example of the output:

```
TVqQAAMAAAAEAAAA//8AALgAAAAAAAAA...
```

For help see: `./chx --help`

## Direct download

A direct download can be found within the GitHub repo.

## Build from source

Ensure you have the Rust compiler  installed. Clone the repository to your local machine:

```bash
git clone https://github.com/0xflux/chx.git
cd chx
```

Build the project using Cargo:

```bash
cargo build --release
```
