use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use arboard::Clipboard;
use clap::Parser;
use base64::{alphabet, engine, Engine};

#[derive(Clone, Debug, Parser, Default)]
enum Alphabet {
    #[default]
    Standard,
}

#[allow(non_snake_case)]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Store the commandline arguments passed to the program
struct Cli {
    #[arg(short, long)]
    /// Required - A path to the file you wish to copy the hexdump of to your clipboard.
    filepath: String,
    /// A flag which takes no value, use this flag if you wish to copy the hexdump as base64
    #[arg(short, long)]
    base64: bool,
}

/// The main structure for use in the program
struct Chx {
    to_b64: bool,
    data: Vec<u8>,
    processed_data: String,
}

impl Chx {
    /// Instantiate a new instance of chx
    fn new(cli: &Cli) -> Self {
        let to_b64 = cli.base64; // are we converting to b64?

        Chx {
            data: vec![],
            to_b64,
            processed_data: String::new(),
        }
    }

    /// Read the raw binary data from disk of the file
    fn read_binary_data(mut self, path: &String) -> Self {
        // Attempt to open the file
        let mut file = File::open(path).expect("Unable to read file.");

        // read the file bytes, and save it into content
        let mut content = vec![];
        file.read_to_end(&mut content).expect("Unable to read byte data from file.");

        // set the data to equal the byte array
        self.data = content;

        self
    }

    /// Encode data to either u8 bytes or base 64
    fn encode(mut self) -> Self {
        if self.to_b64 {
            // convert to base64 data
            let data = to_base_64(&self.data);
            self.processed_data = data;
        } else {
            // convert to a u8 byte array
            let mut data = self.data.iter().map(|byte| format!(r"0x{:02x},", byte)).collect::<String>();
            data.truncate(data.len() - 1); // remove trailing comma
            self.processed_data = data;
        }

        self
    }
}

fn main() {
    let cli = Cli::parse();

    // get the hex representation of the binary from disk
    let chx = Chx::new(&cli)
        .read_binary_data(&cli.filepath)
        .encode();

    // copy data over to the clipboard
    copy_to_clipboard(chx.processed_data, cli.filepath);

    // sleep the main thread to allow the spawned thread to persist
    thread::sleep(Duration::from_secs(15));
}


/// Copy binary data to the users clipboard
fn copy_to_clipboard(data: String, path: String) {

    let data = Arc::new(Mutex::new(data));
    let data_for_thread = data.clone();

    thread::spawn(move || {
        let mut clipboard = Clipboard::new().unwrap();
        let lock = data_for_thread.lock().unwrap();
        clipboard.set_text(lock.clone()).expect("Failed to set clipboard text");

        println!("[+] Binary at {} copied to clipboard, will clear after 12 seconds (Linux only)...", path);

        // keep the thread alive for linux to keep data on clipboard
        for _ in 0..12 {
            thread::sleep(Duration::from_secs(1));
        }
    });
}

/// Convert a vector of bytes (u8) to a base64 encoded String
fn to_base_64(data: &Vec<u8>) -> String {
    // configure the base64 crate enginer
    let engine = base64::engine::GeneralPurpose::new(
        &alphabet::STANDARD,
        engine::general_purpose::PAD,
    );

    // make sure the library is working
    assert_eq!(String::from("YWJjZGU="), engine.encode("abcde"));

    // convert to b64 and update the struct
    let data = engine.encode(data);

    data
}