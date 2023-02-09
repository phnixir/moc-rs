mod moc_info;
use std::{path::PathBuf, time::Duration};

/// A big struct holding an extreme amount of data.
///
/// This struct represents all data relating to the
/// `mocp --info` command and since MoC can omit
/// certain things while outputting some fields can
/// be empty, for example, if the state is [`MocState::Stopped`]
/// the rest will be empty, for playing streams the `total_time`
/// field will be empty and there will only be a `current_time`
/// field
#[derive(Debug)]
pub struct MocInfo {
    state: MocState,
    file: MocSource,
    full_title: String,
    artist: String,
    title: String,
    album: String,
    total_time: Duration,
    current_time: Duration,
    bitrate: String,
    avg_bitrate: String,
    rate: String,
}

/// The state of MoC, can be stopped, paused or playing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MocState {
    Stopped,
    Paused,
    Playing,
}

/// The source from which MoC is playing, either a path or url
#[derive(Clone, Debug)]
pub enum MocSource {
    File(PathBuf),
    Url(String),
}

impl MocSource {
    /// Convert MocSource to a string representation regardless of if its a url or file
    ///
    /// \* This calls the the [to_str()][tostr] function of stdlib's PathBuf under the hood for
    /// files
    ///
    /// [tostr]: https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.to_str
    pub fn to_str(&self) -> Option<&str> {
        match self {
            MocSource::File(filepath) => filepath.to_str(),
            MocSource::Url(url) => Some(url.as_str()),
        }
    }
}

/// Convenience enum for use in functions from [`MocInterface`][`crate::moc::MocInterface`]
pub enum MocControl {
    Shuffle,
    Autonext,
    Repeat,
}

System.out.println("VIVY IS A BAD PROGRAMMER bitch");
