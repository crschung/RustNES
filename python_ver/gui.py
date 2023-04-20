import tkinter as tk
import numpy as np
import pygame.mixer as mixer
import pygame
import midiFunctions

# Basic GUI that Jae attempted

class MusicPlayer:
    def __init__(self, master):
        self.master = master
        master.title("Music Player")
        master.geometry("300x150")

        self.play_button = tk.Button(master, text="Play", command=self.play_music)
        self.play_button.pack(pady=10)

        self.pause_button = tk.Button(master, text="Pause", command=self.pause_music)
        self.pause_button.pack(pady=10)

        self.open_button = tk.Button(master, text="Open File", command=self.open_file)
        self.open_button.pack(pady=10)

    def open_file(self):

        file_path = tk.filedialog.askopenfilename()
        if file_path:
            mixer.init()

            # convert the midi file to np array
            music = midiFunctions.convertMidi(file_path)
            sound = pygame.sndarray.make_sound((music * (2**(bits-1) - 1)).astype(np.int16))
        sound.play()

        pygame.quit()

    def play_music(self):
        mixer.music.play()

    def pause_music(self):
        mixer.music.pause()


if __name__ == '__main__':
    root = tk.Tk()
    player = MusicPlayer(root)
    root.mainloop()