use super::*;

pub fn jirl(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let mut ir = DataFlowIr {
        address: insn.address,
        opcode: DataFlowIrOpcode::Ret,
        operand1: None, 
        operand2: None,
        operand3: None,
    };

    if gr_states[4].state {
        match &gr_states[4].value {
            RegisterRecord::Number(number) => {
                ir.operand1 = Some(DFIOperand::Number(number.clone()));
            }

            RegisterRecord::Symbol(symbol) => {
                ir.operand1 = Some(DFIOperand::Symbol(symbol.clone()));
            }
        }
    }

    irs.push(ir);
}

