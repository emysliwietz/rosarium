use std::{error::Error, path::Path, sync::mpsc::Receiver, thread, time::Duration};

use soloud::{AudioExt, Handle, LoadExt, Soloud, Wav};

use crate::tui::ErrorString;

pub enum AudioCommand {
    Play(String),
    Pause,
    SetVolume(f32),
}

pub fn audio_thread(mut rx: Receiver<AudioCommand>) -> Result<(), Box<dyn Error>> {
    thread::Builder::new()
        .name("rosarium - audio".to_string())
        .spawn(move || {
            let mut sl = Soloud::default();
            if sl.is_err() {
                return;
            }
            let mut sl = sl.unwrap();
            sl.set_global_volume(1.0);
            loop {
                let cmd = rx.recv();
                if cmd.is_err() {
                    return;
                }
                match cmd.unwrap() {
                    AudioCommand::Play(s) => {
                        let audio = play_audio(&mut rx, &mut sl, s);
                        if audio.is_err() {
                            return;
                        }
                    }
                    AudioCommand::Pause => {
                        sl.set_pause_all(true);
                    }
                    AudioCommand::SetVolume(v) => {
                        sl.set_global_volume(v);
                    }
                };
            }
        });
    Err(Box::new(ErrorString::Error(
        "Audio player stopped unexpectedly",
    )))
}

pub fn play_audio(
    rx: &mut Receiver<AudioCommand>,
    sl: &mut Soloud,
    s: String,
) -> Result<(), Box<dyn Error>> {
    let mut wav = Wav::default();
    wav.load(&Path::new(&s));
    let h = sl.play(&wav);
    while sl.voice_count() > 0 {
        let cmd = rx.recv()?;
        match cmd {
            AudioCommand::Pause => {
                fade_audio(sl, h);
                break;
            }
            AudioCommand::Play(n) => {
                return fade_to(s, n, rx, sl, h);
            }
            AudioCommand::SetVolume(v) => sl.set_global_volume(v),
        }
    }
    Ok(())
}

fn fade_to(
    old: String,
    new: String,
    rx: &mut Receiver<AudioCommand>,
    sl: &mut Soloud,
    h: Handle,
) -> Result<(), Box<dyn Error>> {
    fade_audio(sl, h);
    if old != new {
        return play_audio(rx, sl, new);
    }
    Ok(())
}

fn fade_audio(sl: &mut Soloud, h: Handle) -> Result<(), Box<dyn Error>> {
    for i in 0..100 {
        sl.set_volume(h, 1.0 - (i as f32 * 0.01));
        std::thread::sleep(Duration::from_millis(10));
    }
    sl.destroy_voice_group(h);
    Ok(())
}
