extern crate chrono;
extern crate timer;

use std::sync::mpsc::channel;

use clap::Parser;

use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

/// Simple CLI meditation timer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// How many minutes to set the timer for
    #[arg(short, long)]
    minutes: i64,
}

const AUDIO_LENGTH_SECONDS: i64 = 26;
const AUDIO_FILE_PATH: &str = "audio/bell.ogg";

fn main() {
    let args = Args::parse();
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let delay = chrono::Duration::minutes(args.minutes) - chrono::Duration::seconds(AUDIO_LENGTH_SECONDS);

    invite();

    let _guard = timer.schedule_with_delay(delay, move || {
        invite();
        let _ignored = tx.send(());
    });

    rx.recv().unwrap();
    println!(
        "{0} minute meditation complete 🧘‍♂️",
        args.minutes
    );
}

fn invite() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(AUDIO_FILE_PATH).unwrap());
    let source = Decoder::new(file).unwrap(); 
    let _ = stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(AUDIO_LENGTH_SECONDS as u64));
}
