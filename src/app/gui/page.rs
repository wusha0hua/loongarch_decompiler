use crate::app;

pub fn show_pages_bar(ui: &mut egui::Ui, app: &mut app::App) {
    if app.page == app::Page::DisamInst {
        ui.group(|ui| {
            if ui.button("disam inst").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::DisamInst);
            }
        });
    } else {
        if ui.button("disam inst").clicked() {
            app.page = app::Page::DisamInst; 
            app.update_page.clear();
            app.update_page.insert(app::Page::DisamInst);
        }
    }

    if app.page == app::Page::DisamInstDebug {
        ui.group(|ui| {
            if ui.button("disam inst debug").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::DisamInstDebug);
            }
        });
    } else {
        if ui.button("disam inst debug").clicked() {
            app.page = app::Page::DisamInstDebug;
            app.update_page.clear();
            app.update_page.insert(app::Page::DisamInstDebug);
        }
    }

    if app.page == app::Page::DataFlowIr {
        ui.group(|ui| {
            if ui.button("ir").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::DataFlowIr);
            }
        });
    } else {
        if ui.button("ir").clicked() {
            app.page = app::Page::DataFlowIr;
            app.update_page.clear();
            app.update_page.insert(app::Page::DataFlowIr);
        }
    }

    if app.page == app::Page::DataFlowIrDebug {
        ui.group(|ui| {
            if ui.button("ir debug").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::DataFlowIrDebug);
            }
        });
    } else {
        if ui.button("ir debug").clicked() {
            app.page = app::Page::DataFlowIrDebug;
            app.update_page.clear();
            app.update_page.insert(app::Page::DataFlowIrDebug);
        }
    }

    if app.page == app::Page::CFT {
        ui.group(|ui| {
            if ui.button("cft").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::CFT);
            }
        });
    } else {
        if ui.button("cft").clicked() {
            app.page = app::Page::CFT;
            app.update_page.clear();
            app.update_page.insert(app::Page::CFT);
        }
    }

    if app.page == app::Page::CFTDebug {
        ui.group(|ui| {
            if ui.button("cft debug").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::CFTDebug);
            }
        });
    } else {
        if ui.button("cft debug").clicked() {
            app.page = app::Page::CFTDebug;
            app.update_page.clear();
            app.update_page.insert(app::Page::CFTDebug);
        }
    }

    if app.page == app::Page::ASTDebug {
        ui.group(|ui| {
            if ui.button("ast debug").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::ASTDebug);
            }
        });
    } else {
        if ui.button("ast debug").clicked() {
            app.page = app::Page::ASTDebug;
            app.update_page.clear();
            app.update_page.insert(app::Page::ASTDebug);
        }
    }

    if app.page == app::Page::Code {
        ui.group(|ui| {
            if ui.button("code").clicked() {
                app.update_page.clear();
                app.update_page.insert(app::Page::Code);
            }
        });
    } else {
        if ui.button("code").clicked() {
            app.page = app::Page::Code;
            app.update_page.clear();
            app.update_page.insert(app::Page::Code);
        }
    }
}

pub fn show_page(ui: &mut egui::Ui, app: &mut app::App) {
    let text_style = egui::TextStyle::Body;
    let text_height = ui.text_style_height(&text_style);
    match &app.page {
        app::Page::DisamInst => {
            if let Some(_) = app.update_page.get(&app::Page::DisamInst) {
                let mut insn_vec = Vec::<String>::new();
                for (section_name, insns) in app.disassembler.assembly_instructions.iter() {
                    insn_vec.push(format!("<{}>:", section_name));
                    for insn in insns.iter() {
                        insn_vec.push(format!("{}", insn));
                    }
                }
                let len = insn_vec.len();
                egui::ScrollArea::vertical().id_source("disam inst").auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                    for row in range {
                        ui.label(&insn_vec[row]);
                    }        
                });
            }
        }
        app::Page::DisamInstDebug => {
            if let Some(_) = app.update_page.get(&app::Page::DisamInstDebug) {
                let mut insn_debug_vec = Vec::<String>::new();
                for (section_name, insns) in app.disassembler.assembly_instructions.iter() {
                    insn_debug_vec.push(format!("<{}>:", section_name));
                    for insn in insns.iter() {
                        let ir_d = format!("{:#?}", insn);
                        let ir_v: Vec<&str> = ir_d.split("\n").collect();
                        for v in ir_v {
                            insn_debug_vec.push(v.to_string());
                        }
                    }
                }
                
                let len = insn_debug_vec.len();
                egui::ScrollArea::vertical().id_source("disam insn debug").auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                    for row in range {
                        ui.label(&insn_debug_vec[row]);
                    }
                });
            }
        }
        app::Page::DataFlowIr => {
            if let Some(_) = app.update_page.get(&app::Page::DataFlowIr) {
                let mut ir_vec = Vec::<String>::new();
                for (function_name, irs) in app.decompiler.function_map_with_ir.iter() {
                    if *function_name == app.select_function {
                        for ir in irs.iter() {
                            ir_vec.push(format!("{}", ir));
                        }
                        
                        let len = ir_vec.len();
                        egui::ScrollArea::vertical().id_source("data flow ir").auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                            for row in range {
                                ui.label(&ir_vec[row]);
                            }
                        });
                    }
                }
            }
        }
        app::Page::DataFlowIrDebug => {
            if let Some(_) = app.update_page.get(&app::Page::DataFlowIrDebug) {
                let mut ir_debug_vec = Vec::<String>::new();
                for (function_name, irs) in app.decompiler.function_map_with_ir.iter() {
                    if *function_name == app.select_function {
                        for ir in irs.iter() {
                            let ir_debug = format!("{:#?}", ir);
                            let ir_d_v: Vec<&str> = ir_debug.split("\n").collect();
                            for ir_d in ir_d_v {
                                ir_debug_vec.push(ir_d.to_string());
                            }
                        }
                        
                        let len = ir_debug_vec.len();
                        egui::ScrollArea::vertical().id_source("data flow ir debug").auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                            for row in range {
                                ui.label(&ir_debug_vec[row]);
                            }
                        });
                    }
                }
            }
        }

        app::Page::CFT => {
            if let Some(_) = app.update_page.get(&app::Page::CFT) {
                let mut cft_str = Vec::<String>::new();
                for (name, cft) in app.decompiler.cft_map.iter() {
                    if *name == app.select_function {
                        let cft_string = format!("{}", cft);
                        let cft_vec: Vec<&str> = cft_string.split("\n").collect();
                        for v in cft_vec.iter() {
                            cft_str.push(v.to_string());
                        }

                        let len = cft_str.len();
                        egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                            for row in range {
                                ui.label(&cft_str[row]);
                            }
                        });
                    }
                }
            }
        }
        app::Page::CFTDebug => {
            if let Some(_) = app.update_page.get(&app::Page::CFTDebug) {
                let mut cft_debug_str = Vec::<String>::new();
                for (name, cft) in app.decompiler.cft_map.iter() {
                    if *name == app.select_function {
                        let cft_string = format!("{:#?}", cft);
                        let cft_vec: Vec<&str> = cft_string.split("\n").collect();
                        for v in cft_vec.iter() {
                            cft_debug_str.push(v.to_string());
                        }

                        let len = cft_debug_str.len();
                        egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                            for row in range {
                                ui.label(&cft_debug_str[row]);
                            }
                        });
                    }
                }
            }
        }
        app::Page::ASTDebug => {
            if let Some(_) = app.update_page.get(&app::Page::ASTDebug) {
                let mut ast_debug = Vec::<String>::new();
                for (name, ast) in app.decompiler.ast_map.iter() {
                    if *name == app.select_function {
                        let ast_d_str = format!("{:#?}", ast);
                        let ast_d_vec: Vec<&str> = ast_d_str.split("\n").collect();
                        for ast_d in ast_d_vec.iter() {
                            ast_debug.push(ast_d.to_string());
                        }
                        let len = ast_debug.len();
                        egui::ScrollArea::vertical().id_source("ast debug").auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                            for row in range {
                                ui.label(&ast_debug[row]);
                                /*
                                ui.push_id(row, |ui| {
                                    ui.label(&ast_debug[row]);
                                });
                                */
                            }
                        });
                    }
                }
            }
        }
        app::Page::Code => {
            if let Some(_) = app.update_page.get(&app::Page::Code) {
                let mut code = Vec::<String>::new();
                for (name, ast) in app.decompiler.ast_map.iter() {
                    if *name == app.select_function {
                        let code_str = format!("{}", ast.ast.to_string(&ast.symbols));
                        let code_v: Vec<&str> = code_str.split("\n").collect();
                        for c in code_v.iter() {
                            code.push(c.to_string().replace("\t", "\t\t"));
                        }
                        let len = code.len();
                        egui::ScrollArea::vertical().id_source("code").auto_shrink([false; 2]).show_rows(ui, text_height, len, |ui, range| {
                            for row in range {
                                ui.label(&code[row]);
                            }
                        });
                    }
                }
            }
        }
    }
}

