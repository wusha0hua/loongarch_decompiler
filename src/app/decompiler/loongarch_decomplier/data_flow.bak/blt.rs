use crate::loongarch_decomplier::*;

pub fn blt(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    let address = insn.address as isize + (operand3.value << 2) as isize;
    let index1 = operand1.value;
    let index2 = operand2.value;

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
            num.signed = true;
            num.size = set_signed(num.size);
            ir.operand1 = Some(DFIOperand::Number(num));
        } 
        RegisterRecord::Symbol(symbol) => {
            let mut sym = symbol.clone();
            sym.size = set_signed(sym.size);
            ir.operand1 = Some(DFIOperand::Symbol(sym));
        }
    }

    match &gr_states[index2].value {
        RegisterRecord::Number(number) => {
            let mut num = number.clone();
            num.signed = true;
            num.size = set_signed(num.size);
            ir.operand2 = Some(DFIOperand::Number(num));
        }
        RegisterRecord::Symbol(symbol) => {
            let mut sym = symbol.clone();
            sym.size = set_signed(sym.size);
            ir.operand2 = Some(DFIOperand::Symbol(sym));
        }
    }

    irs.push(ir);
}

fn set_signed(size: Size) -> Size {
    match size {
        Size::Unsigned8 => Size::Signed8,
        Size::Unsigned16 => Size::Signed16,
        Size::Unsigned32 => Size::Signed32,
        Size::Unsigned64 => Size::Signed64,
        _ => size,
    } 
}
