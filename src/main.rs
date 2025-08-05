mod hid;
use hid::*;

fn main() -> eframe::Result {

    //init handle here

    //get init brightness first
    let mut brightness: u32 = get_brightness()
        .expect("could not get current brightness");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([760.0, 75.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Let There Be Light", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(1, |columns| {
                columns[0].horizontal_centered(|ui| {
                    ui.heading("brightness");

                    ui.spacing_mut().slider_width = 600.0;
                    ui.spacing_mut().slider_rail_height = 20.0;
                    let slider = ui.add(egui::Slider::new(&mut brightness, 0..=100));
                    if slider.drag_stopped() {
                        set_brightness(brightness);
                    }
                })
            })
        });
    })
}

