use crate::app;

pub fn show_top_bar(ui: &mut egui::Ui, frame: &mut eframe::Frame, app: &mut app::App) {
    ui.menu_button("File", |ui| {
        if ui.button("Open").clicked() {
            app.window_states.insert(app::Window::FileDialog, true);
        }

        if ui.button("Quit").clicked() {
            frame.quit();
        }
    });
}

