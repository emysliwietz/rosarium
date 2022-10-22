use std::{path::Path, sync::mpsc::Receiver, thread, time::Duration};

use soloud::{AudioExt, Handle, LoadExt, Soloud, WavStream};

#[derive(PartialEq, Eq)]
pub enum AudioCommand {
    Play(String),
    Pause,
}

pub fn audio_thread(rx: Receiver<AudioCommand>) {
    thread::Builder::new()
        .name("rosarium - audio".to_string())
        .spawn(move || {
            let mut sl = Soloud::default().expect("Error initializing audio");
            sl.set_global_volume(1.0);
            loop {
                let cmd = rx.recv().expect("Garbled audio command");
                match cmd {
                    AudioCommand::Play(s) => {
                        let mut wav = WavStream::default();
                        wav.load(&Path::new(&s));
                        let h = sl.play(&wav);
                        while sl.voice_count() > 0 {
                            let cmd = rx.recv().expect("Garbled audio command");
                            if cmd == AudioCommand::Pause {
                                fade_audio(&mut sl, h);
                                break;
                            }
                            std::thread::sleep(std::time::Duration::from_millis(100));
                        }
                    }
                    AudioCommand::Pause => {
                        sl.set_pause_all(true);
                    }
                }
            }
        });
}

fn fade_audio(sl: &mut Soloud, h: Handle) {
    for i in 0..100 {
        sl.set_volume(h, 1.0 - (i as f32 * 0.01));
        std::thread::sleep(Duration::from_millis(10));
    }
    sl.destroy_voice_group(h);
}
