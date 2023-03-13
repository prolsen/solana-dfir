# Solana DFIR Tools

The beginning of a set of single purpose. This is mostly a way for me to learn the Rust programming language.

# Usage

```commandline
solana-dfir.exe --help

The beginnings of a forensics toolkit for the Solana blockchain. This runs against mainnet-beta

Usage: solana-dfir.exe [OPTIONS] --addr <ADDR>

Options:
  -a, --addr <ADDR>  Wallet address
  -t, --token        Whether or not the address is a Token account
  -e, --earliest     Earliest transaction signature and its respective date
  -h, --help         Print help
```

# Todo

- Transaction details (method is not working atm)