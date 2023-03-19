#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let icon = image::open("./assets/icons/apple-touch-icon.png").expect("Failed To Open Icon Path").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    
    let options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        initial_window_size: Some(egui::Vec2 { x: 600.0, y: 300.0 }),
        resizable: false,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        follow_system_theme: true,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Vocar",
        options,
        Box::new(|cc| Box::new(vocar::VocarApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "vocar_web", // hardcode it
            web_options,
            Box::new(|cc| Box::new(vocar::VocarApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}