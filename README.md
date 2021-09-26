# moc-rs
This library helps you interact with the MoC (Music on Console) server running on 
your system thru the binary, which in the case of arch linux (and manjaro) is 
installable thru pacman
```
sudo pacman -S moc
```
or for debian and ubuntu
```
sudo apt install moc
```

⚠️ The currently tested and working version of MoC is v2.5.2.

If you can't find an up to date version of MoC in your distributions 
repositories (highly unlikey) please check out the
[download](https://moc.daper.net/download) section of the official website

## Basic example
```rust
use moc_rs::{Moc, MocInterface, MocSource};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make a new instance of Moc
    let mut moc_ctx = Moc::new("mocp".into());

    // Get info about the current state of MoC
    let mocinfo = moc_ctx.info();
    dbg!(mocinfo);

    moc_ctx.set_volume(50);

    // A little delay to let the last task execute
    std::thread::sleep(Duration::from_millis(100));

    // Begin streaming SceneSat Radio immediately
    moc_ctx.immediate_play(MocSource::Url(
        "http://sentinel.scenesat.com:8000/scenesat".into(),
    ));

    // Remember to check out the docs for more info!

    Ok(())
}
```
For more examples check out the [examples/][examples] directory on github

[examples]: https://github.com/phnixir/moc-rs

## Contributing
Thanks for your interest in contributing! please open an issue or merge request
to contibute. Code contributions submitted for inclusion in the work by you, as
defined in the MPL2.0 license, shall be licensed as the above without any
additional terms or conditions.


## License
This project is licenced under [MPL 2.0][license].

[license]: https://www.mozilla.org/en-US/MPL/2.0/
