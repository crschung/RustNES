
"""A simple example of recording from speakers ('What you hear') using the WASAPI loopback device"""

#from _spinner_helper import Spinner
# Spinner is a helper class that is in the same examples folder.
# It is optional, you can safely delete the code associated with it.

import pyaudiowpatch as pyaudio
import time
import wave
import matplotlib.pyplot as plt
import numpy as np
import audioop

duration = 5.0

filename = "loopback_record.wav"
    

"""
Create PyAudio instance via context manager.
Spinner is a helper class, for `pretty` output
"""
# Get default WASAPI info
wasapi_info = pyaudio.PyAudio().get_host_api_info_by_type(pyaudio.paWASAPI)

# Get default WASAPI speakers
default_speakers = pyaudio.PyAudio().get_device_info_by_index(wasapi_info["defaultOutputDevice"])

if not default_speakers["isLoopbackDevice"]:
    for loopback in pyaudio.PyAudio().get_loopback_device_info_generator():
        """
        Try to find loopback device with same name(and [Loopback suffix]).
        Unfortunately, this is the most adequate way at the moment.
        """
        if default_speakers["name"] in loopback["name"]:
            default_speakers = loopback
            break
    else:
        exit()
        

wave_file = wave.open(filename, 'wb')
wave_file.setnchannels(default_speakers["maxInputChannels"])
wave_file.setsampwidth(pyaudio.get_sample_size(pyaudio.paInt16))
wave_file.setframerate(int(default_speakers["defaultSampleRate"]))

def callback(in_data, frame_count, time_info, status):
    """Write frames and return PA flag"""
    wave_file.writeframes(in_data)
    return (in_data, pyaudio.paContinue)

stream_test = pyaudio.PyAudio().open(format=pyaudio.paInt16,
        channels=default_speakers["maxInputChannels"],
        rate=int(default_speakers["defaultSampleRate"]),
        frames_per_buffer=pyaudio.get_sample_size(pyaudio.paInt16),
        input=True,
        input_device_index=default_speakers["index"],
        stream_callback=callback)
"""
Opena PA stream via context manager.
After leaving the context, everything will
be correctly closed(Stream, PyAudio manager)            
"""

CHUNK = 512
RATE = 48000
RECORD_SECONDS = duration

# create matplotlib figure and axes
fig, ax = plt.subplots(1, figsize=(15, 7))

# variable for plotting
x = np.arange(0, 2 * CHUNK, 2)

line = ax.plot(x, np.random.rand(CHUNK), '-', lw=2)[0]
# basic formatting for the axes
ax.set_title('AUDIO WAVEFORM')
ax.set_xlabel('samples')
ax.set_ylabel('volume')
ax.set_xlim(0, 2 * CHUNK)
plt.setp(ax, xticks=[0, CHUNK, 2 * CHUNK], yticks=[-1000, 1000])
# show the plot
plt.show(block=False)


frames = []
for i in range(0, int(RATE / CHUNK * RECORD_SECONDS)):
    data = stream_test.read(512)
    frames.append(data)

    result = np.frombuffer(data, dtype=np.int16)
    line.set_ydata(result)
    fig.canvas.draw()
    fig.canvas.flush_events()


time.sleep(duration) # Blocking execution while playing

wave_file.close()




'''
import numpy as np
import matplotlib.pyplot as plt
import pylab
import wave
import pyaudio 
import struct
import msvcrt


CHUNK = 2048
FORMAT = pyaudio.paInt16
CHANNELS = 1
RATE = 48000
RECORD_SECONDS = 5 # recording will be terminated after 5 sec

p = pyaudio.PyAudio()

stream = p.open(format=FORMAT,
                channels=CHANNELS,
                rate=RATE,
                input=True,
                frames_per_buffer=CHUNK)


# create matplotlib figure and axes
fig, ax = plt.subplots(1, figsize=(15, 7))

# variable for plotting
x = np.arange(0, 2 * CHUNK, 2)

# create a line object with random data
line = ax.plot(x, np.random.rand(CHUNK), '-', lw=2)[0]
# basic formatting for the axes
ax.set_title('AUDIO WAVEFORM')
ax.set_xlabel('samples')
ax.set_ylabel('volume')
ax.set_xlim(0, 2 * CHUNK)
plt.setp(ax, xticks=[0, CHUNK, 2 * CHUNK], yticks=[-1000, 1000])
# show the plot
plt.show(block=False)


#print("* recording")
frames = []
for i in range(0, int(RATE / CHUNK * RECORD_SECONDS)):    
    data = stream.read(CHUNK)
    print(data)
    print("chungus")
    frames.append(data)

    result = np.frombuffer(data, dtype=np.int16)
    line.set_ydata(result)
    fig.canvas.draw()
    fig.canvas.flush_events()
'''


'''
import numpy as np
import matplotlib.pyplot as plt

plt.axis([0, 10, 0, 1])

for i in range(10):
    y = np.random.random()
    plt.scatter(i, y)
    plt.pause(0.05)

plt.show()
'''