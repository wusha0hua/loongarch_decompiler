pub use super::super::loongarch_decomplier::*;

#[derive(Debug, Clone)]
pub struct AssemblyInstructionEliminatedRedundancy {
    pub address: usize,
    pub label: Option<String>,
    pub opcode: Opcode,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
    pub operand3: Option<Operand>,
    pub operand4: Option<Operand>,
}



impl AssemblyInstructionEliminatedRedundancy {
    pub fn new() -> AssemblyInstructionEliminatedRedundancy {
        AssemblyInstructionEliminatedRedundancy {
            address: 0,
            label :None,
            opcode: Opcode::AND,
            operand1: None,
            operand2: None,
            operand3: None,
            operand4: None,
        }
    }

    pub fn from(assembly_instruction: AssemblyInstruction) -> AssemblyInstructionEliminatedRedundancy {
        AssemblyInstructionEliminatedRedundancy {
            address: assembly_instruction.address,
            label: assembly_instruction.label,
            opcode: assembly_instruction.opcode,
            operand1: assembly_instruction.operand1,
            operand2: assembly_instruction.operand2,
            operand3: assembly_instruction.operand3,
            operand4: assembly_instruction.operand4,
        }
    }
}


pub fn eliminate_redundacy(assembly_instructions: Vec<AssemblyInstruction>, symbols: &HashMap<usize, SymbolRecord>) -> Vec<AssemblyInstructionEliminatedRedundancy> {
    let mut assembly_instructions_eliminated_redundancy = Vec::<AssemblyInstructionEliminatedRedundancy>::new();

    let mut gr_state = Vec::<(bool, isize, SymbolRecord)>::new();
    let mut fr_state = Vec::<(bool, isize, SymbolRecord)>::new();

    reset_gr(&mut gr_state);
    reset_fr(&mut fr_state);

    let mut i = 0;
    for assembly_instruction in assembly_instructions {
        if let Some(record) = symbols.get(&assembly_instruction.address) {
            if let SymbolType::Func = record.sym_type {
                reset_gr(&mut gr_state);
                reset_fr(&mut fr_state);
            }
        }

        let (f, assembly_instruction_eliminate_redundancy) = set_register(assembly_instruction, &mut gr_state, &mut fr_state, symbols);

    }

    assembly_instructions_eliminated_redundancy
}

fn set_register(mut assembly_instruction: AssemblyInstruction, gr_state: &mut Vec<(bool, isize, SymbolRecord)>, fr_state: & mut Vec<(bool, isize, SymbolRecord)>, symbols: &HashMap<usize, SymbolRecord>) -> (bool, AssemblyInstructionEliminatedRedundancy) {
    
    match assembly_instruction.opcode {
        Opcode::PCADDU12I => {
            if let Some(oprand1) = &assembly_instruction.operand1 {
                if let Some(operand2) = &assembly_instruction.operand2 {
                    let value = assembly_instruction.address as isize + ((operand2.value as isize) << 12);
                    reset_gr_by_index(&mut gr_state[oprand1.value]);
                    gr_state[oprand1.value].0 = true; 
                    gr_state[oprand1.value].1 = value;
                    println!("{:x}", value);
                }
            }
            (true, AssemblyInstructionEliminatedRedundancy::new())
        }
        Opcode::LD_D => {
            let mut assembly_instruction_eliminate_redundancy = AssemblyInstructionEliminatedRedundancy::from(assembly_instruction.clone());
            if let Some(operand1) = &mut assembly_instruction.operand1 {
                if let Some(operand2) = &assembly_instruction.operand2 {
                    if let Some(operand3) = &assembly_instruction.operand3 {
                        if gr_state[operand2.value].0 {
                            reset_gr_by_index(&mut gr_state[operand1.value]);
                            let mut value = gr_state[operand2.value].1;
                            println!("{}", value);
                            value += operand3.value as isize; 
                            gr_state[operand1.value].0 = true;
                            gr_state[operand1.value].1 = value;
                            println!("{}", value);
                            if let Some(record) = symbols.get(&(value as usize)) {
                                operand1.symbol = Some(record.clone());
                                gr_state[operand1.value].2 = record.clone();
                                println!("{:#?}", record);
                            }
                        }
                    }
                }
            }
            (false, assembly_instruction_eliminate_redundancy)
        }
        _ => (false, AssemblyInstructionEliminatedRedundancy::from(assembly_instruction))
    }

}


fn reset_gr(gr_state: &mut Vec<(bool, isize, SymbolRecord)>) {
    gr_state.clear();
    for i in 0..32 {
        gr_state.push((false, 0, SymbolRecord::new()));
    }
}

fn reset_fr(fr_state: &mut Vec<(bool, isize, SymbolRecord)>) {
    fr_state.clear();
    for i in 0..32 {
        fr_state.push((false, 0, SymbolRecord::new()));
    }
}

fn reset_gr_by_index(gr: &mut (bool, isize, SymbolRecord)) {
    gr.0 = false;
    gr.1 = 0;
    gr.2 = SymbolRecord::new();
}

fn reset_fr_by_index(fr: &mut (bool, isize, SymbolRecord)) {
    fr.0 = false;
    fr.1 = 0;
    fr.2 = SymbolRecord::new();
}

