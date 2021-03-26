

# Terminal Music Player

Downloads and plays your music in one go in a handy terminal.


## Installing

External dependencies:
* [VLC Media player](https://www.videolan.org/vlc/)
* [Pipenv](https://pypi.org/project/pipenv/)

Internal dependencies - once you have pipenv just simply do:
```shell
pipenv init
```
Into:
```bash 
pipenv install && pipenv install --dev
```


## Contributing

All pull requests are welcome though not all will be pulled in. There's no clear style setup other than to follow
current best practises of Python (including upcoming 3.10) and PEP 8 style. Type hints are mandatory though.

When it comes down to commit convention, we follow **strictly [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).**

## Road map

The absolute dream scenario would be to make it look like [spotify-tui](https://github.com/Rigellute/spotify-tui) which
is absolutely phenomenal, however one should tamper their expectations before taking on such a big task.
The big TODOs are:
* find replacement for VLC when it comes down to playing music - we don't want users having to install 3rd party app
on top of already installing ours.
* setup proper and clean UI that:
    * has controlls to play music by title or from a playlist file or from a folder of playlists files.
    * determine what kind of structure playlist file needs to have to be easily parsable AND human readable - eg. .json / .xml is way too much.
    The file and parser has to have ability to play songs from different artists (or maybe different discs of artists too?) from the same file, while ignoring the others.
    * skip, pause, play, forward, next, previous, volume up/down, randomized etc. included.
    * shows what song is being played and for how long. maybe those fancy music graphs too - would be cool.
    * **is completely non blocking** meaning it has to work independently of music being played in the background.
    * some help strings to properly show user how to control stuff. (it's not that important before at least having workable UI).
* download ONLY audio files and determine properly if those were not able to be found by youtube-dl.
* download file ONLY if the file is not present in playlist folder. 
* setup proper folder structure when downloading files - if the user asks for song from multiple playlist files, have files be
seperated into seperate folders from seperate playlists.
* make it so the NEXT song in queue is downloaded during playing of the song, so that the user doesn't have too long of a delay
between songs.