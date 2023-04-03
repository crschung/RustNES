use std::{time::Duration};

use midly::{TrackEvent, MidiMessage, TrackEventKind};
use rodio::{source::{SineWave, Source}, OutputStream, Sink};

mod waves;

fn main() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "RustNES",
        options,
        Box::new(|_cc| Box::new(RustNES::default())),
    )
}

///
/// Takes as an input the current MIDI track, and can be used to retrieve individual notes
/// Note: MIDIs can have multiple tracks
/// 
fn parse_midi(track: &Vec<TrackEvent>)
{
    for track_event in track{
        let kind = track_event.kind;

        match kind {
            TrackEventKind::Midi { channel, message } => match message {
                MidiMessage::NoteOff { key, vel } => println!("Note Off: channel={}, key={}, vel={}", channel, key, vel),
                MidiMessage::NoteOn { key, vel } => println!("Note On: channel={}, key={}, vel={}", channel, key, vel),
                MidiMessage::Aftertouch { key, vel } => println!("Aftertouch: channel={}, key={}, vel={}", channel, key, vel),
                MidiMessage::Controller { controller, value } => println!("Controller: channel={}, controller={}, value={}", channel, controller, value),
                MidiMessage::ProgramChange { program } => println!("Program Change: channel={}, program={}", channel, program),
                MidiMessage::ChannelAftertouch { vel } => println!("Channel Aftertouch: channel={}, vel={}", channel, vel),
                MidiMessage::PitchBend { bend } => println!("Pitch Bend: channel={}, bend={}", channel, bend),
            }
            _ => {}

        }
    }
}

/// Tentative name taken from Github
struct RustNES {
    // Test variable for the GUI. Displays currently selected files name
    picked_path: Option<String>,
}

impl Default for RustNES {
    fn default() -> Self {
        Self {
            picked_path: None,
        }
    }
}

/// Egui base
impl eframe::App for RustNES {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RustNES");


            if ui.button("Open file…").clicked() {

                // rfd is used to access files
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked Midi:");
                    ui.monospace(picked_path);
                });
            }

            if ui.button("Play NES Triangle").clicked(){
                play_nes_triangle_wave(440.0);
            }

            if ui.button("Play NES Pulse").clicked(){
                play_nes_pulse_wave(440.0);
            }

            if ui.button("Play NES Noise").clicked(){
                play_nes_noise();
            }
        });
    }
}


///
/// Simple rodio sink to play an NES triangle wave
/// 
fn play_nes_triangle_wave(freq: f32){
    let source = waves::NESTriangleWave::new(freq).take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

///
/// Simple rodio sink to play an NES pulse wave
/// 
fn play_nes_pulse_wave(freq: f32){
    let source = waves::NESPulseWave::new(freq, 0.8).take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

///
/// Simple rodio sink to play a sine wave
/// 
fn play_nes_noise(){
    let source = waves::NESNoise::new().take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}