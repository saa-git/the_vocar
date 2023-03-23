use crate::demographic;

//use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Screen {
    #[default]
    Start,
    RaceDetermination,
    RaceDisplay,
    ClassDetermination,
    ClassDisplay,
    FactDisplay,
    MissedFactDisplay,
    End
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Vocar {
    dem: demographic::Demo,
    scr: Screen
}

impl Vocar { 
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(); 
        }
        */
        Vocar::default() 
    }
}

impl eframe::App for Vocar {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { dem, scr } = self;

        match scr {
            Screen::Start => start_screen(ctx, scr),
            Screen::RaceDetermination => race_determination_screen(ctx, dem, scr),
            Screen::RaceDisplay => race_display_screen(ctx, dem, scr),
            Screen::ClassDetermination => class_determination_screen(ctx, dem, scr),
            Screen::ClassDisplay => class_display_screen(ctx, dem, scr),
            Screen::FactDisplay => fact_screen(ctx, dem, scr),
            Screen::MissedFactDisplay => missed_fact_screen(ctx, dem, scr,),
            Screen::End => end_screen(ctx, dem, scr)
        }   
    }
}

/*
fn fact_screen(ctx: &egui::Context, demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::SidePanel::left("stat_panel").show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Your Vocar Card");
        });
        ui.separator();
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Race: ");
            ui.text_edit_singleline(&mut
                if let Some(r) = demo.race {
                    r.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Current Class: ");
            ui.text_edit_singleline(&mut
                if let Some(c) = demo.class_n {
                    c.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Starting Class: ");
            ui.text_edit_singleline(&mut
                if let Some(c) = demo.class_zero {
                    c.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Final Class: ");
            ui.text_edit_singleline(&mut
                if let Some(c) = demo.class_five {
                    c.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        for (i, class) in demo.history.iter().enumerate() {
            ui.horizontal(|ui| {
                let text: &str = &format!("Generation {}: ", i + 1);
                ui.label(text);
                ui.text_edit_singleline(&mut class.to_string());
                ui.text_style_height(&egui::TextStyle::Button)
            });
            ui.add_space(10.0);
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(
                if let Some(c) =  demo.class_n {
                    format!("Facts About The {}", c.to_string())
                } else {
                    "?".to_string()
                }
            )
        });

        ui.separator();

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.label("Welcome!\nTo!\nVocar!\n");

                ui.separator();

                ui.label("???");

                ui.separator();
                
                ui.label("Vocar!");
            });
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            if demo.history.len() < 5 {
                if ui.button("Next Generation!").clicked() {
                    *screen = Screen::GetClass
                };
            } else {
                if ui.button("You're Done!").on_hover_text("...Or are you?").clicked() {
                    *screen = Screen::ShowMissedFacts
                }
            }
            
        });
    });
}
*/
fn start_screen(ctx: &egui::Context, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);
            ui.heading("The Vocar");
            ui.label("Welcome to the Vocar! This is an activity that guests participated in at the Bob Moses Conference 2023.");
            ui.add_space(20.0);
            if ui.button("Begin Your Journey!").on_hover_text("Click to go to the next screen.").clicked() {
                *screen = Screen::RaceDetermination;
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
}

fn race_determination_screen(ctx: &egui::Context, demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);
            ui.heading("Getting A Race");
            ui.label("For this experience, you will be given a random race.");
            ui.add_space(20.0);
            if ui.button("Get My Race!").on_hover_text("Click to go to the next screen.").clicked() {
                demo.race = Some(rand::random());
                *screen = Screen::RaceDisplay;
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
}

fn race_display_screen(ctx: &egui::Context, demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);
            ui.heading(&format!("You are {} :).", demo.race.unwrap()));
            ui.label("This will be your race PERMANENTLY.");
            ui.add_space(20.0);
            if ui.button("Next!").on_hover_text("Click to go to the next screen.").clicked() {
                *screen = Screen::ClassDetermination;
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
}

fn class_determination_screen(ctx: &egui::Context, demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(180.0);
            if demo.class_zero.is_none() {
                ui.heading("Getting Your Starting Quintile");
                ui.label("For this experience, you will also be given a random starting quintile.");
                ui.add_space(20.0);
                if ui.button("Starting Quintile!").on_hover_text("Click to go to the next screen.").clicked() {
                    demo.class_zero = Some(rand::random());
                    demo.class_n = demo.class_zero;
                    *screen = Screen::ClassDisplay;
                }
            } else {
                ui.heading("Getting Your Next Quintile");
                ui.label("Your current Quintile has the current chances of change:");
                ui.label("Placeholde %'s");
                ui.add_space(20.0);
                if ui.button("New Quintile!").on_hover_text("Click to go to the next screen.").clicked() {
                    demo.next_gen();
                    *screen = Screen::ClassDisplay;
                }
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
}

fn class_display_screen(ctx: &egui::Context, demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(170.0);
            ui.heading(&format!("You are in the {}.", demo.class_n.unwrap()));
            ui.add_space(10.0);
            if demo.history.len() == 0 {
                ui.label("This starting Quintile is just a jumpoff point and may change for each subsequent generation.");
                ui.label("The chance to get a higher, equal or lower Quintile is not same.");
            } else {
                ui.label("This is your new Quintile for the nth Generation.");
                ui.label("The chance to get a higher, equal or lower Quintile is not same.");
            }
            ui.add_space(20.0);
            if ui.button("Next").on_hover_text("Click to go to the next screen.").clicked() {
                *screen = Screen::FactDisplay;
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
}

fn fact_screen(ctx: &egui::Context, demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::SidePanel::right("stat_sheet").resizable(false).default_width(400.0).show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Your Vocar Card");
        });
        ui.separator();
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Race: ");
            ui.text_edit_singleline(&mut
                if let Some(r) = demo.race {
                    r.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Current Class: ");
            ui.text_edit_singleline(&mut
                if let Some(c) = demo.class_n {
                    c.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Starting Class: ");
            ui.text_edit_singleline(&mut
                if let Some(c) = demo.class_zero {
                    c.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Final Class: ");
            ui.text_edit_singleline(&mut
                if let Some(c) = demo.class_five {
                    c.to_string()
                } else {
                    String::new()
                }
            );
        });
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        for (i, class) in demo.history.iter().enumerate() {
            ui.horizontal(|ui| {
                let text: &str = &format!("Generation {}: ", i + 1);
                ui.label(text);
                ui.text_edit_singleline(&mut class.to_string());
                ui.text_style_height(&egui::TextStyle::Button)
            });
            ui.add_space(10.0);
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Next!").on_hover_text("Click to go to the next screen.").clicked() {
            if demo.history.len() < 5 {
                *screen = Screen::ClassDetermination;
            } else {
                *screen = Screen::MissedFactDisplay;
            }
            
        }
    });
}

fn missed_fact_screen(ctx: &egui::Context, _demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("MFS");
        if ui.button("TODO").clicked() {
            *screen = Screen::End
        }
    });
}

fn end_screen(ctx: &egui::Context, demo: &mut demographic::Demo, screen: &mut Screen) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("ES");
        if ui.button("TODO").clicked() {
            demo.reset();
            *screen = Screen::Start
        }
    });
}