use std::{fmt::Debug, fs, vec};

use subprocess::PopenError;

// const FILE_NAME: &str = "../playlists/chill/chill.ini";
// const FILE_NAME: &str = "tests/examples/playlist_test";
const FILE_NAME: &str = "tests/examples/playlist_albums";

#[derive(Debug)]
struct Song {
    name: String,
}

#[derive(Debug)]
struct Album {
    songs: Vec<Song>,
    name: String,
}

#[derive(Debug)]
struct Artist {
    albums: Vec<Album>,
    singles: Vec<Song>,
    name: String,
}

impl Default for Artist {
    fn default() -> Artist {
        Artist {
            albums: vec![],
            singles: vec![],
            name: String::from("unnamed"),
        }
    }
}

fn get_artists(playlist_file: String) -> Vec<Artist> {
    let mut UNNAMED_ARTIST = Artist {
        ..Default::default()
    };

    let mut artists: Vec<Artist> = vec![UNNAMED_ARTIST];

    let mut found_artist_x: Option<Artist> = None;
    let mut found_album_x: Option<Album> = None;

    let mut found_artist: Option<&str> = None;
    let mut found_album: Option<&str> = None;

    for (index, line) in playlist_file
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .enumerate()
    {
        let file_token = line.chars().next().unwrap();

        match file_token {
            '@' => {
                found_artist = Some(line[1..].trim());
                found_album_x = None;
                found_artist_x = Some(Artist {
                    name: String::from(line[1..].trim()),
                    ..Default::default()
                });
                println!("Artist => {}", found_artist.unwrap());
                println!("Artist STRUCT => {:?}", found_artist_x.unwrap());
            }
            '#' => {
                found_album = Some(line[1..].trim());
                if let Some(artist) = found_artist {
                    println!("Album => {} => artist {}", found_album.unwrap(), artist)
                } else {
                    eprintln!("Album => {}", found_album.unwrap())
                }
            }
            _ => {
                if let (Some(album), Some(artist)) = (found_album, found_artist) {
                    println!("Song => {} => album {} => artist {}", line, album, artist)
                } else if let Some(artist) = found_artist {
                    eprintln!("Song => {} => artist {} !", line, artist)
                } else {
                    eprintln!("Song => {}", line);
                }
            }
        }
    }
    artists
}

fn main() {
    // TODO: Add better error handling
    let playlist_file =
        fs::read_to_string(FILE_NAME).expect("Couldn't find a file specified by path");

    let x = get_artists(playlist_file);

    // let input = Command::new("youtube-dl")
    // .arg("--extract-audio")
    // .arg("--audio-format")
    // .arg("mp3")
    // .arg(format!("ytsearch1:{}", line))
    // .output()
    // .expect("failed to execute process");
}
