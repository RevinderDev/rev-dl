from typing import List

import youtube_dl
import vlc
from cmd2 import Cmd

DOWNLOAD_FINISHED = 'finished'
AUDIO_FORMAT = 'bestaudio/best'


class MusicController:
    music_queue: List[vlc.MediaPlayer] = []

    def __init__(self, app: Cmd) -> None:
        self.youtube_options: dict = {
            'format': AUDIO_FORMAT,
            'logger': MusicController.YtLogger(),
            'progress_hooks': [MusicController.download_hook],
        }
        self.app: Cmd = app

    def play_song(self, song_name: str) -> None:
        with youtube_dl.YoutubeDL(self.youtube_options) as ydl:
            ydl.download([f'ytsearch:{song_name}'])

    def play_playlist(self, playlist_file: str) -> None:
        pass

    @classmethod
    def stop_song(cls) -> None:
        for song in MusicController.music_queue:
            song.stop()

    @staticmethod
    def download_hook(response: dict) -> None:
        if response['status'] == DOWNLOAD_FINISHED:
            MusicController._start_playing(response['filename'])

    @staticmethod
    def _start_playing(file_name: str) -> None:
        currently_playing = vlc.MediaPlayer(file_name)
        currently_playing.play()
        MusicController.music_queue.append(currently_playing)
        while currently_playing.is_playing() == 1:
            continue

    class YtLogger(object):
        def debug(self, msg):
            print(msg)

        def warning(self, msg):
            print(msg)

        def error(self, msg):
            print(msg)
