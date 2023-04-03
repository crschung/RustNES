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
                MidiMessage::NoteOff { key, vel } => todo!(),
                MidiMessage::NoteOn { key, vel } => todo!(),
                MidiMessage::Aftertouch { key, vel } => todo!(),
                MidiMessage::Controller { controller, value } => todo!(),
                MidiMessage::ProgramChange { program } => todo!(),
                MidiMessage::ChannelAftertouch { vel } => todo!(),
                MidiMessage::PitchBend { bend } => todo!(),
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