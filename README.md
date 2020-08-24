# Peppi

Peppi is a Rust parser for [.slp](https://github.com/project-slippi/slippi-wiki/blob/master/SPEC.md) game replay files for [Super Smash Brothers Melee](https://en.wikipedia.org/wiki/Super_Smash_Bros._Melee) for the Nintendo GameCube. These replays are generated by Jas Laferriere's [Slippi](https://github.com/JLaferri/project-slippi) recording code, which runs on a Wii or the [Dolphin](https://dolphin-emu.org/) emulator.

Peppi is fairly full-featured already, but still under active development. **APIs are still subject to breaking change** until version 1.0 is released.

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
peppi = { git = "https://github.com/hohav/peppi" }
```

## Usage

### Object-based parsing:

```rust
use std::path::Path;

fn main() {
	let path = Path::new("game.slp");
	let game = peppi::game(path).unwrap();
	println!("{:#?}", game);
}
```

### Event-driven parsing:

```rust
use std::{fs, io};

use peppi::parse::{Handlers, FrameEvent};
use peppi::frame;

struct FramePrinter {}

impl Handlers for FramePrinter {
	fn frame_post(&mut self, post: FrameEvent<frame::Post>) -> io::Result<()> {
		println!("{}:{}", post.id.port, post.id.index);
		Ok(())
	}
}

fn main() {
	let f = fs::File::open("game.slp").unwrap();
	let mut r = io::BufReader::new(f);
	let mut handlers = FramePrinter {};

	peppi::parse(&mut r, &mut handlers).unwrap();
}
```

## Inspector

Compiling Peppi by itself will give you a binary called `slp` that's useful for inspecting replays. For example, this command shows you the post-frame action state for each port (player) on the last frame of the game:

```bash
$ slp -nq frames[-1].ports[].leader.post.state game.slp
14:WAIT
1:DEAD_LEFT
```

Run `slp --help` for more info.
