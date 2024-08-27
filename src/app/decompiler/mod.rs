mod loongarch_decomplier;
pub use loongarch_decomplier::*;


use serde_json;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::{HashMap, HashSet};

use crate::app::elf::{self, SymbolType, RelactionType};
use crate::app::disassembler;

#[derive(Debug, Clone)]
pub struct DecompilerInfo {
    pub function_map_with_ir: HashMap<String, Vec<DataFlowIr>>,
    pub ast_map: HashMap<String, Ast>,
    pub cft_map: HashMap<String, ControlFlowTree>,
}

impl DecompilerInfo {
    pub fn new() -> Self {
        Self {
            function_map_with_ir: HashMap::new(),
            ast_map: HashMap::new(), 
            cft_map: HashMap::new(),
        }
    }

    pub fn from(elf: elf::Elf, diasm_info: disassembler::DisassemblerInfo) -> Self {
        /*
        let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/assembly_instructions").unwrap(); 
        let mut assembly_instructions_str = String::new();
        file.read_to_string(&mut assembly_instructions_str).unwrap();
        let assembly_instructions: HashMap<String, Vec<AssemblyInstruction>> = serde_json::from_str(&assembly_instructions_str).unwrap();
        */
        let assembly_instructions: HashMap<String, Vec<AssemblyInstruction>> = diasm_info.assembly_instructions;

        /*
        let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/symbol.json").unwrap();
        let mut symbols_str = String::new();
        file.read_to_string(&mut symbols_str).unwrap();
        let mut symbols: HashMap<usize, SymbolRecord> = serde_json::from_str(&symbols_str).unwrap();
        */
        let mut symbols: HashMap<u64, elf::SymbolRecord> = diasm_info.symbols_map;

        /*
        let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/dyn_symbol.json").unwrap();
        let mut symbols_str = String::new();
        file.read_to_string(&mut symbols_str).unwrap();
        let dyn_symbol: Vec<DynSymbolRecord> = serde_json::from_str(&symbols_str).unwrap();
        */
        let dyn_symbol: Vec<elf::DynSymbolRecord> = elf.dyn_symbols;

        /*
        let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/data.json").unwrap();
        let mut data_str = String::new();
        file.read_to_string(&mut data_str).unwrap();
        let data: HashMap<usize, Vec<u8>> = serde_json::from_str(&data_str).unwrap();
        */
        let data: HashMap<u64, Vec<u8>> = elf.data;
   


        let mut globals = HashMap::<u64, NameValue>::new();

        let mut functions = HashMap::<u64, NameValue>::new();
        for symbol in &symbols {
            if symbol.1.sym_type == SymbolType::Func && symbol.1.size != 0 {
                functions.insert(symbol.1.offset, NameValue::Name(symbol.1.name.clone()));
            }

            if symbol.1.sym_type == SymbolType::Val {
                globals.insert(symbol.1.offset, NameValue::Name(symbol.1.name.clone()));
            }
        }

        for dyn_sym in &dyn_symbol {
            match dyn_sym.reloc_type {
                RelactionType::R_LARCH_64 => {
                    if dyn_sym.sym_type == SymbolType::Func {
                        globals.insert(dyn_sym.offset, NameValue::Value(dyn_sym.value));
                        functions.insert(dyn_sym.value, NameValue::Name(dyn_sym.name.clone()));
                    } else if dyn_sym.sym_type == SymbolType::Val {
                        globals.insert(dyn_sym.offset, NameValue::Value(dyn_sym.value));
                        globals.insert(dyn_sym.value, NameValue::Name(dyn_sym.name.clone()));
                    }
                }
                RelactionType::R_LARCH_NONE => {}
                RelactionType::R_LARCH_RELATIVE => {}
                RelactionType::R_LARCH_JUMP_SLOT => {
                }
            }
        }


        
        let mut function_blocks = Vec::<FunctionBlock>::new();
        if let Some(instructions) = assembly_instructions.get(".text") {
            function_blocks = FunctionBlock::from(instructions.clone(), &symbols);
        }

        if let Some(instructions) = assembly_instructions.get(".plt") {
            analyse_plt(instructions, &mut functions, &dyn_symbol);
        }

        let mut function_map = HashMap::<u64, (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>)>::new();
        let mut temp_symbol_map = HashMap::<u64, HashSet<DFISymbolRecord>>::new();
        for function_block in &function_blocks {
            let (irs, gr_state, symbols) = analyse_data_flow(function_block.instruction.clone(), &functions, &globals, &data);
            let temp_symbols: HashSet<DFISymbolRecord> = symbols.clone().into_iter().filter(|s| {
                s.sym_type == DFISymbolType::Temp
            }).collect(); 
            temp_symbol_map.insert(function_block.address, temp_symbols);
            function_map.insert(function_block.address, (irs, gr_state, symbols));
        }
        
        let function_map: HashMap<u64, (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>)> = analyse_parameter(function_map, &functions);

        
        let mut function_list = HashSet::<String>::new();

        let mut function_map_with_ir = HashMap::<String, Vec<DataFlowIr>>::new();
        if SHOW_DATA_FLOW_IR_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- debug data flow ir ----------");
            for function in &function_map {

                if function_list.len() != 0 {
                    if let None = function_list.get(&symbols[function.0].name) {
                        continue;
                    }
                }

                let function_name = symbols[function.0].name.clone();
                println!("<{}>: ", function_name);
                function_map_with_ir.insert(function_name, function.1.0.clone());
                for ir in &function.1.0 {
                    println!("{:?}", ir);
                    println!("{}", ir);
                    println!("");
                }
                println!("");
            }
            println!("-----------------------------------------\n");
        }


        for function in function_map.iter() {
            
            if function_list.len() != 0 {
                if let None = function_list.get(&symbols[function.0].name) {
                    continue;
                }
            }

            if SHOW_DATA_FLOW_IR.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- data flow ir ----------");
                println!("<{}>: ", symbols[function.0].name);
                for ir in &function.1.0 {
                    println!("{}", ir);
                }
                println!("");
                println!("-----------------------------------\n");
            }
        }


        let mut function_blocks = HashMap::<u64, HashMap<usize, Block>>::new();
        for function in &function_map { 

            if function_list.len() != 0 {
                if let None = function_list.get(&symbols[function.0].name) {
                    continue;
                }
            }

            let irs = &function.1.0;
            let address = function.0;
            let blocks = get_blocks(irs);

            if SHOW_BLOCK_WITH_IR.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- block with ir ----------");
                for block in blocks.iter() {
                    println!("---------- block {} ----------", block.0);
                    for ir in block.1.irs.iter() {
                        println!("{}", ir);
                    }
                }
                println!("-------------------------------------\n");
            }

            function_blocks.insert(*address, blocks);
        }

        let mut function_control_flows = HashMap::<u64, ControlFlowGraph>::new();
        for function in function_blocks {
            let address = function.0;
            let blocks = function.1;
            let cfg = ControlFlowGraph::build_control_flow_graph(blocks);
            function_control_flows.insert(address, cfg);
        }


        let mut ast_map = HashMap::<String, Ast>::new();
        let mut cft_map = HashMap::<String, ControlFlowTree>::new();
        for function in function_control_flows.iter_mut() {
            let address = function.0;
            let cfg = function.1;


            let paths = get_cycle_paths(cfg);
            let function_name = match FUNCTIONS.lock().unwrap().get(&address){
                Some(NameValue::Name(name)) => name.clone(),
                Some(NameValue::Value(value)) => format!("func@{:x}", value),
                None => format!("func@{:x}", address),
            };


            if get_cycle_paths(&cfg).len() == 0 {
                let (topo, _topo) = topo_sort(cfg);
                cfg.topo_index = topo;
                cfg._topo_index = _topo;

                let mut n = cfg.nodes.len();

                let cft = get_control_flow_trees(cfg, &mut n, &HashMap::new());
                if SHOW_CONTROL_FLOW_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("\n-------- debug control flow tree ----------");
                    println!("{:#?}", cft);
                    println!("--------------------------------------------\n");
                }
                if SHOW_CONTROL_FLOW_TREE_AS_GRAPH.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("\n---------- control flow tree ----------");
                    println!("{}", cft);
                    println!("---------------------------------------\n");
                }

                let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
                let ast = AbstractSyntaxTree::from_cfg_tree(&cft, &cfg, function_name.clone(), &mut ast_symbol_map);
                ast_map.insert(function_name, Ast {
                    ast,
                    symbols: ast_symbol_map,
                });
            } else {
                let temps = temp_symbol_map[address].clone();
                let mut temps_map = HashMap::<usize, DFISymbolRecord>::new();
                for temp in temps {
                    let id = temp.id;
                    temps_map.insert(id, temp);
                }
                let function_name = match FUNCTIONS.lock().unwrap().get(&address){
                    Some(NameValue::Name(name)) => name.clone(),
                    Some(NameValue::Value(value)) => format!("func@{:x}", value),
                    None => format!("func@{:x}", address),
                };

                let mut structure_counter = Counter::new();
                let mut structure_condiction_map = HashMap::new();
                while get_cycle_paths(&cfg).len() != 0 {
                    //println!("path: {:?}", get_cycle_paths(&cfg));
                    println!("before restruct:");
                    //cfg.info();
                    structure_condiction_map = cfg.restruct_from_cycle(&mut structure_counter, &mut temps_map); 
                }



                if SHOW_CONTROL_FLOW_GRAPH_INFORMATION.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("\n---------- control flow graph information ----------");
                    println!("nodes: ");
                    for node in cfg.nodes.iter() {
                        let cfg_node = node.1;
                        println!("id: {} \ttype: {:?} ", cfg_node.id, cfg_node.node_type);
                        for ir in cfg_node.irs.iter() {
                            println!("{} \t", ir);
                        }
                        println!("\n");
                    }
                    println!("edges: ");
                    for edge in cfg.edges.iter() {
                        let cfg_edge = &edge.1;
                        if let Some(cond) = cfg_edge.condiction.as_ref() {
                            println!("id: {} \t{} -> {} \ttype: {:?}\tcondiction: {}\tis_true: {:?}", edge.0, cfg_edge.from, cfg_edge.to, cfg_edge.edge_type, cfg.condiction[cond], cfg_edge._true);
                        } else {
                            println!("id: {} \t{} -> {} \ttype: {:?}\tcondiction: {:?}\tis_true: {:?}", edge.0, cfg_edge.from, cfg_edge.to, cfg_edge.edge_type, cfg_edge.condiction, cfg_edge._true);
                        }
                    }
                    println!("------------------------------------------------------\n");
                }


                let (topo, _topo) = topo_sort(cfg);
                let mut n = cfg.nodes.len();
                
                let (loop_slices, loop_slices_map) = get_cfg_loop_slices(cfg);

                let mut cft_trees_map = HashMap::<usize, ControlFlowTree>::new();
                for (id, cfg_slice) in loop_slices.iter() {
                    let cft = get_control_flow_trees(cfg_slice, &mut n, &structure_condiction_map);
                    cft_trees_map.insert(*id, cft);
                } 

                let cft = ControlFlowTree::merge(&cft_trees_map, &loop_slices_map, &topo);
                cft_map.insert(function_name.clone(), cft.clone());
                if SHOW_CONTROL_FLOW_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("\n-------- debug control flow tree ----------");
                    println!("{:#?}", cft);
                    println!("--------------------------------------------\n");
                }
                if SHOW_CONTROL_FLOW_TREE_AS_GRAPH.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("\n---------- control flow tree ----------");
                    println!("{}", cft);
                    println!("---------------------------------------\n");
                }
                let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
                let ast = AbstractSyntaxTree::from_cfg_tree(&cft, &cfg, function_name.clone(), &mut ast_symbol_map);

                ast_map.insert(function_name, Ast {
                    ast,
                    symbols: ast_symbol_map,
                });
            }

        }

        let mut return_type_map = HashMap::<String, (ASTSymbolValueType, bool)>::new(); 
        for (name, ast) in ast_map.iter() {
            let mut target = AbstractSyntaxTree::new();
            &ast.ast.search_type(&ASTType::EndReturn, &mut target); 
            if target != AbstractSyntaxTree::new() {
                match &target.next.first() {
                    Some(next) => {
                        match &next.ast_type {
                            ASTType::Variable(is_ptr) => {
                                let symobl = &ast.symbols[&(next.value as usize)];
                                return_type_map.insert(name.to_string(), (symobl.select_type.clone(), *is_ptr));
                            }
                            ASTType::Integer(is_ptr, size) => {
                                return_type_map.insert(name.to_string(), (size.clone(), *is_ptr));
                            }
                            a @ _ => {
                                panic!("type error: {:?}", a);
                            }
                        }
                    }
                    None => panic!("error"),
                }
            } else {
                return_type_map.insert(name.to_string(), (ASTSymbolValueType::Unknown, false));
            }
        }

        for (name, ast) in ast_map.iter_mut() {
            if let Some((var_type, is_ptr)) = return_type_map.get(name) {
                let mut return_ast = AbstractSyntaxTree::new();
                return_ast.ast_type = ASTType::FunctionReturn(var_type.clone(), *is_ptr);
                ast.ast.next.insert(0, Box::new(return_ast));
            }
        }

        for (name, ast) in ast_map.iter_mut() {
            ast.ast.update_return_var_type(&return_type_map, &mut ast.symbols); 
        }


        for (name, ast) in ast_map.iter() {
            if SHOW_ABSTRACT_SYNTAX_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- debug abstract syntax tree ----------");
                println!("{:#?}", ast.ast);
                println!("-------------------------------------------------\n");
            }

            if SHOW_ABSTRACT_SYNTAX_TREE.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- abstract syntax tree ----------");
                println!("{}", ast.ast.to_string(&ast.symbols));
                println!("-------------------------------------------\n");
            }
        }


        Self {
            function_map_with_ir,
            ast_map,
            cft_map,
        }
    }
}


/*
fn main() {
    let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/assembly_instructions").unwrap(); 
    let mut assembly_instructions_str = String::new();
    file.read_to_string(&mut assembly_instructions_str).unwrap();
    let assembly_instructions: HashMap<String, Vec<AssemblyInstruction>> = serde_json::from_str(&assembly_instructions_str).unwrap();

    let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/symbol.json").unwrap();
    let mut symbols_str = String::new();
    file.read_to_string(&mut symbols_str).unwrap();
    let mut symbols: HashMap<usize, SymbolRecord> = serde_json::from_str(&symbols_str).unwrap();

    let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/dyn_symbol.json").unwrap();
    let mut symbols_str = String::new();
    file.read_to_string(&mut symbols_str).unwrap();
    let dyn_symbol: Vec<DynSymbolRecord> = serde_json::from_str(&symbols_str).unwrap();

    let mut file = File::open("/disk/repository/loong/loongarch-decomplier/src/data.json").unwrap();
    let mut data_str = String::new();
    file.read_to_string(&mut data_str).unwrap();
    let data: HashMap<usize, Vec<u8>> = serde_json::from_str(&data_str).unwrap();
   


    let mut globals = HashMap::<usize, NameValue>::new();

    let mut functions = HashMap::<usize, NameValue>::new();
    for symbol in &symbols {
        if symbol.1.sym_type == SymbolType::Func && symbol.1.size != 0 {
            functions.insert(symbol.1.offset, NameValue::Name(symbol.1.name.clone()));
        }

        if symbol.1.sym_type == SymbolType::Val {
            globals.insert(symbol.1.offset, NameValue::Name(symbol.1.name.clone()));
        }
    }

    for dyn_sym in &dyn_symbol {
        match dyn_sym.reloc_type {
            RelactionType::R_LARCH_64 => {
                if dyn_sym.sym_type == SymbolType::Func {
                    globals.insert(dyn_sym.offset, NameValue::Value(dyn_sym.value));
                    functions.insert(dyn_sym.value, NameValue::Name(dyn_sym.name.clone()));
                } else if dyn_sym.sym_type == SymbolType::Val {
                    globals.insert(dyn_sym.offset, NameValue::Value(dyn_sym.value));
                    globals.insert(dyn_sym.value, NameValue::Name(dyn_sym.name.clone()));
                }
            }
            RelactionType::R_LARCH_NONE => {}
            RelactionType::R_LARCH_RELATIVE => {}
            RelactionType::R_LARCH_JUMP_SLOT => {
                /*
                if dyn_sym.sym_type == SymbolType::Func {
                    functions.insert(dyn_sym.offset, NameValue::Name(dyn_sym.name.clone()));
                }
                */
            }
        }
    }

    //let assembly_instructions_eliminated_redundancy = eliminate_redundacy(assembly_instructions, &symbols);

    
    let mut function_blocks = Vec::<FunctionBlock>::new();
    if let Some(instructions) = assembly_instructions.get(".text") {
        function_blocks = FunctionBlock::from(instructions.clone(), &symbols);
    }

    if let Some(instructions) = assembly_instructions.get(".plt") {
        analyse_plt(instructions, &mut functions, &dyn_symbol);
    }

    let mut function_map = HashMap::<usize, (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>)>::new();
    let mut temp_symbol_map = HashMap::<usize, HashSet<DFISymbolRecord>>::new();
    for function_block in &function_blocks {
        let (irs, gr_state, symbols) = analyse_data_flow(function_block.instruction.clone(), &functions, &globals, &data);
        /*
        if function_block.address == 0x120000708 {
            for sym in symbols.iter() {
                if sym.sym_type == DFISymbolType::Param {
                    println!("{:?}", sym);
                }
            }
            panic!("print from main.rs");
        }
        */
        let temp_symbols: HashSet<DFISymbolRecord> = symbols.clone().into_iter().filter(|s| {
            s.sym_type == DFISymbolType::Temp
        }).collect(); 
        temp_symbol_map.insert(function_block.address, temp_symbols);
        function_map.insert(function_block.address, (irs, gr_state, symbols));
    }
    
    let function_map: HashMap<usize, (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>)> = analyse_parameter(function_map, &functions);

        /*
        println!("\n\n\n");
        println!("print from main.rs");
        for symbol in &(&function.1).2 {
            //if symbol.sym_type == DFISymbolType::Param {
                println!("{:?}", symbol);
            //}
        }
        */
    
    let mut function_list = HashSet::<String>::new();
    //function_list.insert("main".to_string());

    if SHOW_DATA_FLOW_IR_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
        println!("\n---------- debug data flow ir ----------");
        for function in &function_map {

            if function_list.len() != 0 {
                if let None = function_list.get(&symbols[function.0].name) {
                    continue;
                }
            }

            println!("<{}>: ", symbols[function.0].name);
            for ir in &function.1.0 {
                println!("{:?}", ir);
                println!("{}", ir);
                println!("");
            }
            println!("");
        }
        println!("-----------------------------------------\n");
    }


    for function in function_map.iter() {
        
        if function_list.len() != 0 {
            if let None = function_list.get(&symbols[function.0].name) {
                continue;
            }
        }

        if SHOW_DATA_FLOW_IR.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- data flow ir ----------");
            println!("<{}>: ", symbols[function.0].name);
            for ir in &function.1.0 {
                println!("{}", ir);
            }
            println!("");
            println!("-----------------------------------\n");
        }
    }

    /*
    for function in &function_map {
        println!("<{}>: ", symbols[function.0].name);
        for sym in &function.1.2 {
            if sym.sym_type == DFISymbolType::Param {
            println!("{:?}", sym);
            }
        }
        println!("");

    }
    */

    let mut function_blocks = HashMap::<usize, HashMap<usize, Block>>::new();
    for function in &function_map { 

        if function_list.len() != 0 {
            if let None = function_list.get(&symbols[function.0].name) {
                continue;
            }
        }

        let irs = &function.1.0;
        let address = function.0;
        let blocks = get_blocks(irs);

        if SHOW_BLOCK_WITH_IR.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- block with ir ----------");
            for block in blocks.iter() {
                println!("---------- block {} ----------", block.0);
                for ir in block.1.irs.iter() {
                    println!("{}", ir);
                }
            }
            println!("-------------------------------------\n");
        }

        function_blocks.insert(*address, blocks);
        /*
        let mut start: Vec<usize> = start.into_iter().collect();
        let mut end: Vec<usize> = end.into_iter().collect();
        start.sort();
        end.sort();
        */

        /*
        println!("{}", symbols[address].name);
        println!("\nstart:");
        for s in start {
            println!("{:x}", s);
        }
        println!("\nend:");
        for e in end {
            println!("{:x}", e);
        }
        */
        /*
        for b in blocks.iter() {
            println!("address: {}", b.address);
            println!("next: {:?}", b.next);
            println!("condiction: {:?}", b.condiction);
            println!("true_next: {:?}", b.true_next);
            println!("false_next: {:?}", b.false_next);
            println!("\n");
        }
        */
        //function_blocks.insert(*address, blocks);
    }

    let mut function_control_flows = HashMap::<usize, ControlFlowGraph>::new();
    for function in function_blocks {
        let address = function.0;
        let blocks = function.1;
        let cfg = ControlFlowGraph::build_control_flow_graph(blocks);
        function_control_flows.insert(address, cfg);
    }


    let mut ast_map = HashMap::<String, Ast>::new();
    for function in function_control_flows.iter_mut() {
        let address = function.0;
        let cfg = function.1;

        /*
        if *address == 0x120000650 {
            panic!("");
        } else {
            println!("address: {:x}", address);
        }
        */

        let paths = get_cycle_paths(cfg);
        let function_name = match FUNCTIONS.lock().unwrap().get(&address){
            Some(NameValue::Name(name)) => name.clone(),
            Some(NameValue::Value(value)) => format!("func@{:x}", value),
            None => format!("func@{:x}", address),
        };

        //cfg.info();
        //println!("{:?}", paths);
        
        //println!("{}", function_name);

        if get_cycle_paths(&cfg).len() == 0 {
            let (topo, _topo) = topo_sort(cfg);
            cfg.topo_index = topo;
            cfg._topo_index = _topo;
            //println!("{:?}", cfg.topo_index);
            //println!("{:?}", _topo);

            let mut n = cfg.nodes.len();

            let cft = get_control_flow_trees(cfg, &mut n, &HashMap::new());
            //println!("{:#?}", cft);
            if SHOW_CONTROL_FLOW_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n-------- debug control flow tree ----------");
                println!("{:#?}", cft);
                println!("--------------------------------------------\n");
            }
            if SHOW_CONTROL_FLOW_TREE_AS_GRAPH.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- control flow tree ----------");
                println!("{}", cft);
                println!("---------------------------------------\n");
            }

            let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
            let ast = AbstractSyntaxTree::from_cfg_tree(&cft, &cfg, function_name.clone(), &mut ast_symbol_map);
            ast_map.insert(function_name, Ast {
                ast,
                symbols: ast_symbol_map,
            });
            /*
            if SHOW_ABSTRACT_SYNTAX_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- debug abstract syntax tree ----------");
                println!("{:#?}", ast);
                println!("-------------------------------------------------\n");
            }

            if SHOW_ABSTRACT_SYNTAX_TREE.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- abstract syntax tree ----------");
                println!("{}", ast.to_string(&ast_symbol_map));
                println!("-------------------------------------------\n");
            }
            */
        } else {
            let temps = temp_symbol_map[address].clone();
            let mut temps_map = HashMap::<usize, DFISymbolRecord>::new();
            for temp in temps {
                let id = temp.id;
                temps_map.insert(id, temp);
            }
            let function_name = match FUNCTIONS.lock().unwrap().get(&address){
                Some(NameValue::Name(name)) => name.clone(),
                Some(NameValue::Value(value)) => format!("func@{:x}", value),
                None => format!("func@{:x}", address),
            };

            /*
            println!("nodes: ");
            for v in cfg.nodes.iter() {
                print!("{} ", v.0);
            }
            println!("\nedges: ");
            for e in cfg.edges.iter() {
                println!("{} -> {}", e.1.from, e.1.to);
            }
            println!("paths: {:?}", paths);
            */
            //println!("{}: {:?}", function_name, paths);
            //println!("before restruct:");
            //cfg.info();
            let mut structure_counter = Counter::new();
            let mut structure_condiction_map = HashMap::new();
            while get_cycle_paths(&cfg).len() != 0 {
                //println!("path: {:?}", get_cycle_paths(&cfg));
                println!("before restruct:");
                //cfg.info();
                structure_condiction_map = cfg.restruct_from_cycle(&mut structure_counter, &mut temps_map); 
            }



            //println!("path: {:?}", get_cycle_paths(&cfg));
            //println!("after restruct: ");
            //cfg.info();


            if SHOW_CONTROL_FLOW_GRAPH_INFORMATION.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- control flow graph information ----------");
                println!("nodes: ");
                for node in cfg.nodes.iter() {
                    let cfg_node = node.1;
                    println!("id: {} \ttype: {:?} ", cfg_node.id, cfg_node.node_type);
                    for ir in cfg_node.irs.iter() {
                        println!("{} \t", ir);
                    }
                    println!("\n");
                }
                println!("edges: ");
                for edge in cfg.edges.iter() {
                    let cfg_edge = &edge.1;
                    if let Some(cond) = cfg_edge.condiction.as_ref() {
                        println!("id: {} \t{} -> {} \ttype: {:?}\tcondiction: {}\tis_true: {:?}", edge.0, cfg_edge.from, cfg_edge.to, cfg_edge.edge_type, cfg.condiction[cond], cfg_edge._true);
                    } else {
                        println!("id: {} \t{} -> {} \ttype: {:?}\tcondiction: {:?}\tis_true: {:?}", edge.0, cfg_edge.from, cfg_edge.to, cfg_edge.edge_type, cfg_edge.condiction, cfg_edge._true);
                    }
                    //println!("id: {}\t{} -> {} type: {:?}\ncondiction: {:?}\tis_true: {:?}\n", edge.0, cfg_edge.from, cfg_edge.to, cfg_edge.edge_type, , cfg_edge._true);
                }
                println!("------------------------------------------------------\n");
            }


            //println!("{}", function_name);
            //cfg.info();
            //println!("\n{:x}", address);
            let (topo, _topo) = topo_sort(cfg);
            let mut n = cfg.nodes.len();
            //println!("{:?}", topo);
            //println!("{:?}", _topo);
            
            let (loop_slices, loop_slices_map) = get_cfg_loop_slices(cfg);
            //println!("{:?}", loop_slices_map);

            /*
            for (id, slice) in loop_slices.iter() {
                println!("{}", id); 
                slice.info();
            }
            */

            let mut cft_trees_map = HashMap::<usize, ControlFlowTree>::new();
            for (id, cfg_slice) in loop_slices.iter() {
                //println!("{}", id);
                /*
                if *id != 0 {
                    continue;
                }
                */

                let cft = get_control_flow_trees(cfg_slice, &mut n, &structure_condiction_map);
                //println!("{:#?}", cft);
                cft_trees_map.insert(*id, cft);
            } 

            let cft = ControlFlowTree::merge(&cft_trees_map, &loop_slices_map, &topo);
            //println!("{:#?}", cft);
            
            //panic!("");
            
            /*
            println!("-----------------------------------------");
            println!("-----------------------------------------");
            */
            //let cft = get_control_flow_trees(&cfg);
            //println!("{:#?}", cft);
            if SHOW_CONTROL_FLOW_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n-------- debug control flow tree ----------");
                println!("{:#?}", cft);
                println!("--------------------------------------------\n");
            }
            if SHOW_CONTROL_FLOW_TREE_AS_GRAPH.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- control flow tree ----------");
                println!("{}", cft);
                println!("---------------------------------------\n");
            }
            let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
            let ast = AbstractSyntaxTree::from_cfg_tree(&cft, &cfg, function_name.clone(), &mut ast_symbol_map);

            ast_map.insert(function_name, Ast {
                ast,
                symbols: ast_symbol_map,
            });
            /*
            if SHOW_ABSTRACT_SYNTAX_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- debug abstract syntax tree ----------");
                println!("{:#?}", ast);
                println!("-------------------------------------------------\n");
            }

            if SHOW_ABSTRACT_SYNTAX_TREE.load(std::sync::atomic::Ordering::SeqCst) {
                println!("\n---------- abstract syntax tree ----------");
                println!("{}", ast.to_string(&ast_symbol_map));
                println!("-------------------------------------------\n");
            }
            */
        }

        /*
        let (topo, _topo) = topo_sort(cfg);
        cfg.topo_index = topo;
        cfg._topo_index = _topo;
        //println!("{:?}", topo);
        //println!("{:?}", _topo);

        let function_name = match FUNCTIONS.lock().unwrap().get(&address){
            Some(NameValue::Name(name)) => name.clone(),
            Some(NameValue::Value(value)) => format!("func@{:x}", value),
            None => format!("func@{:x}", address),
        };
        let cft = get_control_flow_trees(cfg);
        if SHOW_CONTROL_FLOW_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n-------- debug control flow tree ----------");
            println!("{:#?}", cft);
            println!("--------------------------------------------\n");
        }
        if SHOW_CONTROL_FLOW_TREE_AS_GRAPH.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- control flow tree ----------");
            println!("{}", cft);
            println!("---------------------------------------\n");
        }

        let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
        let ast = AbstractSyntaxTree::from_cfg_tree(&cft, &cfg, function_name, &mut ast_symbol_map);
        if SHOW_ABSTRACT_SYNTAX_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- debug abstract syntax tree ----------");
            println!("{:#?}", ast);
            println!("-------------------------------------------------\n");
        }

        if SHOW_ABSTRACT_SYNTAX_TREE.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- abstract syntax tree ----------");
            println!("{}", ast.to_string(&ast_symbol_map));
            println!("-------------------------------------------\n");
        }
        */
    }

    let mut return_type_map = HashMap::<String, (ASTSymbolValueType, bool)>::new(); 
    for (name, ast) in ast_map.iter() {
        let mut target = AbstractSyntaxTree::new();
        &ast.ast.search_type(&ASTType::EndReturn, &mut target); 
        if target != AbstractSyntaxTree::new() {
            match &target.next.first() {
                Some(next) => {
                    match &next.ast_type {
                        ASTType::Variable(is_ptr) => {
                            let symobl = &ast.symbols[&next.value];
                            /*
                            println!("{}", name);
                            println!("{:?}", symobl);
                            println!("{:?}", next);
                            */
                            return_type_map.insert(name.to_string(), (symobl.select_type.clone(), *is_ptr));
                        }
                        ASTType::Integer(is_ptr, size) => {
                            return_type_map.insert(name.to_string(), (size.clone(), *is_ptr));
                            //println!("{}", name);
                            //println!("{:?}", next);       
                        }
                        a @ _ => {
                            panic!("type error: {:?}", a);
                        }
                    }
                }
                None => panic!("error"),
            }
        } else {
            return_type_map.insert(name.to_string(), (ASTSymbolValueType::Unknown, false));
        }
    }

    for (name, ast) in ast_map.iter_mut() {
        if let Some((var_type, is_ptr)) = return_type_map.get(name) {
            let mut return_ast = AbstractSyntaxTree::new();
            return_ast.ast_type = ASTType::FunctionReturn(var_type.clone(), *is_ptr);
            //println!("{}: {:?}", name, return_ast);
            ast.ast.next.insert(0, Box::new(return_ast));
        }
    }

    for (name, ast) in ast_map.iter_mut() {
        ast.ast.update_return_var_type(&return_type_map, &mut ast.symbols); 
    }

    /*
    let mut queue = Vec::<String>::new();
    let mut marked = HashSet::<String>::new();
    queue.push("main".to_string());
    marked.insert("main".to_string());
    let mut paramters_map = HashMap::<String, Vec<ASTSymbolValueType>>::new();
    while queue.len() != 0 {
        queue.reverse();
        let caller_name = queue.pop().unwrap();
        queue.reverse();
        if let Some(ast) = ast_map.get(&caller_name) {
            for next in ast.ast.next.iter() {
                if let ASTType::Function(callee_name) = &next.ast_type {
                    if let None = marked.get(callee_name) {
                        queue.push(callee_name.to_string());
                        marked.insert(callee_name.to_string());
                    } else {
                        continue;
                    }
                    let mut type_vec = Vec::<ASTSymbolValueType>::new();
                    for next in next.next.iter() {
                        match &next.ast_type {
                            ASTType::Variable(is_ptr) => {
                                let id = next.value;
                                //println!("caller: {} callee: {}", caller_name, callee_name);
                                //println!("{:?} {}", ast.symbols[&id], is_ptr);
                                type_vec.push(ast.symbols[&id].select_type.clone());
                            }
                            ASTType::Integer(is_ptr, _type) => {
                                type_vec.push(_type.clone());
                            }
                            _ => {}
                        }
                    }
                    paramters_map.insert(callee_name.to_string(), type_vec);
                }
            }
        }
    }

    //println!("{:?}", paramters_map);
    let mut paramters_index_map = HashMap::<String, HashSet<usize>>::new();
    for (name, ast) in ast_map.iter_mut() {
        //println!("{:?}\n", ast.symbols);
        //println!("{}", name);
        for next in ast.ast.next.iter_mut() {
            if let ASTType::Parameter = next.ast_type {
                let len = next.next.len();
                if len == 0 {
                    continue;
                }
                let paramters_types = match paramters_map.get(name) {
                    Some(types) => types,
                    None => continue,
                };
                let mut index_set = HashSet::<usize>::new();
                for i in 0..len {
                    let id = next.next[i].value;
                    let sym = ast.symbols.entry(id).or_insert(ASTSymbol::new(usize::MAX));
                    //let paramters_types = paramters_map.get(name).unwrap();
                    sym.select_type = paramters_types[i].clone();
                    //println!("{:?}", sym.select_type);
                    index_set.insert(id);
                }
                paramters_index_map.insert(name.to_string(), index_set);
                break;
            }
        }
    }

    //println!("{:?}", paramters_index_map);
    let mut paramters_update_map = HashMap::<String, HashMap<usize, ASTSymbolValueType>>::new();
    for (name, ast) in ast_map.iter() {
        //println!("{}: ", name);
        if let Some(index_set) = paramters_index_map.get(name) {
            let mut updata_map = HashMap::<usize, ASTSymbolValueType>::new();
            let mut index_set = index_set.clone();
            let mut i = 0;
            let len = ast.ast.next.len();
            
            while (i < len) && (index_set.len() != 0) {
                if let ASTType::Assign(is_ptr) = &ast.ast.next[i].ast_type {
                    if let Some(tree) = &ast.ast.next[i].next.first() {
                        let id = match index_set.get(&tree.value) {
                            Some(id) => *id,
                            None => continue,
                        };

                        let assign_sym = &ast.symbols[&ast.ast.next[i].value];
                        let parameter_sym = &ast.symbols[&id];
                        
                        /*
                        let _type = match &assign_sym.select_type {
                            ASTSymbolValueType::UnsignedChar => ASTSymbolValueType::UnsignedChar,
                            ASTSymbolValueType::Char => ASTSymbolValueType::Char,
                            ASTSymbolValueType::UnsignedShort => ASTSymbolValueType::UnsignedShort,
                            ASTSymbolValueType::Short => ASTSymbolValueType::Short,
                            ASTSymbolValueType::UnsignedInt => ASTSymbolValueType::UnsignedInt,
                            ASTSymbolValueType::Int => ASTSymbolValueType::Int,
                            ASTSymbolValueType::UnsignedLong => ASTSymbolValueType::UnsignedLong,
                            ASTSymbolValueType::Long => ASTSymbolValueType::Long,
                            ASTSymbolValueType::PtrUnsignedChar => ASTSymbolValueType::PtrUnsignedChar,
                            ASTSymbolValueType::PtrChar => ASTSymbolValueType::PtrChar,
                            ASTSymbolValueType::PtrUnsignedShort => ASTSymbolValueType::PtrUnsignedShort,
                            ASTSymbolValueType::PtrShort => ASTSymbolValueType::PtrShort,
                            ASTSymbolValueType::PtrUnsignedInt => ASTSymbolValueType::PtrUnsignedInt,
                            ASTSymbolValueType::PtrInt => ASTSymbolValueType::PtrInt,
                            ASTSymbolValueType::PtrUnsignedLong => ASTSymbolValueType::PtrUnsignedLong,
                            ASTSymbolValueType::PtrLong => ASTSymbolValueType::PtrLong,
                            ASTSymbolValueType::Unknown => ASTSymbolValueType::Unknown,
                        };
                        */

                        updata_map.insert(id, assign_sym.select_type.clone());

                        index_set.remove(&id);
                    }
                }
                i += 1;
            } 

            paramters_update_map.insert(name.to_string(), updata_map);
            /*
            for next in ast.ast.next.iter() {
                if let ASTType::Assign(is_ptr) = &next.ast_type {
                    if let Some(tree) = next.next.first() { 
                    } 
                }
            }
            */
        }
    }


    for (name, ast) in ast_map.iter_mut() {
        if let Some(updata_map) = paramters_update_map.get(name) {
            let mut updata_map = updata_map.clone();
            for next in ast.ast.next.iter_mut() {
                if let ASTType::Parameter = &next.ast_type {
                    for next in next.next.iter_mut() {
                        let id = next.value;
                        let sym = ast.symbols.entry(id).or_insert(ASTSymbol::new(usize::MAX));
                        if let Some(_type) = updata_map.get(&id) {
                            sym.select_type = _type.clone();
                        }
                    }
                    break;
                }
            }
        }
    }
 

    //println!("{:?}", paramters_map);
    /*
    for (name, ast) in ast_map.iter() {
        for next in ast.ast.next.iter() {
            if let ASTType::Function(func_name) = &next.ast_type {
                //println!("{:?}", next);
                for next in next.next.iter() {
                    if let ASTType::Variable(is_ptr) = &next.ast_type {
                        let id = next.value;
                        println!("{}: {:?} {}", func_name, ast.symbols[&id], is_ptr);
                    }
                }
            }
        }
    }
    */
    */
    
    

    for (name, ast) in ast_map.iter() {
        if SHOW_ABSTRACT_SYNTAX_TREE_BY_DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- debug abstract syntax tree ----------");
            println!("{:#?}", ast.ast);
            println!("-------------------------------------------------\n");
        }

        if SHOW_ABSTRACT_SYNTAX_TREE.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- abstract syntax tree ----------");
            println!("{}", ast.ast.to_string(&ast.symbols));
            println!("-------------------------------------------\n");
        }
    }


}
*/

#[derive(Debug, Clone)]
pub enum NameValue {
    Name(String),
    Value(u64),
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub ast: AbstractSyntaxTree,
    pub symbols: HashMap<usize, ASTSymbol>,
}
