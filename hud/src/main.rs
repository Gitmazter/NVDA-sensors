


mod nvda;
use std::any::type_name;

use nvda::{nvda, NvdaInfo};
use eframe::{egui::{Context, Pos2, Rgba, Vec2, ViewportBuilder, WidgetText, Window}, Result};

#[derive(Default)]
struct  MyApp {}


impl MyApp{
    pub fn new(_cc: &eframe::CreationContext<'_>)->Self{
        Default::default()
    }
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}


impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        // let nvda_info = nvda();
        let nvda_info = match nvda() {
            Ok(info) => info,
            Err(e) => {
                eprintln!("Failed to get NVDA info: {:?}", e);
                return;
            }
        };

        let brand_clone = "Nvidia Gpu";
        let fan_1_clone = String::from("Fan 1 Speed: ") + &nvda_info.fan_speed_1.to_string() + "%";
        let fan_2_clone = String::from("Fan 2 Speed: ") + &nvda_info.fan_speed_2.to_string() + "%";
        let memory_used_clone = String::from("Memory Used: ") + &nvda_info.memory_used.to_string().split_at(4).0 + " GB";
        let memory_total_clone = String::from("Memory Available: ") + &nvda_info.memory_total.to_string().split_at(4).0 + " GB";
        let power_limit_clone = String::from("Power Limit: ") + &nvda_info.power_limit.to_string() + " W";
        let temp_board_clone = String::from("Board Temp: ") + &nvda_info.temp_board.to_string() + "°C";
        let gpu_max_clock_clone = String::from("GPU Max Clock: ") + &nvda_info.gpu_max_clock.to_string() + "MHz";
        let gpu_current_clock_clone = String::from("GPU Current Clock: ") + &nvda_info.gpu_current_clock.to_string() + "MHz";
        let memory_max_clock_clone = String::from("Memory Max Clock: ") + &nvda_info.memory_max_clock.to_string() + "MHz";
        let mem_current_clock_clone = String::from("Memory Current Clock: ") + &nvda_info.mem_current_clock.to_string() + "MHz";

        let window_size = Vec2::new(200.0, 300.0);   
        let default_pos = Pos2::new(50.0, 50.0);
        Window::new("AndziHud").auto_sized().title_bar(false).collapsible(false).default_size(window_size).default_pos(default_pos).fade_in(true).show(ctx, |ui| {
            ui.label("Andzi Hud v0.0.1");
                ui.label(brand_clone);
                ui.label(fan_1_clone);
                ui.label(fan_2_clone);
                ui.label(memory_used_clone);
                ui.label(memory_total_clone);
                ui.label(power_limit_clone);
                ui.label(temp_board_clone);
                ui.label(gpu_max_clock_clone);
                ui.label(gpu_current_clock_clone);
                ui.label(memory_max_clock_clone);
                ui.label(mem_current_clock_clone);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let size = Vec2::new(200.0, 300.0);
    let position = Pos2::new(50.0, 50.0);
    let vp =ViewportBuilder::default()
    .with_transparent(true)
    .with_decorations(false)
    .with_always_on_top()
    .with_mouse_passthrough(true)
    .with_window_type(eframe::egui::X11WindowType::Notification)
    .with_inner_size(size)
    .with_position(position)
    .with_mouse_passthrough(true)
    .with_fullscreen(true);

    eframe::run_native(
        "My App",
        eframe::NativeOptions {
            centered: true,
            viewport: vp,
            ..Default::default()
        },
        Box::new(|cc: &eframe::CreationContext<'_>|Ok(Box::new(MyApp::new(cc)))),
    )
}