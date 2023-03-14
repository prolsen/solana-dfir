# Solana DFIR Tools

The beginning of a set of single purpose tools that can be used to perform investigations on the Solana Blockchain. 

Having said that, this is mostly a way for me to learn the Rust programming language.

# Usage

```commandline
solana-dfir.exe --help

The beginnings of a forensics toolkit for the Solana blockchain. This runs against mainnet-beta

Usage: solana-dfir.exe [OPTIONS] --addr <ADDR>

Options:
  -a, --addr <ADDR>  Wallet address
  -t, --token        Whether the address is a Token account
  -e, --earliest     Earliest transaction signature and its respective date
  -h, --help         Print help
```

#Examples

earliest is a default true, so it will always return earliest. token is default false, so you need to pass --token if you want the mint and owner returned. This will also be the same values returned if you pass an account that isn't a token account but use --token.

Passing a token account.

```commandline
solana-dfir.exe -a 2qFUz6kJ9nxLy32q2885GDNDhhrt8hXz8ohGFAR9Xm2V --token

{
  "pubkey": "2qFUz6kJ9nxLy32q2885GDNDhhrt8hXz8ohGFAR9Xm2V",
  "first_date": "2021-08-15 01:54:35 UTC",
  "first_tx": "3kvm9JQ2nebqhZgoTYbRmMF7EUy5WxfhtriKvqqDXeKWDgM2g4eSkH2YV7uJzYV5BdMdXqBpWz3EtPFdxfzkuZ5r",
  "mint": "G9HLECsin2AGae3P9ro4e9MJqKEafz2reyz4JydvmH9F",
  "owner": "AmGhEhDEjeVVbQLmw66bDvqfSC5NF1GHBLT3dw4xUphT"
}
```

Passing and account that isn't a token account, but maybe you think it is. It will return _null_ for mint and owner.

```commandline
solana-dfir.exe -a 9kFiXsZQHNpvvQb8oiN2sceBdGu81GhNRQ2MZCds8vHi
{
  "pubkey": "9kFiXsZQHNpvvQb8oiN2sceBdGu81GhNRQ2MZCds8vHi",
  "first_date": "2022-02-19 16:59:21 UTC",
  "first_tx": "4vteP45iMKuACB6zFT2t4FT32T6pEZM3SF4P9rsLf3WBcAzSt46aziFbefVkz2Hq8FQ8C5tN36ECubif4mEYFxMC",
  "mint": null,
  "owner": null
}
```

# Todo

- Transaction details (method is not working atm)