import sys
from pathlib import Path


from music_player.player import MusicController


class CmdApp:
    BASIC_PROMPT: str = 'Music Player'

    def __init__(self):
        self.music_controller: MusicController = MusicController()
        self.prompt: str = CmdApp.BASIC_PROMPT

    def cmd_loop(self) -> None:

        while True:
            self.clear_screen()
            if self.music_controller.currently_playing and self.music_controller.currently_playing.is_playing():
                self.prompt = self.music_controller.get_song_name()
            sys.stdout.write(self.prompt + '>')
            tokens = input().split()
            if not tokens:
                continue

            command = tokens[0]

            if command in ['quit', 'exit', 'break', 'q', 'e']:
                break

            elif command in ['play', 'playsong', 'pl']:
                song_name = ' '.join(tokens[1:])
                self.music_controller.download_song(song_name)

            elif command in ['playlist']:
                playlist = Path(' '.join(tokens[1:]))
                self.music_controller.download_playlist(playlist)

            elif command in ['stop', 's']:
                self.prompt = CmdApp.BASIC_PROMPT
                self.music_controller.stop_song()

            else:
                sys.stdout.write('Unrecognized Command - type help for list of commands.\n')

    @classmethod
    def clear_screen(cls) -> None:
        sys.stdout.write(chr(27) + "[2J")


if __name__ == '__main__':
    app = CmdApp()
    app.cmd_loop()
