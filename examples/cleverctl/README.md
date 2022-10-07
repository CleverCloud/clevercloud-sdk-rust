# Command line interface example

> An example of command line interface written in Rust using the clevercloud-sdk.

## Installation

To install the command line interface, you will need the same requisites that
the software development kit.

Firstly, we will clone the git repository using the following command.

```shell
$ git clone https://github.com/CleverCloud/clevercloud-sdk-rust.git
```

Then, go into the command line interface example.

```shell
$ cd clevercloud-sdk-rust/examples/cleverctl
```

Now, we are able to build the command line interface.

```shell
$ cargo build --release
```

The binary of the command line interface will be located at the following path
`target/release/cli`.

## Usage

Once, the command line interface is built, you can use it like this:

```shell
$ target/release/cleverctl -h
cleverctl 0.10.9
Command enum contains all operations that the command line could handle

USAGE:
    cleverctl [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -t, --check      Check the healthiness of the configuration
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Increase log verbosity

OPTIONS:
    -c, --config <config>    Specify a configuration file

SUBCOMMANDS:
    addon    Interact with addons
    help     Prints this message or the help of the given subcommand(s)
    self     Interact with the current user
```

## Get in touch

- [@FlorentinDUBOIS](https://twitter.com/FlorentinDUBOIS)
