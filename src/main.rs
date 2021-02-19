use iced::{Application, Settings};
use simp::{deck::*, gui::*};
use std::{env, fs};

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    let f = &args[1];
    let content = fs::read(f).expect(&format!("failed to open slide file {}", f));
    let input = String::from_utf8_lossy(&content);

    let deck = parse_deck(&input).expect(&format!("failed to parse slide file {}", f));

    println!("{:?}", &deck);

    DeckViewer::run(Settings {
        antialiasing: true,
        flags: deck.1,
        ..Settings::default()
    })
}
