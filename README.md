# Space Invaders clone

Simple game made with [bevy](https://github.com/bevyengine/bevy), implemented it after following this [tutorial series](https://www.youtube.com/watch?v=Yb3vInxzKGE) (my implementaion isn't complete yet).

### How to build & run:
At first, clone source code.
```
git clone https;//github.com/HeavyRain266/space_invaders.git
```

Now enter project directory and build it, please note that current version have hardcoded Windows-style path for assets in `setup.rs` and DX12 rendering backend in `main.rs`.

Before you build on macOS or Linux, change following:

on macOS in file `main.rs`:

```diff
- Some(Backends::DX12)
+ Some(Backends::METAL)
```

on Linux in file `main.rs`:

```diff
- Some(Backends::DX12)
+ Some(Backends::VULKAN)
```

on both in file `setup.rs`:

```diff
- server.load("actors\\ferris.png")
- server.load("actors\\gopher.png")
+ server.load("actors/ferris.png")
+ server.load("actors/gopher.png")

- server.load("lasers\\red.png")
+ server.load("lasers/red.png")
```

Now you can simply build and run game.

> Note: after building, move executable from release folder into project's root folder in order to load assets or simply run `cargo run --release` instead.

```
cd space_invaders

cargo build --release
```
