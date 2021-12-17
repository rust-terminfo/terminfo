terminfo
========

Terminal capabilities with type-safe getters.

Usage
-----
Add this to your `Cargo.toml`:

```toml
[dependencies]
terminfo = "0.1"
```

and this to your crate root:

```rust
extern crate terminfo;
```

Example
-------
```rust
extern crate terminfo;

use std::io::{self, Write};
use terminfo::{Expand, Database, capability as cap};

fn main() {
  let info = Database::from_env().unwrap();

  if let Some(cap::MaxColors(n)) = info.get::<cap::MaxColors>() {
    println!("The terminal supports {} colors.", n);
  }
  else {
    println!("The terminal does not support colors, what year is this?");
  }

  if let Some(flash) = info.get::<cap::FlashScreen>() {
    io::stdout().write_all(&flash.expand(&[], &mut Default::default()).unwrap()).unwrap();
  }
  else {
    println!("FLASH GORDON!");
  }
}
```

Packaging and Distributing
--------------------------
For all terminals but windows consoles, this library depends on a non-hashed
(for now) terminfo database being present. For example, on Debian derivitives,
you should depend on ncurses-term; on Arch Linux, you depend on ncurses; and on
MinGW, you should depend on mingw32-terminfo.

Unfortunately, if you're using a non-windows console on Windows (e.g. MinGW,
Cygwin, Git Bash), you'll need to set the TERMINFO environment variable to
point to the directory containing the terminfo database.

License
-------

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
