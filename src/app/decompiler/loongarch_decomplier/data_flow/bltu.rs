use super::*;

pub fn bltu(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let operand1 = match insn.operand1 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand2 = match insn.operand2 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand3 = match insn.operand3 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let address = insn.address as i64 + (operand3.value << 2) as i64;
    let index1 = operand1.value as usize;
    let index2 = operand2.value as usize;

    let mut ir = DataFlowIr {
        address: insn.address,
        opcode: DataFlowIrOpcode::Jcc(Relation::L),
        operand1:None,
        operand2: None,
        operand3: Some(DFIOperand::Number(Number::from(address, false, Size::Unsigned64))),
    };

    match &gr_states[index1].value {
        RegisterRecord::Number(number) => {
            let mut num = number.clone();
            num.signed = false;
            num.size = set_unsigned(num.size);
            ir.operand1 = Some(DFIOperand::Number(num));
        } 
        RegisterRecord::Symbol(symbol) => {
            let mut sym = symbol.clone();
            sym.size = set_unsigned(sym.size);
            ir.operand1 = Some(DFIOperand::Symbol(sym));
        }
    }

    match &gr_states[index2].value {
        RegisterRecord::Number(number) => {
            let mut num = number.clone();
            num.signed = false;
            num.size = set_unsigned(num.size);
            ir.operand2 = Some(DFIOperand::Number(num));
        }
        RegisterRecord::Symbol(symbol) => {
            let mut sym = symbol.clone();
            sym.size = set_unsigned(sym.size);
            ir.operand2 = Some(DFIOperand::Symbol(sym));
        }
    }

    irs.push(ir);
}

fn set_unsigned(size: Size) -> Size {
    match size {
        Size::Signed8 => Size::Unsigned8,
        Size::Signed16 => Size::Unsigned16,
        Size::Signed32 => Size::Unsigned32,
        Size::Signed64 => Size::Unsigned64,
        _ => size,
    } 
}
