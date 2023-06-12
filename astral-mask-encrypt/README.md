# Astral Mask Encrypt

Astral Mask Encrypt is a simple command line tool to encrypt and decrypt string using a key.

## Usage

An easy way to get help information is to run `astral-mask-encrypt` without any arguments.

```bash
astral-mask-encrypt
```

```
Astral Mask Encrypt Cli

Usage: astral-mask-encrypt <COMMAND>

Commands:
  key-encrypt   Encrypt a string with a password and seed
  hash-encrypt  Encrypt a string with a hash
  mint-key      Generate the hash which is used to encrypt a string. Never save this hash anywhere!
  key-decrypt   Decrypt a string with a password and seed
  hash-decrypt  Decrypt a string with a hash
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For each command, you can get more information by running `astral-mask-encrypt <COMMAND> --help`.

```bash
astral-mask-encrypt key-encrypt --help
```

## Key Generation Algorithm

First, encode the password provided by the user in utf8 as bytes. This byte string will be repeated `seed` times as the starting point of the hash loop.

In the hash loop, the output of the last time is used as the input of this time for sha512. The loop will continue until the first `zero_height` bits are all 0.

Computation time cannot be reduced by multithreaded parallel computation and dynamic planning.