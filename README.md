

# RevDL 

Downloads your playlists into handy dandy file structure.

## Dependencies

External dependencies that are required in your $PATH variable:
* [Youtube-DL](https://youtube-dl.org/)
* [FFMpeg](https://www.ffmpeg.org/)

You can install them however you like, just make sure they are both reachable from your $PATH variable.

## Running

```shell
RevDL 0.1

Michał K. <michal0kasprzyk@gmail.com>

Download playlists using Youtube-DL and FFMpeg!

USAGE:
    revdl.exe [OPTIONS] --playlist <FILE>

OPTIONS:
    -h, --help               Print help information
    -o, --output <PATH>      Output path where to save music. [default: .]
    -p, --playlist <FILE>    Playlist to download from, see examples.
    -V, --version            Print version information
```

So for example:

```shell
$ revdl -p "./playlist_file" -o "./my_music"
```

```shell
$ revdl --playlist "playlist_file"
```

## Contributing

All pull requests are welcome though not all will be pulled in.

When it comes down to commit convention, we follow **[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).**

Aha - don't touch playlist folder, it's mine.

## Building

We use Rust, which means wonderful cargo. Take a look at `Cargo.toml` for more details and simply

```shell
$ cargo build
```

## Road map

The big TODOs are:
* [x] **pararellize downloading songs**
* [x] get rid of `crossterm` colored output syntax and substitute it for something simpler (it's an overkill..)
* [ ] setup proper and clean UI that:
    * [x] determine what kind of structure playlist file needs to have to be easily parsable AND human readable - eg. .json / .xml is way too much.</del>
    * [ ] ability to INITIALIZE playlists for users, append to specific playlist and specific category. Searching when there's a duplicate in the playlist too!
    * [x] download ONLY audio files and determine properly if those were not able to be found by youtube-dl. </del>
    * [x] download file ONLY if the file is not present in playlist folder. </del>
    * [x] setup proper folder structure when downloading files - if the user asks for song from multiple playlist files, have files be seperated into seperate folders from seperate playlists.
* [ ] resolve issue with lacking ffmpeg on a system to decode music file.
* [ ] some kind of testing/tests would be nice lmao.
* [ ] preferably find a way to move away from youtube-dl to relay on a VARIOUS different services to find songs rather than just one. 
* [ ] allow users to specify what site they want their song to be downloaded from..
* [ ] allow for direct links to be downloaded IF they are still available, otherwise search again and download found. Something like:
    * [ ] `` My Artist - My Music # link: https://www.youtube.com/watch?v=e-_SCZZlrNs ```. So first you try link, if it doesn't work then try searching title.
    * [ ] Maybe update the link if the one is not working? Or leave it up to user.
