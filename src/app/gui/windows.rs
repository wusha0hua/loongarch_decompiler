use crate::app;

pub fn show_file_dialog(frame: &mut eframe::Frame, ctx: &egui::Context, app: &mut app::App) {
    app::show_file_dialog();
    app.window_states.insert(app::Window::FileDialog, false);
}


pub fn show_load_elf_error_window(frame: &mut eframe::Frame, ctx: &egui::Context, app: &mut app::App) {
    egui::Window::new("load elf error").resizable(true).collapsible(true).show(ctx, |ui| {
        if ui.button("ok").clicked() {
            app.window_states.insert(crate::app::Window::LoadElfErr, false);
        }
    });
}
