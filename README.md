# Dynamic NFT

Dynamic NFTs (dynNFTs) are a unique type of tokens that bridge the gap between classical NFTs and badges. dynNFT are non-fungible tokens in which metadata are shared across each token like in badges. The difference here is that each collection owns different metadata and the metadata of a single NFT can be turned in a specific timespan.

The inception of the Dynamic NFT project took place during HackWasm Berlin 2023.

## Spec

You can find them [here](./spec/README.md)

## Getting Started

These instructions will help you get a copy of the smart contract up and running on your local machine for development and testing purposes.

### Prerequisites

- [CosmWasm](https://github.com/CosmWasm/cosmwasm)
- Rust: [Installation Guide](https://www.rust-lang.org/tools/install)
- Command runner: [just](https://github.com/casey/just)

### Installation

1. Clone the repository:

    ```shell
    git clone https://github.com/kintsugi-tech/dyn-nft.git
    ```

2. Change into the project directory:

    ```shell
    cd dyn-nft
    ```

3. Build the smart contract:

    ```shell
    just optimize
    ```

## Description

The project is composed of two contracts.

### Dyn-Nft

This is our wrapper around sg-721, which itself is a custom implementation of cw-721.

### Factory

This is the factory used to instantiate collections of dyn-nfts.

### common

This package contains common types shared across the workspace's contracts.

## Usage

Describe how to deploy and interact with the smart contract. Provide examples and explanations of the available functionality.

### Test

```shell
just test
```

### Lint

```shell
just clippy && just fmt 
```

## Contributing

If you want to contribute to this project please, remember to run the following command before making a PR:

```shell
just default-flow
```

## References

This project has been inspired by the following codes:

- [sg721-base](https://github.com/public-awesome/launchpad/tree/main/contracts/collections/sg721-base)

- [badges](https://github.com/public-awesome/badges)

## License

This project is licensed under the MIT License - see the LICENSE file for details.
