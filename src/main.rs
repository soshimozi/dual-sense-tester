// src/main.rs
mod hid;
mod effects;
mod report;

use clap::{Parser, ValueEnum};
use eframe::egui;
use effects::{ControllerEffect, trigger::TriggerMode, led::LedEffect};
use hid::{find_dualsense_path, write_report};
use report::DualSenseReport;
use serde::Deserialize;
use std::{fs, thread, time::Duration};

#[derive(Parser, Debug)]
#[command(name = "DualSense CLI", version, about = "Send adaptive trigger effects to a DualSense controller")]
struct Args {
    #[arg(long)]
    config: Option<String>,

    #[arg(long)]
    ui: bool,

    #[arg(value_enum)]
    mode: Option<TriggerEffectMode>,

    #[arg(long)]
    start: Option<u8>,

    #[arg(long)]
    force: Option<u8>,

    #[arg(long)]
    end: Option<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TriggerEffectMode {
    #[default]
    Off,
    Rigid,
    Pulse,
    Slope,
}

#[derive(Debug, Deserialize)]
struct Config {
    mode: TriggerEffectMode,
    start: Option<u8>,
    force: Option<u8>,
    end: Option<u8>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.ui {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "DualSense UI",
            native_options,
            Box::new(|_cc| Box::new(AppState::default())),
        ).expect("Failed to start UI");
        return Ok(());
    }

    let config = if let Some(path) = args.config {
        let contents = fs::read_to_string(path)?;
        Some(toml::from_str::<Config>(&contents).expect("Invalid config format"))
    } else {
        None
    };

    let mode = args.mode.or_else(|| config.as_ref().map(|c| c.mode)).unwrap_or(TriggerEffectMode::Rigid);
    let start = args.start.or_else(|| config.as_ref().and_then(|c| c.start)).unwrap_or(0x40);
    let force = args.force.or_else(|| config.as_ref().and_then(|c| c.force)).unwrap_or(0xFF);
    let end = args.end.or_else(|| config.as_ref().and_then(|c| c.end)).unwrap_or(0xFF);

    let Some(path) = find_dualsense_path() else {
        eprintln!("DualSense not found");
        return Ok(());
    };

    println!("Using device: {}", path);

    let mut report = DualSenseReport::default();
    report.report_id = 0x02;
    report.flags = 0xFF;
    report.enable_bits = 0x07;
    report.rumble_left = 0x40;
    report.rumble_right = 0x40;

    // report.l2_effect = match mode {
    //     TriggerEffectMode::Off => build_l2_effect(TriggerMode::Off),
    //     TriggerEffectMode::Rigid => build_l2_effect(TriggerMode::Rigid { start, force }),
    //     TriggerEffectMode::Pulse => build_l2_effect(TriggerMode::Pulse { start, force }),
    //     TriggerEffectMode::Slope => build_l2_effect(TriggerMode::Slope { start, end }),
    // };

    // report.r2_effect = report.l2_effect;

    let effect = ControllerEffect::new_shared_trigger(
        match mode {
            TriggerEffectMode::Off => TriggerMode::Off,
            TriggerEffectMode::Rigid => TriggerMode::Rigid { start, force },
            TriggerEffectMode::Pulse => TriggerMode::Pulse { start, force },
            TriggerEffectMode::Slope => TriggerMode::Slope { start, end },
        },
        LedEffect::new(0x00, 0x20, 0xFF),
    );

    effect.apply_to_report(&mut report);

    report.lightbar_red = 0x00;
    report.lightbar_green = 0x20;
    report.lightbar_blue = 0xFF;

    println!("Sending {:?} effect...", mode);
    for _ in 0..40 {
        write_report(&path, &report.as_bytes())?;
        thread::sleep(Duration::from_millis(250));
    }

    report.clear_triggers();
    report.rumble_left = 0;
    report.rumble_right = 0;
    write_report(&path, &report.as_bytes())?;

    Ok(())    
}

#[derive(Default)]
struct AppState {
    mode: TriggerEffectMode,
    start: u8,
    force: u8,
    end: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DualSense Trigger Editor");

            ui.horizontal(|ui| {
                ui.label("Mode:");
                egui::ComboBox::from_id_source("mode_combo")
                    .selected_text(format!("{:?}", self.mode))
                    .show_ui(ui, |ui| {
                        for variant in [TriggerEffectMode::Off, TriggerEffectMode::Rigid, TriggerEffectMode::Pulse, TriggerEffectMode::Slope] {
                            ui.selectable_value(&mut self.mode, variant, format!("{:?}", variant));
                        }
                    });
            });

            ui.add(egui::Slider::new(&mut self.start, 0..=255).text("Start"));
            ui.add(egui::Slider::new(&mut self.force, 0..=255).text("Force"));
            ui.add(egui::Slider::new(&mut self.end, 0..=255).text("End"));

            ui.separator();
            ui.label("Lightbar Color:");
            let mut rgb = [
                self.red as f32 / 255.0,
                self.green as f32 / 255.0,
                self.blue as f32 / 255.0,
            ];
            ui.horizontal(|ui| {
                if ui.color_edit_button_rgb(&mut rgb).changed() {
                    self.red = (rgb[0] * 255.0) as u8;
                    self.green = (rgb[1] * 255.0) as u8;
                    self.blue = (rgb[2] * 255.0) as u8;
                }
            });

            if ui.button("Send").clicked() {
                if let Some(path) = find_dualsense_path() {
                    let mut report = DualSenseReport::default();
                    report.report_id = 0x02;
                    report.flags = 0xFF;
                    report.enable_bits = 0x07;
                    report.rumble_left = 0x40;
                    report.rumble_right = 0x40;

                    let trigger_mode = match self.mode {
                        TriggerEffectMode::Off => TriggerMode::Off,
                        TriggerEffectMode::Rigid => TriggerMode::Rigid { start: self.start, force: self.force },
                        TriggerEffectMode::Pulse => TriggerMode::Pulse { start: self.start, force: self.force },
                        TriggerEffectMode::Slope => TriggerMode::Slope { start: self.start, end: self.end },
                    };

                    let led = LedEffect::new(self.red, self.green, self.blue);
                    let effect = ControllerEffect::new_shared_trigger(trigger_mode, led);

                    effect.apply_to_report(&mut report);

                    for _ in 0..40 {
                        let _ = write_report(&path, &report.as_bytes());
                        thread::sleep(Duration::from_millis(250));
                    }

                    report.clear_triggers();
                    report.rumble_left = 0;
                    report.rumble_right = 0;
                    let _ = write_report(&path, &report.as_bytes());
                } else {
                    eprintln!("DualSense not found");
                }
            }
        });
    }
}
