# RustNES (IN_PROGRESS)

## Description
---
Retro chiptune music is starting to make a resurgence with the popularity of many retro indie games like Hades and Celeste. One of the reasons why it's gaining its popularity again is because it resonates with a lot of people; it brings back nostalgia and fond memory associated with it. However, a majority of music produced for these games rarely use actual hardware that made the 8-bit and 16-bit music charming. The purpose of this project is to replicate a sound chip from the NES (Nintendo Entertainment System) using Rust, and bring back those memories to the users.


## Resources
---
>* [Retro Game Mechanics Explained Playlist](https://www.youtube.com/playlist?list=PLHQ0utQyFw5JD2wWda50J8XuzQ2cFr8RX)
>* https://www.youtube.com/watch?v=8RrQrATnXXY
>* https://www.egui.rs/
>* [NES Basics](https://bugzmanov.github.io/nes_ebook/)
>* [Crate Midly](https://docs.rs/midly/latest/midly/)
>* https://docs.rs/midly/latest/rodio
>* https://github.com/RustAudio/rodio
>* https://docs.rs/basic_waves/latest/basic_waves/index.html
>* https://www.youtube.com/watch?v=gKXGDuKrCfA

## Relevant Research Papers
---
>* [Automatic conversion of pop music into chiptunes for 8-bit pixel art](https://ieeexplore.ieee.org/abstract/document/7952188)
>* [Music Genre Classification Using MIDI and Audio Features](https://link.springer.com/content/pdf/10.1155/2007/36409.pdf)
>* [Melody extraction on MIDI music files](https://ieeexplore.ieee.org/abstract/document/1565863/)

## Programming Language
---
Mainly Rust, and possibly Python as well

## Possible Function Implemenations
---
* MIDI Input
* Visualization (waveform)
* GUI
* Pass songsusing import 

## Good to have:
---
* Filters?
* Entrie chip (real code)
* Export and Save (.wav, .ogg, .mp3)

## Crates
---
* [Rodio](https://github.com/RustAudio/rodio)
* simple_waves
* [mido](https://mido.readthedocs.io/en/latest/)
* [egui](https://github.com/emilk/egui)


## How To Run
---
```
cargo build
cargo run
```

