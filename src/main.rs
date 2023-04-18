use midly::{TrackEvent, MidiMessage, TrackEventKind};
use rodio::{source::{Source}};

mod rustnes; 

fn main() {

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

/// Egui base
impl eframe::App for rustnes::RustNES {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            self.navigation_bar(ui);

            self.control_bar(ui);
        });
    }
    
}