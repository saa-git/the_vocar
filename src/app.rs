use crate::demo;

use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub enum Screen {
    #[default]
    StartMenu,
    GetRace,
    RaceScreen,
    GetStartingClass,
    StartingClassScreen,
    Activity
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct VocarApp {
    demo: demo::Demo,
    screen: Screen
}

impl VocarApp { 
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self { 
        /*if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(); 
        }*/ 
        Default::default() 
    }
}

impl eframe::App for VocarApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { demo, screen } = self;

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        // println!("{:?}", frame.info().window_info.size);

        match screen {
            Screen::StartMenu => {
                start_menu(ctx, screen);
            },
            Screen::GetRace => {
                get_race(ctx, demo, screen);
            },
            Screen::RaceScreen => {
                display_race(ctx, demo, screen);
            }
            Screen::GetStartingClass => {
                get_starting_class(ctx, demo, screen);
            },
            Screen::StartingClassScreen => {
                display_starting_class(ctx, demo, screen)
            }
            Screen::Activity => {
                fact_screen(ctx, demo, frame)
            }
        }   
    }
}

fn start_menu(ctx: &egui::Context, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);
            ui.heading("The Vocar");
            ui.label("Welcome to the Vocar! This is an activity that guests participated in at the Bob Moses Conference 2023.");
            ui.add_space(20.0);
            if ui.button("Begin Your Journey!").on_hover_text("Clic").clicked() {
                *screen = Screen::GetRace;
            }
        });
        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
}

fn get_race(ctx: &egui::Context, demo: &mut demo::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);
            ui.heading("Getting A Race");
            ui.label("For this experience, you will be given a random race.");
            ui.add_space(20.0);
            if ui.button("Get My Race!").on_hover_text("Click to go to the next screen.").clicked() {
                demo.race = random();
                *screen = Screen::RaceScreen;
            }
        });
    });
}

fn display_race(ctx: &egui::Context, demo: &mut demo::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);
            ui.heading(&format!("You are {} :).", demo.race));
            ui.label("This will be your race PERMANENTLY.");
            ui.add_space(20.0);
            if ui.button("Next!").on_hover_text("Click to go to the next screen.").clicked() {
                *screen = Screen::GetStartingClass;
            }
        });
    });
}

fn get_starting_class(ctx: &egui::Context, demo: &mut demo::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);

            ui.heading("Getting Your Starting Quintile");

            ui.label("For this experience, you will also be given a random starting quintile.");

            ui.add_space(20.0);

            if ui.button("Get My Starting Quintile!").on_hover_text("Click to go to the next screen.").clicked() {
                demo.class_zero = random();
                demo.class_n = demo.class_zero;
                *screen = Screen::StartingClassScreen;
            }
        });
    });
}

fn display_starting_class(ctx: &egui::Context, demo: &mut demo::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(170.0);

            ui.heading(&format!("You are in the {}.", demo.class_zero));

            ui.add_space(10.0);

            ui.label("This starting Quintile is just a jumpoff point and may change for each subsequent generation.");

            ui.label("The chance to get a higher, equal or lower Quintile is not same.");

            ui.add_space(20.0);

            if ui.button("Next").on_hover_text("Click to go to the next screen.").clicked() {
                demo.class_zero = random();

                demo.class_n = demo.class_zero;

                *screen = Screen::Activity;
            }
        });
    });
}

fn fact_screen(ctx: &egui::Context, demo: &mut demo::Demo, frame: &mut eframe::Frame) {
    let width = (frame.info().window_info.size.x)/5.0;

    egui::SidePanel::left("card_panel").resizable(true).default_width(width).show(ctx, |ui| {
        ui.separator();
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {

            ui.heading("Your Vocar Card");
        });

        ui.separator();
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Race: ");

            ui.text_edit_singleline(&mut demo.race.to_string());
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Current Class: ");

            ui.text_edit_singleline(&mut demo.class_n.to_string());
        });
        ui.add_space(5.0);

        ui.separator();
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Starting Class: ");

            ui.text_edit_singleline( &mut demo.class_zero.to_string());
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Final Class: ");

            ui.text_edit_singleline(&mut demo.class_five.to_string());
        });
        ui.add_space(5.0);

        ui.separator();
        ui.add_space(5.0);
        for (i, class) in demo.history.iter().enumerate() {
            ui.horizontal(|ui| {
                let text: &str = &format!("Generation {}: ", i + 1);

                ui.label(text);

                ui.text_edit_singleline(&mut class.to_string());
            });

            ui.add_space(10.0)
        }
        if ui.button("Add Gen").clicked() {
            if demo.history.len() < 5 {
                demo.history.push(random());

                demo.class_n = demo.history[demo.history.len() - 1];

                if demo.history.len() == 5 {

                    demo.class_five = demo.history[4];
                }
            }
            
        }
        ui.add_space(5.0);
    });

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("eframe template");
        ui.hyperlink("https://github.com/emilk/eframe_template");
        ui.add(egui::github_link_file!(
            "https://github.com/emilk/eframe_template/blob/master/",
            "Source code."
        ));
        egui::warn_if_debug_build(ui);
    });
}

/*
fn (ctx: &egui::Context, demo: &mut demo::Demo, screen: &mut Screen) {

}
*/