use crate::common::{MocInfo, MocSource, MocState};
use std::{path::PathBuf, time::Duration};

#[allow(dead_code)]
impl MocInfo {
    /// Get the state from a [`MocInfo`][`crate::common::MocInfo`] struct, all the other methods
    /// that **don't** have the parameter called `with` do the same for other fields aswell.
    pub fn state(&self) -> MocState {
        self.state
    }
    /// Builder-style function to compose a [`MocInfo`][`crate::common::MocInfo`] struct, this and
    /// all the other functions that **take** a `with` parameter can also be used the same way...
    ///
    /// For example:
    /// ```rust
    /// use moc_rs::common::{MocInfo, MocState};
    ///
    /// let mocinfo = MocInfo::default()
    ///     .with_state(MocState::Playing)
    ///     .with_full_title("Wayfinder & Phace - Hymn (-)".into())
    ///     .with_artist("Wayfinder & Phace".into())
    ///     .with_title("Hymn".into())
    ///     .with_album("-".into());
    ///
    /// // All other values will be left as default in this case
    ///
    /// ```
    pub fn with_state(mut self, with: MocState) -> Self {
        self.state = with;
        self
    }
    pub fn file(&self) -> MocSource {
        self.file.clone()
    }
    pub fn with_file(mut self, with: MocSource) -> Self {
        self.file = with;
        self
    }
    pub fn full_title(&self) -> String {
        self.full_title.clone()
    }
    pub fn with_full_title(mut self, with: String) -> Self {
        self.full_title = with;
        self
    }
    pub fn artist(&self) -> String {
        self.artist.clone()
    }
    pub fn with_artist(mut self, with: String) -> Self {
        self.artist = with;
        self
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn with_title(mut self, with: String) -> Self {
        self.title = with;
        self
    }
    pub fn album(&self) -> String {
        self.album.clone()
    }
    pub fn with_album(mut self, with: String) -> Self {
        self.album = with;
        self
    }
    pub fn total_time(&self) -> Duration {
        self.total_time
    }
    pub fn with_total_time(mut self, with: Duration) -> Self {
        self.total_time = with;
        self
    }
    pub fn current_time(&self) -> Duration {
        self.current_time
    }
    pub fn with_current_time(mut self, with: Duration) -> Self {
        self.current_time = with;
        self
    }
    pub fn bitrate(&self) -> String {
        self.bitrate.clone()
    }
    pub fn with_bitrate(mut self, with: String) -> Self {
        self.bitrate = with;
        self
    }
    pub fn avg_bitrate(&self) -> String {
        self.avg_bitrate.clone()
    }
    pub fn with_avg_bitrate(mut self, with: String) -> Self {
        self.avg_bitrate = with;
        self
    }
    pub fn rate(&self) -> String {
        self.rate.clone()
    }
    pub fn with_rate(mut self, with: String) -> Self {
        self.rate = with;
        self
    }
}

impl Default for MocInfo {
    fn default() -> Self {
        MocInfo {
            state: MocState::Stopped,
            file: MocSource::File(PathBuf::default()),
            full_title: String::default(),
            artist: String::default(),
            title: String::default(),
            album: String::default(),
            total_time: Duration::from_secs(0),
            current_time: Duration::from_secs(0),
            bitrate: String::default(),
            avg_bitrate: String::default(),
            rate: String::default(),
        }
    }
}
