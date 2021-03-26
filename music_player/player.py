import sys
import time
from pathlib import Path
from typing import List, Optional, Dict
import mimetypes
import youtube_dl
import vlc
import os
from os import name as os_name

DOWNLOAD_FINISHED = 'finished'
AUDIO_FORMAT = 'bestaudio/best'


class MusicController:

    def __init__(self) -> None:
        self.youtube_options: Dict = {
            'format': AUDIO_FORMAT,
            # 'logger': MusicController.YtLogger(),
            'progress_hooks': [self.download_hook],
            'quiet': True
        }
        self.music_queue: List[vlc.MediaPlayer] = []
        self.current_song_name: Optional[str] = None
        self.currently_playing = None

    def download_song(self, song_name: str) -> None:
        self.youtube_options['output'] = song_name
        self.current_song_name: str = song_name
        with youtube_dl.YoutubeDL(self.youtube_options) as ydl:
            ydl.download([f'ytsearch:{song_name}'])

    def download_playlist(self, playlist_path: Path) -> None:
        pass

    def stop_song(self) -> None:
        for song in self.music_queue:
            song.stop()

    def download_hook(self, response: dict) -> None:
        if response.get('status') == DOWNLOAD_FINISHED:
            raw_filename: str = response.get('filename')
            mime_type: str = mimetypes.guess_type(raw_filename)[0]
            if mime_type in ['audio/mp4', 'audio/mp3', 'audio/m4a']:
                self.current_song_name: str = raw_filename[:-16]
            elif mime_type in ['video/webm']:
                self.current_song_name: str = raw_filename[:-17]
            self._start_playing(response.get('filename'))

    def _start_playing(self, file_name: str) -> None:
        self.currently_playing = vlc.MediaPlayer(file_name)
        self.currently_playing.play()
        time.sleep(2)
        self.music_queue.append(self.currently_playing)
        # while currently_playing.is_playing():
        #     continue

        # self.app.prompt = f'Music Player>'

    def get_song_name(self) -> str:
        return self.current_song_name

    # @staticmethod
    # def clear_screen() -> None:
    #     print('\n' * 100)
    #     # if os_name == 'nt':
    #     #     os.system('cls')
    #     # else:
    #     #     os.system('clear')
    #
    # class YtLogger(object):
    #     def debug(self, msg):
    #         print(msg)
    #
    #     def warning(self, msg):
    #         print(msg)
    #
    #     def error(self, msg):
    #         print(msg)
