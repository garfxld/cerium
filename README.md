# Cerium

Yet another high-performance Minecraft server library written in Rust.

> [!CAUTION]
> This repository is still in a very, very, very early stage of development.

## Goals

- High-performance
- Lightweight
- Easy to use


## Roadmap

- Protocol
    - [x] Server list ping
    - [x] Encryption
    - [x] Compression
    - [x] Joining a world *(still very basic)*
    - [x] Registries
- World
    - [ ] Blocks
    - [ ] Entities
    - [ ] Block Interactions
    - [ ] Light API
    - [ ] Chunk Generation API
- [ ] Item/Inventory API
- [ ] Commands
- [ ] Text components
- [ ] Event system (WIP)
- [ ] Resource Pack Support
- [ ] Advancements
- [ ] Proxy Support
- [ ] Scoreboards

Of course, more features are planned for the future.


## Running

```sh
cargo r --example flat_world
```


## Examples

### Debug World

<img src="thumbnail.png" width="800" alt="Debug World">