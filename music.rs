use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::Source;


pub struct MusicPlayer;

impl MusicPlayer {
    pub fn play_music_on_startup(file_path: &str) {
        let file_path = file_path.to_owned();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(3)); 

            if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(file) = File::open(Path::new(&file_path)) {
                    if let Ok(decoder) = Decoder::new(BufReader::new(file)) {
                        let looping_source = decoder.repeat_infinite();
                        if let Ok(sink) = Sink::try_new(&stream_handle) {
                            sink.append(looping_source);
                            sink.sleep_until_end();
                        } else {
                            eprintln!("Failed to create sink");
                        }
                    } else {
                        eprintln!("Failed to create decoder");
                    }
                } else {
                    eprintln!("Failed to open file: {}", file_path);
                }
            } else {
                eprintln!("Failed to get output stream");
            }
        });
    }
}


   


