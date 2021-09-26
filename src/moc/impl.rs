use crate::common::{MocControl, MocInfo, MocSource, MocState};
use crate::moc::{Moc, MocInterface};
use std::{collections::HashMap, path::Path, process::Command, time::Duration};

impl MocInterface for Moc {
    fn new(moc_path: String) -> Self {
        Moc { moc_path }
    }

    fn info(&self) -> MocInfo {
        let output = Command::new(self.moc_path.clone())
            .arg("--info")
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            panic!("server is not running"); // FIXME: return error instead of panicking
        }

        let info = String::from_utf8_lossy(&output.stdout);
        let info = info.trim();
        let mut infohmp: HashMap<String, String> = HashMap::new();
        for i in info.split('\n') {
            let kv = i.split(": ");
            infohmp.insert(
                kv.clone().next().unwrap().into(),
                kv.clone().nth(1).unwrap_or("").into(),
            );
        }

        let mocstate = match infohmp["State"].as_str() {
            "STOP" => MocState::Stopped,
            "PAUSE" => MocState::Paused,
            "PLAY" => MocState::Playing,
            _ => panic!("UNIMPLEMENTED MOCSTATE! CRASH AND BURN!!!!!"), // FIXME: make it return a result instead of panic
        };

        let mut moc_info;
        if mocstate == MocState::Stopped {
            moc_info = MocInfo::default().with_state(MocState::Stopped);
        } else {
            let file = infohmp["File"].clone();
            let mocsource;
            if file.starts_with("http") {
                mocsource = MocSource::Url(file);
            } else {
                mocsource = MocSource::File(file.into());
            }

            let title = infohmp["Title"].clone();
            let artist = infohmp["Artist"].clone();
            let song_title = infohmp["SongTitle"].clone();
            let album = infohmp["Album"].clone();
            let current_time = infohmp["CurrentSec"].clone();
            let bitrate = infohmp["Bitrate"].clone();
            let avg_bitrate = infohmp["AvgBitrate"].clone();
            let rate = infohmp["Rate"].clone();

            moc_info = MocInfo::default()
                .with_state(mocstate)
                .with_file(mocsource)
                .with_full_title(title)
                .with_artist(artist)
                .with_title(song_title)
                .with_album(album)
                .with_current_time(Duration::from_secs(current_time.parse::<u64>().unwrap()))
                .with_bitrate(bitrate)
                .with_avg_bitrate(avg_bitrate)
                .with_rate(rate);

            if infohmp.contains_key("TotalSec") {
                moc_info = moc_info.with_total_time(Duration::from_secs(
                    infohmp["TotalSec"].parse::<u64>().unwrap(),
                ));
            } else {
                moc_info = moc_info.with_total_time(Duration::from_secs(0));
            }
        }

        moc_info
    }

    fn clear_playlist(&mut self) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--clear")
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn play_playlist(&mut self) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--play")
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn immediate_play(&mut self, source: MocSource) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--playit")
            .arg(source.to_str().unwrap())
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn stop_server(&mut self) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--stop")
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn next_song(&mut self) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--next")
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn previous_song(&mut self) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--previous")
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn pause_playback(&mut self) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--pause")
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn resume_playback(&mut self) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--unpause")
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn set_volume(&mut self, vol: usize) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--volume")
            .arg(format!("{}", vol))
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn increase_volume_by(&mut self, step: usize) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--volume")
            .arg(format!("+{}", step))
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn decrease_volume_by(&mut self, step: usize) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--volume")
            .arg(format!("-{}", step))
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn append_music(&mut self, path: &Path) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--append")
            .arg(path.to_str().unwrap())
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn seek(&mut self, step: isize) -> bool {
        let mut sign = String::from("");
        if step.signum() == 1 {
            sign.push('+')
        }

        let status = Command::new(self.moc_path.clone())
            .arg("--seek")
            .arg(format!("{}{}", sign, step))
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn jump_to(&mut self, time: Duration) -> bool {
        let status = Command::new(self.moc_path.clone())
            .arg("--jump")
            .arg(format!("{}s", time.as_secs()))
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn enable_control(&mut self, control: MocControl) -> bool {
        let string = match control {
            MocControl::Autonext => "autonext",
            MocControl::Repeat => "repeat",
            MocControl::Shuffle => "shuffle",
        };
        let status = Command::new(self.moc_path.clone())
            .arg("--on")
            .arg(string)
            .status()
            .expect("failed to run process");

        status.success()
    }
    fn disable_control(&mut self, control: MocControl) -> bool {
        let string = match control {
            MocControl::Autonext => "autonext",
            MocControl::Repeat => "repeat",
            MocControl::Shuffle => "shuffle",
        };
        let status = Command::new(self.moc_path.clone())
            .arg("--off")
            .arg(string)
            .status()
            .expect("failed to run process");

        status.success()
    }
}
