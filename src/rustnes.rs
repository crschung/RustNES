use std::time::Duration;
use rodio::{source::{Source}, OutputStream, Sink};

// Moved waves below rustnes to allow mod to play waves, and clear up main
mod waves;

pub(crate) struct RustNES {
    // Test variable for the GUI. Displays currently selected files name
    pub(crate) _picked_path: Option<String>,
    pub(crate) _test_bool: bool,
    pub(crate) volume: f32,
}

impl Default for RustNES {
    fn default() -> Self {
        Self {
            _picked_path: None,
            _test_bool: false,
            volume: 100.0,
        }
    }
}

impl RustNES{

    
    pub(crate) fn navigation_bar(&mut self, ui: &mut egui::Ui){

        egui::TopBottomPanel::top("navigation_menu")
        .resizable(false)
        .min_height(25.0)
        .show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.menu_button("File", Self::file_menu);
                ui.menu_button("Edit", Self::edit_menu);
    
                #[cfg(debug_assertions)]
                ui.menu_button("Debug", Self::debug_menu);
            });
        });
    }

    /// The File context menu
    /// Contains New, Open File, Save, Export (MIDI/NSF), Import (MIDI/NSF)
    pub(crate) fn file_menu(ui: &mut egui::Ui) {
        if ui.button("New").clicked() {
            println!("TODO! clear current work");
            ui.close_menu();
        }

        if ui.button("Open File...").clicked() {
            // rfd is used to access files
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                Some(path.display().to_string());
            }
            ui.close_menu();
        }

        if ui.button("Save").clicked() {
            println!("TODO! save current work");
            ui.close_menu();
        }

        ui.menu_button("Export", |ui| {
            if ui.button("MIDI").clicked() {
                println!("TODO! export work as MIDI");
                ui.close_menu();
            }
            if ui.button("NSF").clicked() {
                println!("TODO! export work as NSF");
                ui.close_menu();
            }
        });

        ui.menu_button("Import", |ui| {
            if ui.button("MIDI").clicked() {
                println!("TODO! import MIDI");
                ui.close_menu();
            }
            if ui.button("NSF").clicked() {
                println!("TODO! import NSF");
                ui.close_menu();
            }
        });

    }

    // The edit context menu
    pub(crate) fn edit_menu(ui: &mut egui::Ui) {
        if ui.button("STUB").clicked() {
            println!("TODO!");
            ui.close_menu();
        }
    }

    // The debug contect menu.
    // This menu should only be visible in debug mode
    pub(crate) fn debug_menu(ui: &mut egui::Ui) {
        if ui.button("Play NES Triangle").clicked(){
            play_nes_triangle_wave(440.0);
        }

        if ui.button("Play NES Pulse").clicked(){
            play_nes_pulse_wave(440.0);
        }

        if ui.button("Play NES Noise").clicked(){
            play_nes_noise();
        }
    }

    ///
    /// The botton control bar with play, pause, and volume
    /// 
    pub(crate) fn control_bar(&mut self, ui: &mut egui::Ui){
        egui::TopBottomPanel::bottom("control_menu")
        .resizable(false)
        .min_height(25.0)
        .show_inside(ui, |ui| {
        ui.horizontal_centered(|ui| {
            if ui.button("Play").clicked(){
                println!("TODO!");
            }
            if ui.button("Pause").clicked(){
                println!("TODO!"); 
            }
            ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0).show_value(false));
        });
        });
    }
}



///
/// Temporarily moved here
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
/// Temporarily moved here
/// Simple rodio sink to play an NES pulse wave
/// 
fn play_nes_pulse_wave(freq: f32){
    let source = waves::NESPulseWave::new(freq, 0.5).take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

///
/// Temporarily moved here
/// Simple rodio sink to play a sine wave
/// 
fn play_nes_noise(){
    let source = waves::NESNoise::new().take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}