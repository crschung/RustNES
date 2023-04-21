import numpy as np
from mido import MidiFile
from scipy.io.wavfile import write

def convertMidi(fileName):
    mid = MidiFile('debussy-clair-de-lune.mid')

    notes = []

    for i, track in enumerate(mid.tracks):
        for msg in track:
            if msg.type == 'note_on' and msg.velocity > 0 and msg.time > 10:
                note = round(note_to_freq(msg.note))
                time = msg.time * 0.005
                velocity = msg.velocity
                notes.append(('on', note, time, velocity))
            elif msg.type == 'note_off' and msg.time > 10:
                note = round(note_to_freq(msg.note))
                time = msg.time * 0.005
                velocity = msg.velocity
                notes.append(('off', note, time, velocity))

    data = np.zeros((0,))
    for i in range(len(notes) - 1):
        if i == len(notes) - 1:
            break
            
        currentNote = notes[i]
        nextNote = notes[i + 1]
        if (currentNote[0] == 'on' and currentNote[3] > 0):
            sound = generate_triangle(currentNote[1], dur=nextNote[2])
            data = np.hstack([data,sound])
    
        elif (currentNote[0] == 'off'):
            sound = generate_triangle(0, dur=nextNote[2])
            data = np.hstack([data,sound])
    
    data = data / np.max(np.abs(data))

    return data


    # TODO: convert these noteon events to triangle waves


def note_to_freq(note):
    return 440 * 2 ** ((note - 69) / 12)

def generate_triangle(freq, dur=1, amp=1.0, sr=44100):
    t = np.arange(0, dur, 1.0/sr)
    return 2 * (2*np.floor(freq*t) - np.floor(2*freq*t)) + 1

# this sounds good too, but not so "chiptune"
def triangle_wave(freq:float, dur:float=1.0, sr:float=44100):

	t = np.arange(0, dur, 1.0/sr)
	x = np.zeros(t.shape[0])
	for i in range(4):
		n = 2*i + 1
		x += ((-1)**i)*(n**(-2))*(np.sin(2*np.pi*freq*n*t))
	return (8/(np.pi**2)) * x

def export_to_wav(track: list, srate: int, name):

	assert len(track) > 0
	write(name + '.wav', srate, track.astype(np.int16))

if __name__ == "__main__":
    main()