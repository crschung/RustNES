import mido
import pygame
import pygame.midi
from mido import MidiFile
import midiFunctions
import os

def main():
    mid = MidiFile('midi_files\debussy-clair-de-lune.mid')
    pygame.init()
    pygame.midi.init()

    # Load MIDI file
    midi_file = "midi_files\debussy-clair-de-lune.mid"
    midi_data = pygame.midi.midi_to_frequency

    # Open MIDI output device
    midi_out = pygame.midi.Output(0)

    # Play MIDI file
    for event in midi_data.play():
        midi_out.write(event)

    # Close MIDI output device
    midi_out.close()

    # Quit Pygame and Pygame MIDI
    pygame.midi.quit()
    pygame.quit()





        
def convert_midi():
    print("convert")

def playMidi():
    print("convert") 






if __name__ == "__main__":
    main()