mod elf;
mod disassembler;
mod decompiler;
pub mod javascript;
mod gui;

use wasm_bindgen::prelude::*;
use base64;
use std::collections::HashSet;

#[wasm_bindgen]
extern "C" {
    pub fn show_file_dialog();
    fn get_file_data() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//pub use serde::{Serializer, Deserializer};

use javascript::*;
use std::collections::HashMap;
use std::path::PathBuf;

use self::elf::LoongArchError;

#[derive(Debug, Clone)]
pub struct App {
    file_data: Vec<u8>,
    elf: elf::Elf,
    disassembler: disassembler::DisassemblerInfo,
    decompiler: decompiler::DecompilerInfo,

    window_states: HashMap<Window, bool>,
    states: HashMap<AppState, State>,
    page: Page,
    select_function: String,
    function_name_list: Vec<String>,
    update_page: HashSet<Page>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Window {
    FileDialog,
    LoadElfErr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    LoadElf, 
    Disam,
    Decompile,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    Success,
    Fail,
    None,
    ToDo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Page {
    DisamInst,
    DisamInstDebug,
    DataFlowIr,
    DataFlowIrDebug,
    CFT,
    CFTDebug,
    ASTDebug,
    Code,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_data: Vec::new(),
            elf: elf::Elf::new(),
            disassembler: disassembler::DisassemblerInfo::new(),
            decompiler: decompiler::DecompilerInfo::new(),

            window_states: HashMap::new(),
            states: HashMap::new(),
            page: Page::DisamInst,
            select_function: String::new(), 
            function_name_list: Vec::new(),
            update_page: HashSet::new(),
        }
    }

}

impl App {
    fn check_window_states(&mut self, /*ui: &mut egui::Ui,*/ ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(true) = self.window_states.get(&Window::FileDialog) {
            gui::windows::show_file_dialog(frame, ctx, self);
        }
        if let Some(true) = self.window_states.get(&Window::LoadElfErr) {
            gui::windows::show_load_elf_error_window(frame, ctx, self);            
        }
    }



    fn receive_file_data(&mut self) {
        let file_data = get_file_data();
        if file_data.len() != 0 {
            let mut data = Vec::<u8>::new();
            let len = file_data.len();
            let file_data_vec = file_data.as_bytes();
            for i in 0..(len / 2) {
                let mut num1 = file_data_vec[i * 2];
                let mut num2 = file_data_vec[i * 2 + 1];
                if num1 >= '0' as u8 && num1 <= '9' as u8 {
                    num1 -= 48;
                } else if num1 >= 'a' as u8 && num1 <= 'z' as u8 {
                    num1 -= 87;
                }
                if num2 >= '0' as u8 && num2 <= '9' as u8 {
                    num2 -= 48;
                } else if num2 >= 'a' as u8 && num2 <= 'z' as u8 {
                    num2 -= 87;
                }

                let c = num1 * 16 + num2;
                data.push(c);
            }
            self.file_data = data;
            
            self.states.insert(AppState::LoadElf, State::ToDo);
        }
    }

    fn check_app_states(&mut self) {
        if let Some(State::ToDo) = self.states.get(&AppState::LoadElf) {
            match elf::Elf::from(self.file_data.clone()) {
                Ok(elf) => {
                    log(&format!("{:?}", elf));
                    self.elf = elf;
                    self.states.insert(AppState::Disam, State::ToDo);
                }
                Err(e) => {
                    log(&format!("{:?}", e));
                    self.window_states.insert(Window::LoadElfErr, true);
                }
            }
            self.states.insert(AppState::LoadElf, State::None);
        }

        if let Some(State::ToDo) = self.states.get(&AppState::Disam) {
            match disassembler::DisassemblerInfo::from(&self.elf) {
                Ok(disam_info) => {
                    self.disassembler = disam_info;
                    log(&format!("{:?}", self.disassembler));
                }
                Err(_) => {
                    log(&format!("disassembler error"));
                }
            }  
            self.states.insert(AppState::Disam, State::None);
            self.states.insert(AppState::Decompile, State::ToDo);
        }

        if let Some(State::ToDo) = self.states.get(&AppState::Decompile) {
            self.decompiler = decompiler::DecompilerInfo::from(self.elf.clone(), self.disassembler.clone()); 
            log(&format!("{:?}", self.decompiler));
            let mut string = String::new();
            for (name, ast) in self.decompiler.ast_map.iter() {
                string += &format!("{}", ast.ast.to_string(&ast.symbols)); 
            }
            log(string.as_str());
            for (name, _) in self.decompiler.ast_map.iter() {
                self.function_name_list.push(name.clone());
            }
            self.states.insert(AppState::Decompile, State::None);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        self.check_window_states(ctx, frame);
        self.receive_file_data();
        self.check_app_states();
        egui::TopBottomPanel::top("top bar").show(ctx, |ui| {
            gui::top_bar::show_top_bar(ui, frame, self);
        });

        egui::SidePanel::right("Function").resizable(true).show(ctx, |ui| {
            let len = self.function_name_list.len();
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(ui, 0.01, len, |ui, range| {
                for row in range {
                    let name = &self.function_name_list[row];
                    if self.select_function == *name {
                        ui.group(|ui| {
                            if ui.button(name).clicked() {};
                        });
                    } else {
                        if ui.button(name).clicked() {
                            self.select_function = name.clone();
                        }
                    }
                    ui.separator();
                }
            }); 
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                gui::page::show_pages_bar(ui, self);            
            });
            ui.separator();
            gui::page::show_page(ui, self);
        }); 

    }
}
