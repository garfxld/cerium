# Cerium

Yet another high-performance Minecraft server library written in Rust.

> [!CAUTION]
> This repository is still in a very, very, very early stage of development. I'm a student so don't expect frequent updates.


## Goals

- High-performance
- Lightweight
- Easy to use


## Usage

The project is still very experimental so you would need to add it as a git dependency.
To add it to you project add the following line in your `Cargo.toml`:
```
cerium = { git = "https://github.com/garfxld/cerium.git" }
```

You can use one of the examples to quick-start a project.


## Examples

There are some simple (and maybe not fully functional) examples in the [examples](/cerium/examples/) directory in the `cerium` crate.

### List of Examples

- [debug_world.rs](/cerium/examples/debug_world.rs)
- [flat_world.rs](/cerium/examples/flat_world.rs)
- [inventory.rs](/cerium/examples/inventory.rs)
- [npc.rs](/cerium/examples/npc.rs)
- [text.rs](/cerium/examples/text.rs)


### Running

```sh
cargo r --example debug_world
```

```rust
fn main() {
    let server = Server::new();

    let world = World::new(&DimensionType::OVERWORLD);

    for (ix, pos) in (0..27946).enumerate() {
        let bz = (pos / 168) + 1;
        let bx = (pos % 168) + 1;

        let block = BlockState::from_id(ix as i32).unwrap();
        world.set_block((bz * 2) - 1, 70, (bx * 2) - 1, block);
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position((0.5, 71., 0.5));
        });

    server.bind("127.0.0.1:25565").unwrap();
}

```

<img src="thumbnail.png" alt="Debug World">


## Roadmap

- Protocol
    - [x] Server List Ping
    - [x] Encryption
    - [x] Compression
    - [x] Joining a World
    - [x] Registries
    - [ ] All Packets
- World
    - [ ] Blocks
    - [ ] Entities
    - [ ] Block Interactions
    - [ ] Light API
    - [ ] Chunk Generation API
    - [ ] Batching
- Entity
    - [ ] Entity API
    - [ ] Entity Metadata
- Inventory/Item
    - [x] Open Inventory
    - [x] Close Inventory
    - [x] Set Slot Content
    - [ ] Click Slot
    - [x] Create ItemStack
- [x] Text components
- [ ] Command System
- [ ] Event System
- [ ] Resource Pack Support
- [ ] Advancements
- [ ] Proxy Support
- [ ] Scoreboards

Of course, more features are planned for the future.
 