#![windows_subsystem = "windows"]

mod error;
mod hid;
use hid::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut studio_display_handle = get_studio_display()?;
    let percent = get_brightness_percent(&mut studio_display_handle).unwrap();

    //get init brightness first
    let mut brightness: u8 = percent;

    // let icon = image::open("icon.jpg")?.to_rgba8();
    // let (icon_width, icon_height) = icon.dimensions();

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([760.0, 75.0]).with_icon(egui::IconData {
    //         rgba: icon.into_raw(),
    //         width: icon_width,
    //         height: icon_height
    //     }),
    //     ..Default::default()
    // };

     let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([760.0, 75.0]),
        ..Default::default()
    };

    let _ = eframe::run_simple_native("Let There Be Light", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(1, |columns| {
                columns[0].horizontal_centered(|ui| {
                    ui.heading("brightness");

                    ui.spacing_mut().slider_width = 600.0;
                    ui.spacing_mut().slider_rail_height = 20.0;
                    let slider = ui.add(egui::Slider::new(&mut brightness, 0..=100));
                    if slider.drag_stopped() {
                        // set_brightness(brightness);
                        set_brightness_percent(&mut studio_display_handle, brightness).unwrap();
                    }
                })
            })
        });
    });

    Ok(())
}
