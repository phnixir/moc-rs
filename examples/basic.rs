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
