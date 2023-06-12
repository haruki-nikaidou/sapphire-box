# Astral Mask

A super safe and super slow password manager.

*This project is a work in progress.*

## Introduction

With this tool, you can store your passwords on the blockchain safely. The passwords are encrypted with a key that only you know.

This project uses a very slow key generation algorithm, which can take several hours per key generation. The key is generated from a password and a private seed you provide.

Because it uses such a slow key generation algorithm, it is inherently resistant to dictionary attack and brute-force cracking attack, and even to some extent to social engineering attack.

You can specify time control parameters `h` to control the time spent by the key generation algorithm. The time complexity of key generation algorithm is `O(2^h)` in average.

`astral-mask-encrypt` generates the key and encrypts the data with `threefish` algorithm. The encrypted data is stored on the blockchain using `astral-mask-storage` smart contract.

To protect your passwords. You should use a unique complex unapproachable password. You should also use a unique private seed. Each key is generated from the password and the private seed. If you use the same password and private seed, you will get the same key.

## Structure

The project contains 3 main parts:

+ `astral-mask-encrypt` - a command line tool to encrypt and decrypt string using a key.
+ `astral-mask-storage` - a smart contract to store encrypted data on the blockchain.
+ `astral-mask-ui` - a web application to manage encrypted data.
