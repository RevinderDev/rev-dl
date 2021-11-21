use std::collections::HashMap;
use std::env::{current_dir, set_current_dir};
use std::fs::create_dir_all;
use std::io::{self, stdout};
use std::process::Command;
use std::time::Instant;
use std::{fmt::Debug, fs, vec};

use clap::{App, Arg};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

#[derive(Debug)]
struct Artist {
    songs: Vec<String>,
    name: String,
}

impl Default for Artist {
    fn default() -> Artist {
        Artist {
            songs: vec![],
            name: String::from("unnamed"),
        }
    }
}

fn get_artists(playlist_file: String) -> HashMap<String, Artist> {
    let mut artists = HashMap::new();
    for line in playlist_file
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
    {
        let first_char = line.chars().next().unwrap();
        if first_char == '#' {
            continue;
        }
        let mut iter = line.splitn(2, '-');
        let artist_name = iter.next();
        let artist = artists
            .entry(String::from(artist_name.unwrap()))
            .or_insert(Artist {
                name: String::from(artist_name.unwrap()),
                ..Default::default()
            });
        artist.songs.push(String::from(line));
    }
    artists
}

fn download_songs(artists: HashMap<String, Artist>, output_path: &str) -> Result<(), io::Error> {
    let path = if output_path == "." {
        fs::canonicalize(current_dir()?)?
    } else {
        fs::canonicalize(output_path)?
    };
    set_current_dir(&path).expect("Couldn't find output path.");
    for artist in artists.into_values() {
        create_dir_all(&artist.name)?;
        set_current_dir(&artist.name)?;
        println!("🎤 Artist: {}", artist.name);
        let downloaded_songs: Vec<String> = fs::read_dir(".")
            .unwrap()
            .map(|file| String::from(file.unwrap().file_name().to_str().unwrap()))
            .collect();
        for song in artist.songs {
            execute!(
                stdout(),
                SetForegroundColor(Color::White),
                Print(format!("\t🎶 Downloading '{}' 🎶\n", song)),
                ResetColor
            )?;

            if let Some(found) = downloaded_songs.iter().find(|&s| {
                (*s).to_ascii_lowercase()
                    .contains(&song.to_ascii_lowercase())
            }) {
                // TODO: Fix these for some normal coloring calls because my god is it too much.
                execute!(
                    stdout(),
                    SetForegroundColor(Color::DarkYellow),
                    Print(format!(
                        "\t⚠️Found duplicate song from file. Skipping..⚠️\n\t`{}` \n",
                        found
                    )),
                    ResetColor
                )?;
                continue;
            }
            let now = Instant::now();
            Command::new("youtube-dl")
                .arg("--extract-audio")
                .arg("--audio-format")
                .arg("mp3")
                .arg(format!("ytsearch1:{}", song))
                .output()
                .expect("failed to execute process");

            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!(
                    "\t✅ Downloaded in {}s.\n ",
                    now.elapsed().as_secs()
                )),
                ResetColor
            )?;
        }
        set_current_dir(&path)?;
    }
    Ok(())
}

fn main() {
    let matches = App::new("RevDL")
        .version("0.1.0")
        .author("Michał K. <michal0kasprzyk@gmail.com>")
        .about("Download playlists using Youtube-DL and FFMpeg!")
        .arg(
            Arg::new("playlist")
                .short('p')
                .long("playlist")
                .value_name("FILE")
                .required(true)
                .about("Playlist to download from, see examples.")
                .takes_value(true),
        )
        .arg(
            Arg::new("output_path")
                .short('o')
                .long("output")
                .value_name("PATH")
                .about("Output path where to save music.")
                .default_missing_value(".")
                .default_value("."),
        )
        .get_matches();

    if let (Some(playlist_path), Some(output_path)) = (
        matches.value_of("playlist"),
        matches.value_of("output_path"),
    ) {
        let playlist_file = fs::read_to_string(playlist_path).expect("Couldn't find playlist file");
        let artists = get_artists(playlist_file);

        match download_songs(artists, output_path) {
            Err(e) => println!("{:?}", e),
            _ => println!("✨ Success! All songs downloaded! Happy listening ✨"),
        }
    }
}
