import argparse
from cmd2 import Cmd, cmd2

from player import MusicController

class App(Cmd):
    speak_parser = argparse.ArgumentParser()
    speak_parser.add_argument('-p', '--playlist', type=str, help='Full name to playlist')
    speak_parser.add_argument('-s', '--songname', type=str, help='Full name of a song to play')

    def __init__(self):
        super().__init__()
        self.music_controller = MusicController(app=self)

    @cmd2.with_argparser(speak_parser)
    def do_play(self, args) -> None:
        """Plays and downloads song for me.""" #  TODO Better help
        if args.playlist:
            # config = configparser.ConfigParser(delimiters='-')
            # [(key, config['Simple Values'][key]) for key in config['Simple Values']]
            self.music_controller.play_playlist(playlist_file=args.playlist)
        elif args.songname:
            self.music_controller.play_song(song_name=args.songname)

    def do_stop(self, args) -> None:
        self.music_controller.stop_song()

def test():
    app.prompt = 'Hello world>'

if __name__ == '__main__':
    app = App()
    test()
    app.cmdloop()
