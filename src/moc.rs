use crate::common::{MocControl, MocInfo, MocSource};
use std::{path::Path, time::Duration};
mod r#impl;

/// A struct holding the path to the MoC executable.
pub struct Moc {
    pub moc_path: String,
}

/// **The** trait for most of this crate's functionality, it exposes all the functions
/// to interact with MoC.
pub trait MocInterface {
    fn new(moc_path: String) -> Self;
    /// Get all the info about what MoC is currently doing, refer to
    /// [`MocInfo`][`crate::common::MocInfo`]'s implementations for
    /// the methods to get each field
    fn info(&self) -> MocInfo;

    /// Clear MoC's current playlist.
    fn clear_playlist(&mut self) -> bool;
    /// Play MoC's current playlist starting from the first item.
    fn play_playlist(&mut self) -> bool;
    /// Immediately play the `source`, it can be a file or stream url
    /// but it has to be constructed using [`MocSource`][`crate::common::MocSource`].
    fn immediate_play(&mut self, source: MocSource) -> bool;
    /// Literally just `game end` MoC.
    fn stop_server(&mut self) -> bool;
    /// Skip to the next song.
    fn next_song(&mut self) -> bool;
    /// Go back to the previous song.
    fn previous_song(&mut self) -> bool;
    /// Pause playback.
    fn pause_playback(&mut self) -> bool;
    /// Resume playback.
    fn resume_playback(&mut self) -> bool;
    /// Set the volume for MoC.
    fn set_volume(&mut self, vol: usize) -> bool;
    /// Instead of setting the volume, increase the volume by the `step`.
    fn increase_volume_by(&mut self, step: usize) -> bool;
    /// Same as above but for decreasing.
    fn decrease_volume_by(&mut self, step: usize) -> bool;
    /// Append a music **file** to the playlist, doesn't support streams...
    ///
    /// You could however clear the current playlist, write the stream url to
    /// an `m3u` file and load that file.
    fn append_music(&mut self, path: &Path) -> bool;
    /// Seek forwards and backwards on the currently playing track, this **doesn't**
    /// jump to that duration, it seeks, therefore if you give the step the value of
    /// `-10` it seeks ten seconds back, for the value of `10` it seeks ten seconds
    /// forwards
    fn seek(&mut self, step: isize) -> bool;
    /// Jump to a location in the current track.
    fn jump_to(&mut self, time: Duration) -> bool;
    /// Enable one of the flags, refer to [`MocControl`][`crate::common::MocControl`]
    fn enable_control(&mut self, control: MocControl) -> bool;
    /// Disable one of the flags, refer to [`MocControl`][`crate::common::MocControl`]
    fn disable_control(&mut self, control: MocControl) -> bool;
}
