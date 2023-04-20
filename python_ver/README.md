## Documentation for David Kim:

## Rust Stuff
---
Here are some resources I consulted while trying to make some visual output working in Rust:

>* [https://www.egui.rs/](https://www.egui.rs/) (Spencer recommended I shold check this link to see how the visual output should be like, specifically the "Dancing Strings" demo)
    
>* [https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/dancing_strings.rs](https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/dancing_strings.rs) (Source code for the "Dancing Strings" demo)
    
>* [https://docs.rs/egui/latest/egui/containers/panel/index.html](https://docs.rs/egui/latest/egui/containers/panel/index.html) (The UI for the Rust program uses this module for its formatting)

## Python Stuff
---
Here are some resources I consulted to make the visual output in Python:

>* [https://stackoverflow.com/questions/60416179/real-time-audio-recording-and-plotting-in-python-without-causing-discontinuities](https://stackoverflow.com/questions/60416179/real-time-audio-recording-and-plotting-in-python-without-causing-discontinuities) (Initially used this resource as a reference of how to plot the noise output in real time)
    
>* [https://stackoverflow.com/questions/26573556/record-speakers-output-with-pyaudio](https://stackoverflow.com/questions/26573556/record-speakers-output-with-pyaudio) (Initially used this resource as a reference of getting audio from computer apps)
    
>* [https://pypi.org/project/PyAudioWPatch/](https://pypi.org/project/PyAudioWPatch/) (Tried to use this library to get the audio playing in computer apps, including the RustNES project)
    
>* [https://github.com/s0d3s/PyAudioWPatch/blob/master/examples/pawp_record_wasapi_loopback.py](https://github.com/s0d3s/PyAudioWPatch/blob/master/examples/pawp_record_wasapi_loopback.py) (Initially used this example code from the PyAudioWPatch library as a reference to get the computer audio so it can be processed by Python) 

>* [https://stackoverflow.com/questions/39658717/plot-dynamically-changing-graph-using-matplotlib-in-jupyter-notebook](https://stackoverflow.com/questions/39658717/plot-dynamically-changing-graph-using-matplotlib-in-jupyter-notebook) (Referenced this StackOverflow post to code in the visual output in the Chiptune_python.ipynb Jupyter notebook)