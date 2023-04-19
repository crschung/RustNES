use midly::{TrackEvent, MidiMessage, TrackEventKind};
use rodio::{source::{Source}};
use std::i16;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::time::Duration; // temp

mod rustnes; 
mod waves; // for testing purposes

fn main() {

    // Test code for export_wav function
    /* let data = waves::NESTriangleWave::new(220.0).take_duration(Duration::from_secs_f32(1.0));
    export_wav("test.wav", data); */
    //

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "RustNES",
        options,
        Box::new(|_cc| Box::new(rustnes::RustNES::default())),
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

///
/// Function for exporting audio as a 16 bit WAV file
/// NESTriangleWave data type is temporary
///

fn export_wav(filename: &str, data: rodio::source::TakeDuration<waves::NESTriangleWave>) { 
    let header = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer= hound::WavWriter::create(filename, header).unwrap();

    for sample in data {
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }

}

/// Egui base
impl eframe::App for rustnes::RustNES {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            self.navigation_bar(ui);

            self.control_bar(ui);
        });
    }
    
}