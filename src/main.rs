use rodio::{source::Source, Decoder, OutputStream};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    thread,
    time::Duration,
};

fn main() {
    // Start a new thread to play the audio
    let audio_thread = thread::spawn(|| {
        // Create an audio output stream
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to get default output stream");
        let file = File::open("badapple.wav").expect("Unable to open audio file");
        let source = Decoder::new(file).expect("Failed to decode audio file");

        // Play the audio
        thread::sleep(Duration::from_micros(3450000));
        stream_handle
            .play_raw(source.convert_samples())
            .expect("Failed to play audio");

        // Keep the thread alive until the audio is done
        thread::park();
    });

    // Open the file and seek to the end to determine its size
    let mut file = File::open("badapple.txt").expect("Unable to open file");
    file.seek(SeekFrom::End(0))
        .expect("Unable to seek to end of file");
    let file_size = file.stream_position().expect("Unable to get file position");

    // Print the file size
    // println!("{}", file_size);

    // Create a buffer and read the entire file content
    let mut buffer: Vec<u8> = vec![0u8; file_size as usize];
    file.seek(SeekFrom::Start(0))
        .expect("Unable to seek to the beginning of file");
    file.read(&mut buffer).expect("Unable to read the file");

    // Convert buffer to string and split the content based on the delimiter "nekomark"
    let content = String::from_utf8(buffer).expect("File contains invalid UTF-8");

    let frames: Vec<&str> = content.split("nekomark").collect();

    for frame in frames {
        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", frame);
        thread::sleep(Duration::from_micros(130000));
    }

    audio_thread.thread().unpark();
}
