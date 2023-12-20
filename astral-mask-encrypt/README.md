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
  encrypt  Encrypt a string with a Combination Lock
  decrypt  Decrypt a string which was encrypted with a Combination Lock
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For each command, you can get more information by running `astral-mask-encrypt <COMMAND> --help`.

```bash
astral-mask-encrypt key-encrypt --help
```

## Key Generation Algorithm

This algorithm is designed to generate cryptographic keys based on specified criteria for leading or trailing zeros in the hash output. 

It starts by hashing an initial byte vector using SHA-512, and then iteratively modifies this hash according to a given set of conditions (Combination Lock), producing a series of keys.