use crate::egui::FontFamily::Proportional;
use crate::egui::FontId;
use crate::egui::RichText;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use csv::ReaderBuilder;
use eframe::egui::{self};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

const TEXT_SIZE: f32 = 20.0;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Word Search",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    word: String,
    results: Vec<String>,
    search_time: Option<std::time::Duration>,
    scalar: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            word: "".to_owned(),
            results: Vec::new(),
            search_time: None,
            scalar: 6,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, RichText::new("File").size(TEXT_SIZE), |ui| {
                    if ui.button(RichText::new("Quit").size(TEXT_SIZE)).clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Word Search").size(TEXT_SIZE * 3.0));
            ui.horizontal(|ui: &mut egui::Ui| {
                let mut updated: bool = false;
                while self.scalar > self.word.len() {
                    self.word.push('?');
                    updated = true;

                }
                while self.scalar < self.word.len() {
                    self.word.pop();
                    updated = true;
                }
                let search_text_edit = ui.add(
                    egui::TextEdit::singleline(&mut self.word)
                        .char_limit(20)
                        .min_size(eframe::epaint::Vec2 { x: 500.0, y: 50.0 })
                        .font(FontId::new(TEXT_SIZE * 2.0, Proportional)),
                );

                if search_text_edit.lost_focus() {
                    search_text_edit.request_focus()
                }

                let changed = search_text_edit.changed();

                if changed || updated{
                    self.search_time = Some(measure_search_time(&self.word));
                    self.results = find_words(&self.word).unwrap();
                }

                if ui.button(RichText::new("Clear").size(TEXT_SIZE * 2.0)).clicked() {
                    self.word.clear();
                }

                ui.add(egui::Slider::new(&mut self.scalar, 3..=19));
            });

            ui.separator();

            if let Some(duration) = self.search_time {
                ui.label(RichText::new(format!("Search time: {:?}", duration)).size(TEXT_SIZE));
            }

            if !self.results.is_empty() {
                ui.label(RichText::new(format!("Results {}", &self.results.len())).size(TEXT_SIZE * 2.0));
                ui.label(RichText::new("Click to copy").size(TEXT_SIZE));

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for result in &self.results {
                            if ui
                                .button(RichText::new(result).font(FontId::new(
                                    TEXT_SIZE,
                                    eframe::epaint::FontFamily::Monospace,
                                )))
                                .clicked()
                            {
                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                ctx.set_contents(result.to_owned()).unwrap();
                            }
                            ui.separator();
                        }
                    });
                });
            }
        });
    }
}

fn measure_search_time(search_text: &str) -> std::time::Duration {
    let start_time = Instant::now();
    find_words(search_text).unwrap();
    let end_time = Instant::now();
    end_time - start_time
}

fn find_words(search_text: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file_path = Path::new("Skribbl-words.csv");
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);

    let mut results: Vec<String> = Vec::new();

    for record in reader.records() {
        let record = record?;

        if let (Some(word), Some(length)) = (record.get(0), record.get(1)) {
            let search_text_lower = search_text.to_lowercase();
            if length.parse::<usize>().unwrap() == search_text_lower.len()
                && matches_search_text(word, &search_text_lower)
            {
                results.push(word.to_owned());
            }
        }
    }

    Ok(results)
}

fn matches_search_text(word: &str, search_text: &str) -> bool {
    for (word_char, search_char) in word.chars().zip(search_text.chars()) {
        if search_char != '?' && search_char != word_char {
            return false;
        }
    }

    true
}
