//use crate::loongarch_decomplier::*;
//use crate::*;
use crate::app::decompiler::loongarch_decomplier::*;
use crate::app::decompiler::*;
use std::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::PathBuf;
use super::pre::*;

mod extern_function; 


#[derive(Debug, Clone)]
pub struct DataFlowIr{
    pub address: u64,
    pub opcode: DataFlowIrOpcode,
    pub operand1: Option<DFIOperand>,
    pub operand2: Option<DFIOperand>,
    pub operand3: Option<DFIOperand>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataFlowIrOpcode {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Not,
    Call,
    Load,
    Store,
    Ret,
    Jmp,
    Jcc(Relation),
    Function,
    Nop,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Relation {
    EQ,
    L,
    G,
    LE,
    GE,
    NE,
}

impl std::fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Relation::EQ => write!(f, "=="),
            Relation::L => write!(f, "<"),
            Relation::G => write!(f, ">"),
            Relation::LE => write!(f, "<="),
            Relation::GE => write!(f, ">="),
            Relation::NE => write!(f, "!="),
        }
    }
}

impl std::ops::Not for Relation {
    type Output = Self;
    
    fn not(self) -> Self::Output {
        match self {
            Relation::EQ => Relation::NE,
            Relation::NE => Relation::EQ,
            Relation::L => Relation::GE,
            Relation::G => Relation::LE,
            Relation::LE => Relation::G,
            Relation::GE => Relation::L,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)] 
pub enum DFIOperand {
    Symbol(DFISymbolRecord),
    Number(Number),
    Parameter(Vec<RegisterRecord>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number {
    pub value: i64,
    pub signed: bool,
    pub size: Size,
}

#[derive(Debug, Clone, Eq)]
pub struct DFISymbolRecord{
    pub address: Address, 
    pub sym_type: DFISymbolType,
    pub id: usize,
    pub size: Size,
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct DFISymbolRecordTable {
    pub symbols: HashSet<DFISymbolRecord>,
    pub loc_counter: Counter,
    pub global_counter: Counter,
    pub tmp_counter: Counter,
    //pub param_counter: Counter,
    //pub return_counter: Counter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Address {
    GR(usize),
    FR(usize),
    Stack(i64),
    Memory(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DFISymbolType {
    Param,
    Local,
    Temp,
    Global,
    Func,
    Label,
    Return,
    None,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Size {
    Signed8,
    Unsigned8,
    Signed16,
    Unsigned16,
    Signed32,
    Unsigned32,
    Signed64,
    Unsigned64,
    /*
    PtrSigned8,
    PtrUnsigned8,
    PtrSigned16,
    PtrUnsigned16,
    PtrSigned32,
    PtrUnsigned32,
    PtrSigned64,
    PtrUnsigned64,
    */
}

#[derive(Debug, Clone)]
pub struct GRRecord {
    pub value: RegisterRecord,
    pub state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegisterRecord {
    //Address(Address),
    Number(Number),
    Symbol(DFISymbolRecord),
}

lazy_static! {
    pub static ref FUNCTIONS: Mutex<HashMap<u64, NameValue>> = Mutex::new(HashMap::new());
    pub static ref GLOBALS: Mutex<HashMap<u64, NameValue>> = Mutex::new(HashMap::new());
    pub static ref DATA: Mutex<HashMap<u64, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub fn analyse_data_flow(insns: Vec<AssemblyInstruction>, functions: &HashMap<u64, NameValue>, globals: &HashMap<u64, NameValue>, data: &HashMap<u64, Vec<u8>>) -> (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>) {
    let mut data_flow_ir = Vec::<DataFlowIr>::new();

    let mut f = FUNCTIONS.lock().unwrap();
    *f = functions.clone();
    let mut g = GLOBALS.lock().unwrap();
    *g = globals.clone();
    let mut d = DATA.lock().unwrap();
    *d = data.clone();

    let mut symbol_table = DFISymbolRecordTable {
        symbols: HashSet::<DFISymbolRecord>::new(),
        loc_counter: Counter::new(),
        global_counter: Counter::new(),
        tmp_counter: Counter::new(),
        //param_counter: Counter::new(),
        //return_counter: Counter::new(),
    };

    let mut gr_states = Vec::<GRRecord>::new();
    for i in 0..32 {
        gr_states.push(GRRecord::new());
    }
    gr_states[0].state = true;
    gr_states[0].value = RegisterRecord::Number(Number::from(0, false, Size::Unsigned64));
    gr_states[1].state = true;
    gr_states[1].value = RegisterRecord::Symbol(DFISymbolRecord {
        address: Address::Memory(0),
        sym_type: DFISymbolType::None,
        id: 0, 
        size: Size::Unsigned64,
        value: false,
    });
    gr_states[3].state = true;
    gr_states[3].value = RegisterRecord::Symbol(DFISymbolRecord {
        address: Address::Stack(0),
        sym_type: DFISymbolType::Local,
        id: 0,
        size: Size::Unsigned64,
        value: false,
    });
    gr_states[22].state = true;
    gr_states[22].value = RegisterRecord::Symbol(DFISymbolRecord {
        address: Address::Stack(0),
        sym_type: DFISymbolType::Local,
        id: 0,
        size: Size::Unsigned64,
        value: false,
    });

    let mut gr_states_parameter = gr_states.clone();
    let mut symbol_parameter = symbol_table.symbols.clone();

    
    let ir = DataFlowIr {
        address: insns[0].address,
        opcode: DataFlowIrOpcode::Function,
        operand1: None,
        operand2: None, 
        operand3: None,
    };
    data_flow_ir.push(ir);

    for insn in insns {
        match insn.opcode {
            Opcode::ADD_D =>  add_d(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::ADD_W => add_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::SUB_W => sub_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter), 
            Opcode::ADDI_D => addi_d(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter), 
            Opcode::ADDI_W => addi_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter), 
            Opcode::ST_D => st_d(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::ST_W => st_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::LD_D => ld_d(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::LD_W => ld_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::STPTR_D => stptr_d(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::STPTR_W => stptr_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BL => bl(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::B => b(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BEQ => beq(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BNE => bne(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BGE => bge(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BLT => blt(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BLTU => bltu(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BGEU => bgeu(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BEQZ => beqz(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::BNEZ => bnez(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::OR => or(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::ANDI => andi(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::LDPTR_W => ldptr_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::LDPTR_D => ldptr_d(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::PCADDU12I => pcaddu12i(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::JIRL => jirl(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::SLLI_W => slli_w(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            Opcode::SLLI_D => slli_d(insn, &mut data_flow_ir, &mut gr_states, &mut symbol_table, &mut gr_states_parameter, &mut symbol_parameter),
            _ => {}
        }   
        
    }


    (data_flow_ir, gr_states, symbol_table.symbols)

    /*
    for ir in &data_flow_ir {
        println!("{:?}", ir);
    }
    */

    //println!("{:#?}", gr_states);
    //println!("{:#?}", symbol_table);
}


impl GRRecord {
    pub fn new() -> Self {
        GRRecord {
            value: RegisterRecord::Number(Number::new()),
            state: false,  
        }
    }
}

impl DFISymbolRecord {
    pub fn new() -> Self {
        DFISymbolRecord {
            address: Address::Memory(0),
            sym_type: DFISymbolType::Temp,
            id: 0,
            size: Size::Signed8,
            value: false,
        }
    }
}

impl Hash for DFISymbolRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.address {
            Address::GR(gr) => {
                self.address.hash(state);
                self.sym_type.hash(state);
                self.id.hash(state);
            }
            Address::FR(fr) => {
                self.address.hash(state);
                self.sym_type.hash(state);
                self.id.hash(state);
            }
            Address::Stack(s) => {
                self.address.hash(state);
                self.sym_type.hash(state);
            }
            Address::Memory(m) => {
                self.address.hash(state);
                self.sym_type.hash(state);
                self.id.hash(state);
            }
        }
    }
}

impl PartialEq for DFISymbolRecord {
    fn eq(&self, other: &Self) -> bool {
        match &self.address {
            Address::GR(gr1) => {
                if let Address::GR(gr2) = &other.address {
                    if (&self.sym_type == &DFISymbolType::Param && &other.sym_type == &DFISymbolType::Param) || (&self.sym_type == &DFISymbolType::Temp && &other.sym_type == &DFISymbolType::Temp) {
                        self.id == other.id
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Address::FR(fr1) => {
                if let Address::FR(fr2) = &other.address {
                    if (&self.sym_type == &DFISymbolType::Param && &other.sym_type == &DFISymbolType::Param) || (&self.sym_type == &DFISymbolType::Temp && &other.sym_type == &DFISymbolType::Temp) {
                        self.id == other.id
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Address::Stack(s1) => {
                if let Address::Stack(s2) = &other.address {
                    if &self.sym_type == &DFISymbolType::Local && &other.sym_type == &DFISymbolType::Local {
                        s1 == s2
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Address::Memory(m1) => {
                if let Address::Memory(m2) = &other.address {
                    if &self.sym_type == &DFISymbolType::Global && &other.sym_type == &DFISymbolType::Global {
                        m1 == m2
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
}



impl Number {
    pub fn new() -> Self {
        Number {
            value: 0,
            signed: false,
            size: Size::Unsigned64,
        }
    }

    pub fn from(value: i64, signed: bool, size: Size) -> Self {
        Number {
            value,
            signed,
            size,
        }
    }
}


pub fn analyse_parameter(function_map: HashMap<u64, (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>)>, functions: &HashMap<u64, NameValue>) -> HashMap::<u64, (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>)>{
    let mut res_map = HashMap::<u64, (Vec<DataFlowIr>, Vec<GRRecord>, HashSet<DFISymbolRecord>)>::new();

    let mut parameter_map = HashMap::<u64, usize>::new();
    for function in function_map.iter() {
        let address = function.0;
        let irs = &function.1.0;
        let gr_states = &function.1.1;
        let symbol_table = &function.1.2;

        let mut n = 0;
        for symbol in symbol_table {
            if symbol.sym_type == DFISymbolType::Param {
                n += 1;
            }
        }
        parameter_map.insert(*address, n);

        //println!("{:#?}", parameter_map);
        //panic!("print from data_flow.rs");
    } 


    for function in function_map {
        let address = function.0;
        let _irs = function.1.0;
        let gr_states = function.1.1;
        let symbol_table = function.1.2;
        let mut irs = Vec::<DataFlowIr>::new();

        let mut parameter_index_map = HashMap::<Address, usize>::new();
        let mut parameter_index_change_map = HashMap::<usize, usize>::new();

        for _ir in _irs {
            let mut ir = _ir.clone();
            let mut flag = true;

            if _ir.opcode == DataFlowIrOpcode::Call {
                if let Some(DFIOperand::Parameter(parameter)) = &_ir.operand2 {
                    if let Some(DFIOperand::Number(number)) = &_ir.operand1 {
                        if let Some(n) = parameter_map.get(&(number.value as u64)) {
                            let mut p = Vec::<RegisterRecord>::new();
                            //println!("\nprint from data_flow.rs: {:x}: {}\n\n", address, *n);
                            //println!("{:x}", _ir.address);
                            for i in 0..*n {
                                //println!("{:?}", parameter[i]);
                                p.push(parameter[i].clone());
                            }
                            ir.operand2 = Some(DFIOperand::Parameter(p));
                        } else {
                            if let Some(nv) = functions.get(&(number.value as u64)) {
                                if let NameValue::Name(name) = nv {
                                    let (n, flag) = extern_function::analyse_extern_function(name, parameter);
                                    /*
                                    if number.value == 0x120000630 {
                                        println!("{}", n);
                                        panic!("print from data_flow.rs");
                                    }
                                    */
                                    let mut p = Vec::<RegisterRecord>::new();
                                    //println!("{}\n{:#?}", n, parameter);
                                    for i in 0..n {
                                        p.push(parameter[i].clone());
                                    }
                                    ir.operand2 = Some(DFIOperand::Parameter(p));
                                    if flag {
                                        parameter_map.insert(number.value as u64, n);
                                    }
                                }
                            }
                        }
                    }
                }
            } else if _ir.opcode == DataFlowIrOpcode::Function {
                /*
                if _ir.address == 0x12000077c {
                    println!("{:#?}", symbol_table);    
                    panic!("print from data_flow.rs");
                }
                */
                let mut parameter = HashMap::<Address, DFISymbolRecord>::new();
                for sym in &symbol_table {
                    if sym.sym_type == DFISymbolType::Param {
                        parameter.insert(sym.address.clone(), sym.clone());
                    }
                }

                /*
                println!("{:x}", address);
                for p in &parameter {
                    println!("{:?}", p);
                }
                println!("");
                */

                let mut p = Vec::<RegisterRecord>::new();
                let mut parameter_n = 0;
                for i in 4..12 {
                    if let Some(sym) = parameter.get(&Address::GR(i)) {
                        if i == 4 {
                            parameter_n = sym.id;
                        }
                        p.push(RegisterRecord::Symbol(sym.clone())); 
                        parameter_index_map.insert(sym.address.clone(), sym.id);
                        parameter.remove(&Address::GR(i));
                    } else {
                        break;
                    }
                }

                //println!("{:#?}", parameter);
                if parameter.len() != 0 {
                    let sn = parameter.len();
                    let mut max = 0;
                    for p in parameter.iter() {
                        if let Address::Stack(s) = &p.0 {
                            if *s > max {
                                max = *s;
                            }
                        } 
                    }

                    for i in 0..sn {
                        if let Some(sym) = parameter.get(&Address::Stack(max)) {
                            let mut sym = sym.clone();
                            
                            p.push(RegisterRecord::Symbol(sym)); 
                            max -= 8;
                        }
                    }
                }

                ir.operand1 = Some(DFIOperand::Parameter(p));
                irs.push(ir.clone());
                flag = false;

                /*
                if let Some(n) = parameter_map.get(&address) {
                    let mut p = Vec::<RegisterRecord>::new();
                    for i in 0..*n {
                        let sym = DFISymbolRecord {
                            address: Address::GR(usize::MAX),
                            sym_type: DFISymbolType::Param,
                            id: i,
                            size: Size::Unsigned64,
                            value: true,
                        };

                        p.push(RegisterRecord::Symbol(sym));
                    }

                    ir.operand1 = Some(DFIOperand::Parameter(p));
                    irs.push(ir.clone());
                    flag = false;
                }
                */
            }   

            if flag {
                irs.push(ir);
            }

        }

        res_map.insert(address, (irs, gr_states, symbol_table));
    }
    

    res_map
}

/*
fn analyse_extern_function(name: &String, parameters: &Vec<RegisterRecord>) -> (usize, bool) {
    let path = format!("/disk/repository/loong/loongarch-decomplier/src/loongarch_decomplier/script/library_function/{}.lua", name);
    match File::open(path) {
        Ok(mut file) => {
            let data = &*DATA.lock().unwrap();
            let param = &parameters[0];
            
            let string = match param {
                RegisterRecord::Number(number) => {
                    match get_c_string_from_data(number.value as usize, data) {
                        Some(string) => string,
                        None => "".to_string(),
                    }
                }

                RegisterRecord::Symbol(symbol) => {
                    return (0, false);
                }
            };

            let mut lua_code = String::new();
            file.read_to_string(&mut lua_code).unwrap();

            let mut lua = Lua::new();
            load_lua_functions(&mut lua);
            lua.set("_arg_", string);           
            let _: () = lua.exec_string(lua_code).unwrap();
            let res: usize = lua.query("_res_").unwrap();
            (res, false)
        }
        Err(err) => {
            (0, false)
        }
    }
}
*/

impl fmt::Display for DFISymbolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DFISymbolType::Temp => write!(f, "temp"),
            DFISymbolType::None => write!(f, "none"),
            DFISymbolType::Local => write!(f, "stack"),
            DFISymbolType::Func => write!(f, "func"),
            DFISymbolType::Param => write!(f, "param"),
            DFISymbolType::Global => write!(f, "global"),
            DFISymbolType::Label => write!(f, "label"),
            DFISymbolType::Return => write!(f, "return"),
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Size::Signed8 => write!(f, "char"),
            Size::Unsigned8 => write!(f, "unsigned char"),
            Size::Signed16 => write!(f, "short"),
            Size::Unsigned16 => write!(f, "unsigned short"),
            Size::Signed32 => write!(f, "int"),
            Size::Unsigned32 => write!(f, "unsigned int"),
            Size::Signed64 => write!(f, "long"),
            Size::Unsigned64 => write!(f, "unsigned long"),
        }
    }
}

impl fmt::Display for DataFlowIr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:<08x}\t", self.address);
        
        match &self.opcode {
            DataFlowIrOpcode::Call => {
                match &self.operand3 {
                    Some(DFIOperand::Number(number)) => {}
                    Some(DFIOperand::Symbol(symbol3)) => {
                        write!(f, "{} {}@{} = ", symbol3.size, symbol3.sym_type, symbol3.id);
                    }
                    _ => {}
                }
                if let Some(DFIOperand::Number(number)) = &self.operand1 {
                    let functions = FUNCTIONS.lock().unwrap();
                    if let Some(name) = functions.get(&(number.value as u64)) {
                        match name {
                            NameValue::Name(name) => write!(f, "{}", name),
                            NameValue::Value(value) => write!(f, "func@0x{:<08x}", number.value),
                        };
                    } else {
                        write!(f, "func@0x{:<08x}", number.value);
                    }
                }

                if let Some(DFIOperand::Parameter(parameters)) = &self.operand2 {
                    write!(f, "(");
                    let mut i = 0;
                    let n = parameters.len();
                    if n != 0 {
                        while i < n - 1 {
                            match &parameters[i] {
                                RegisterRecord::Number(number) => {
                                    let data = &*DATA.lock().unwrap();   
                                    if let Some(string) = get_c_string_from_data(number.value as u64, &data) {
                                        write!(f, "\"{}\", ", string);
                                    } else {
                                        write!(f, "{}, ", number.value);
                                    }
                                }
                                RegisterRecord::Symbol(symbol) => {
                                    match &symbol.address {
                                        Address::Stack(stack) => {
                                            if symbol.value {
                                                write!(f, "*({}*)(stack@{}), ", symbol.size, stack);
                                            } else {
                                                write!(f, "stack@{}, ", stack);
                                            }  
                                        }
                                        Address::Memory(memory) => {}
                                        _ => {
                                            if !symbol.value {
                                                write!(f, "{}@{}, ", symbol.sym_type, symbol.id);
                                            } else {
                                                write!(f, "*({}*){}@{}, ", symbol.size, symbol.sym_type, symbol.id);
                                            }
                                        }
                                    } 
                                }
                            }
                            i += 1;
                        }

                        match &parameters[i] {
                            RegisterRecord::Number(number) => {
                                let data = &*DATA.lock().unwrap();   
                                if let Some(string) = get_c_string_from_data(number.value as u64, data) {
                                    write!(f, "\"{}\"", string);
                                } else {
                                    write!(f, "{}", number.value);
                                }
                            }

                            RegisterRecord::Symbol(symbol) => {
                                match &symbol.address {
                                    Address::Stack(stack) => {
                                        if symbol.value {
                                            write!(f, "*({}*)(stack@{})", symbol.size, stack);
                                        } else {
                                            write!(f, "stack@{}", stack);
                                        }  
                                    }
                                    Address::Memory(memory) => {}
                                    _ => {
                                        if !symbol.value {
                                            write!(f, "{}@{}", symbol.sym_type, symbol.id);
                                        } else {
                                            write!(f, "*({}*){}@{}", symbol.size, symbol.sym_type, symbol.id);
                                        }
                                    }
                                } 
                            }
                        }
                    }
                    write!(f, ");");
                }
            }
            DataFlowIrOpcode::Ret => {
                write!(f, "return ");
                if let Some(operand) = &self.operand1 {
                    match &operand {
                        DFIOperand::Symbol(symbol) => {
                            match &symbol.address {
                                Address::Stack(stack) => {
                                    if let DFISymbolType::Local = symbol.sym_type {
                                        if symbol.value {
                                            write!(f, "*({}*)(stack@{})", symbol.size, stack);
                                        } else {
                                            write!(f, "stack@{}", stack);
                                        }
                                    } else if let DFISymbolType::Param = symbol.sym_type {
                                        write!(f, "param@{}", symbol.id);
                                    }

                                }

                                Address::Memory(memory) => {
                                }

                                _ => {
                                    write!(f, "{}@{}", symbol.sym_type, symbol.id);
                                }
                            }
                        }
                        DFIOperand::Number(number) => {
                            write!(f, "{}", number.value);
                        }
                        _ => {}
                    }
                }
                write!(f, ";");
            }
/*
            DataFlowIrOpcode::Store => {
                match &self.operand1 {
                    Some(DFIOperand::Number(number)) => {
                        match &self.operand2 {
                            Some(DFIOperand::Number(number)) => {}
                            Some(DFIOperand::Symbol(symbol)) => {
                                match &symbol.address {
                                    Address::Stack(stack) => {
                                        match &number.size {
                                            Size::Signed8 => {}
                                            Size::Unsigned8 => {}
                                            Size::Signed16 => {}
                                            Size::Unsigned16 => {}
                                            Size::Signed32 => {write!(f, "int *(stack@{}) = {};", stack, number.value);}
                                            Size::Unsigned32 => {}
                                            Size::Signed64 => {}
                                            Size::Unsigned64 => {}

                                        }
                                    }

                                    Address::Memory(memory) => {
                                        match &number.size {
                                            Size::Signed8 => {}
                                            Size::Unsigned8 => {}
                                            Size::Signed16 => {}
                                            Size::Unsigned16 => {}
                                            Size::Signed32 => {write!(f, "*(int*)*((unsigned long*)global@0x{:x}) = {};", memory, number.value);}
                                            Size::Unsigned32 => {}
                                            Size::Signed64 => {}
                                            Size::Unsigned64 => {}
                                        }
                                    }

                                    _ => {}
                                }
                            }
                            None => {}
                        }
                    }

                    Some(DFIOperand::Symbol(symbol)) => {}

                    None => {}
                }
            }
            // DataFlowIrOpcode::Store
*/
            DataFlowIrOpcode::Jmp => {
                match &self.operand1 {
                    Some(DFIOperand::Number(number)) => {
                        write!(f, "goto label@0x{:x};", number.value);
                    }

                    _ => {}
                }
            }

            DataFlowIrOpcode::Add => {

                let operand1 = match &self.operand1 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand2 = match &self.operand2 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand3 = match &self.operand3 {
                    Some(o) => o,
                    None => panic!("error"),
                };

             
                let size = match &operand1 {
                    DFIOperand::Symbol(symbol) => {
                        match &symbol.sym_type {
                            DFISymbolType::Temp => {
                                write!(f, "{} {}@{} = ", symbol.size, symbol.sym_type, symbol.id);
                                /*
                                match symbol.size {
                                    Size::Signed8 => {}
                                    Size::Unsigned8 => {}
                                    Size::Signed16 => {}
                                    Size::Unsigned16 => {}
                                    Size::Signed32 => {write!(f, "int temp@{} = ", symbol.id); size = Size::Signed32;}
                                    Size::Unsigned32 => {}
                                    Size::Signed64 => {}
                                    Size::Unsigned64 => {}
                                }
                                */
                            }
                            DFISymbolType::Param => {
                                write!(f, "{} {}@{} = ", symbol.size, symbol.sym_type, symbol.id);
                            }

                            _ => {
                                
                            }
                        }
                        symbol.size.clone()
                    }
                    DFIOperand::Number(number) => {
                        number.size.clone()
                    }
                     _ =>{panic!("error");}
                };

                match &operand2 {
                     DFIOperand::Symbol(symbol) => {
                         match &symbol.address {
                            Address::Stack(stack) => {
                                match &symbol.sym_type {
                                    DFISymbolType::Local => write!(f, "*(*{})({}@{}) + ", size, symbol.sym_type, stack),
                                    _ => write!(f, "{}@{} + ", symbol.sym_type, symbol.id),
                                };
                            }
                            Address::Memory(memory) => {
                                write!(f, "*(*{})({}@0x{:x}) + ", size, symbol.sym_type, memory);
                            }
                            _ => {
                                write!(f, "{}@{} + ", symbol.sym_type, symbol.id);
                            }
                         }
                         /*
                         match &symbol.address {
                            Address::Stack(stack) => {
                                match &symbol.sym_type {
                                    DFISymbolType::Local =>  write!(f, "*({}*)({}@{}) + ", size, symbol.sym_type, stack),
                                    _ => write!(f, "{}@{} + ", symbol.sym_type, symbol.id),
                                };
                            }
                            Address::Memory(memory) => {}
                            _ => {
                                write!(f, "*({}*)({}@{}) + ", size, symbol.sym_type, symbol.id);
                            }
                         }
                         */
                    }
                    DFIOperand::Number(number) => {
                        if number.signed {
                            write!(f, "{} + ", number.value);
                        } else {
                            write!(f, "{} + ", number.value as usize);
                        }
                    }
                    _ => {}
                };

                match &operand3 {
                    DFIOperand::Number(number) => {
                        write!(f, "{};", number.value);
                    }
                    DFIOperand::Symbol(symbol) => {
                         match &symbol.address {
                            Address::Stack(stack) => {
                                match &symbol.sym_type {
                                    DFISymbolType::Local => write!(f, "*(*{})({}@{});", size, symbol.sym_type, stack),
                                    _ => write!(f, "{}@{};", symbol.sym_type, symbol.id),
                                };
                            }
                            Address::Memory(memory) => {
                                write!(f, "*(*{})({}@0x{:x});", size, symbol.sym_type, memory);
                            }
                            _ => {
                                write!(f, "{}@{};", symbol.sym_type, symbol.id);
                            }
                         }
                        /*
                        match &symbol.sym_type {
                            _ => {
                                match &symbol.address {
                                    Address::Stack(stack) => {
                                        match size {
                                            Size::Signed8 => {}
                                            Size::Unsigned8 => {}
                                            Size::Signed16 => {}
                                            Size::Unsigned16 => {}
                                            Size::Signed32 => {write!(f, "*(int*)(stack@{});", stack);}
                                            Size::Unsigned32 => {}
                                            Size::Signed64 => {}
                                            Size::Unsigned64 => {}
                                        }
                                    }
                                    Address::Memory(memory) => {}
                                    _ => {}
                                }
                            }
                        }
                        */
                    }
                    _ => {}
                };


            }
            // DataFlowIrOpcode::Add

            DataFlowIrOpcode::Sub => {

                let operand1 = match &self.operand1 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand2 = match &self.operand2 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand3 = match &self.operand3 {
                    Some(o) => o,
                    None => panic!("error"),
                };

             
                let size = match &operand1 {
                    DFIOperand::Symbol(symbol) => {
                        match &symbol.sym_type {
                            DFISymbolType::Temp => {
                                write!(f, "{} {}@{} = ", symbol.size, symbol.sym_type, symbol.id);
                                /*
                                match symbol.size {
                                    Size::Signed8 => {}
                                    Size::Unsigned8 => {}
                                    Size::Signed16 => {}
                                    Size::Unsigned16 => {}
                                    Size::Signed32 => {write!(f, "int temp@{} = ", symbol.id); size = Size::Signed32;}
                                    Size::Unsigned32 => {}
                                    Size::Signed64 => {}
                                    Size::Unsigned64 => {}
                                }
                                */
                            }
                            DFISymbolType::Param => {
                                write!(f, "{} {}@{} = ", symbol.size, symbol.sym_type, symbol.id);
                            }

                            _ => {
                                
                            }
                        }
                        symbol.size.clone()
                    }
                    DFIOperand::Number(number) => {
                        number.size.clone()
                    }

                    _ => {panic!("error")}
                };

                match &operand2 {
                     DFIOperand::Symbol(symbol) => {
                         match &symbol.address {
                            Address::Stack(stack) => {
                                write!(f, "*({}*)({}@{}) - ", size, symbol.sym_type, stack);
                                /*
                                match size {
                                    Size::Signed8 => {}
                                    Size::Unsigned8 => {}
                                    Size::Signed16 => {}
                                    Size::Unsigned16 => {}
                                    Size::Signed32 => {write!(f, "*(int*)(stack@{}) + ", stack);}
                                    Size::Unsigned32 => {}
                                    Size::Signed64 => {}
                                    Size::Unsigned64 => {}
                                }
                                */
                            }
                            Address::Memory(memory) => {}
                            _ => {
                                
                            }
                         }
                         /*
                        match symbol.sym_type {
                            DFISymbolType::Temp => {
                            }

                            _ => {
                                match symbol.address {
                                    Address::Stack(stack) => {
                                        match size {
                                            Size::Signed8 => {}
                                            Size::Unsigned8 => {}
                                            Size::Signed16 => {}
                                            Size::Unsigned16 => {}
                                            Size::Signed32 => {write!(f, "*(int*)(stack@{}) + ", stack);}
                                            Size::Unsigned32 => {}
                                            Size::Signed64 => {}
                                            Size::Unsigned64 => {}
                                        }
                                    }
                                    Address::Memory(memory) => {}
                                    _ => {}
                                }
                            }
                        }
                        */
                    }
                    DFIOperand::Number(number) => {}
                    _ => {}
                }

                match &operand3 {
                    DFIOperand::Number(number) => {
                        write!(f, "{};", number.value);
                    }
                    DFIOperand::Symbol(symbol) => {
                        match &symbol.sym_type {
                            DFISymbolType::Temp => {
                                write!(f, "{}@{};", symbol.sym_type, symbol.id); 
                            }

                            _ => {
                                match &symbol.address {
                                    Address::Stack(stack) => {
                                        match size {
                                            Size::Signed8 => {}
                                            Size::Unsigned8 => {}
                                            Size::Signed16 => {}
                                            Size::Unsigned16 => {}
                                            Size::Signed32 => {write!(f, "*(int*)(stack@{});", stack);}
                                            Size::Unsigned32 => {}
                                            Size::Signed64 => {}
                                            Size::Unsigned64 => {}
                                        }
                                    }
                                    Address::Memory(memory) => {}
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }

            }
            // DataFlowIrOpcode::Sub



            DataFlowIrOpcode::Store => {

                let operand1 = match &self.operand1 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand2 = match &self.operand2 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                match &operand2 {
                    DFIOperand::Number(number) => {
                        write!(f, "{}", number.value);
                    }

                    DFIOperand::Symbol(symbol) => {
                        match &symbol.address {
                            Address::Stack(stack) => {
                                if let DFISymbolType::Local = &symbol.sym_type {
                                    if symbol.value {
                                        write!(f, "*({}*)(stack@{}) = ", symbol.size, stack);
                                    } else {
                                        write!(f, "stack@{} = ", stack);
                                    }
                                } else if let DFISymbolType::Param = &symbol.sym_type {
                                }

                            }
                            Address::Memory(memory) => {
                                if symbol.value {
                                    write!(f, "*({}*)(global@0x{:x}) = ", symbol.size, memory);
                                } else {
                                    write!(f, "global@0x{:x} = ", memory);                           
                                }
                            }
                            _ => {
                                if !symbol.value {
                                    write!(f, "{}@{} = ", symbol.sym_type, symbol.id);
                                } else {
                                    write!(f, "*({}*)({}@{}) = ", symbol.size, symbol.sym_type, symbol.id);
                                }
                            }
                        }
                    }

                    _ => {}
                }


                match &operand1 {
                    DFIOperand::Number(number) => {
                        write!(f, "{};", number.value);
                    }

                    DFIOperand::Symbol(symbol) => {
                        match &symbol.address {
                            Address::Stack(stack) => {
                                if let DFISymbolType::Local = &symbol.sym_type {
                                    if symbol.value {
                                        write!(f, "*({}*)(stack@{});", symbol.size, stack);
                                    } else {
                                        write!(f, "stack@{};", stack);                           
                                    }
                                } else if let DFISymbolType::Param = &symbol.sym_type {
                                    write!(f, "param@{};", symbol.id);
                                }
                            }
                            Address::Memory(memory) => {
                                if symbol.value {
                                    write!(f, "*({}*)(global@0x{:x});", symbol.size, memory);
                                } else {
                                    write!(f, "global@0x{:x};", memory);
                                }
                            }
                            _ => {
                                write!(f, "{}@{};", symbol.sym_type, symbol.id);
                            }
                        }
                    }

                    _ => {}
                }

                /*
                match &operand2 {
                    DFIOperand::Number(number) => {}
                    DFIOperand::Symbol(symbol2) => {
                        match &symbol2.address {
                            Address::Stack(stack2) => {
                                let size = get_size(operand1);
                                match operand1 {
                                    DFIOperand::Number(number) => {
                                        write!(f, "{} *({}*)(stack@{}) = {};", symbol2.size, symbol2.size, stack2, number.value);}
                                    }
                                    DFIOperand::Symbol(symbol1) => {
                                        match &symbol1.address {
                                            Address::Stack(stack1) => {
                                                if symbol1.value {

                                                } else {
                                                    write!(f, "*({}*)(stack@{}) = stack@{};", symbol1.size, stack1, stack2);
                                                }
                                            }
                                            Address::Memory(memory2) => {}
                                            _ => {
                                                write!(f, "*({}*)(stack@{}) = {}@{};", size, stack2, symbol1.sym_type, symbol1.id);
                                                /*
                                                match size {
                                                    Size::Signed8 => {}
                                                    Size::Unsigned8 => {}
                                                    Size::Signed16 => {}
                                                    Size::Unsigned16 => {}
                                                    Size::Signed32 => {write!(f, "*(int*)(stack@{}) = temp@{};", stack2, symbol1.id);}
                                                    Size::Unsigned32 => {}
                                                    Size::Signed64 => {}
                                                    Size::Unsigned64 => {}
                                                }
                                                */
                                            }
                                        }
                                    }
                                }
                            }

                            Address::Memory(memory) => {
                                if symbol2.value {
                                    match &operand1 {
                                        DFIOperand::Number(number) => {
                                            match &number.size{
                                                    Size::Signed8 => {}
                                                    Size::Unsigned8 => {}
                                                    Size::Signed16 => {}
                                                    Size::Unsigned16 => {}
                                                    Size::Signed32 => {write!(f, "*(int*)(*(unsigned long*)(global@0x{:x})) = {};", memory, number.value);}
                                                    Size::Unsigned32 => {}
                                                    Size::Signed64 => {}
                                                    Size::Unsigned64 => {}
                                            }
                                        }

                                        DFIOperand::Symbol(symbol) => {

                                        }
                                    }
                                }
                            }

                            _ => {}
                        }
                    }
                }
            */
            }
            // DataFlowIrOpcode::Store

            DataFlowIrOpcode::Jcc(jcc) => {
                write!(f, "if(");
                match &self.operand1 {
                    Some(DFIOperand::Number(number)) => {
                        write!(f, "{}", number.value);
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        match &symbol.address {
                            Address::Stack(stack) => {
                                write!(f, "*({}*)(stack@{})", symbol.size, stack);
                            }
                            Address::Memory(memory) => {}
                            _ => {
                                write!(f, "{}@{}", symbol.sym_type, symbol.id);
                            }
                        }
                    }
                    _ => {}
                }

                match jcc {
                    Relation::L => write!(f, " < "),
                    Relation::G => write!(f, " > "),
                    Relation::EQ => write!(f, " == "),
                    Relation::NE => write!(f, " != "),
                    Relation::LE => write!(f, " <= "), 
                    Relation::GE => write!(f, " >= "),
                };

                match &self.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        write!(f, "{}", number.value);
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        match &symbol.address {
                            Address::Stack(stack) => {
                                if symbol.value {
                                    match &symbol.size {
                                        Size::Signed8 => {}
                                        Size::Unsigned8 => {}
                                        Size::Signed16 => {}
                                        Size::Unsigned16 => {}
                                        Size::Signed32 => {write!(f, "*(int*)(stack@{})", stack);}
                                        Size::Unsigned32 => {}
                                        Size::Signed64 => {}
                                        Size::Unsigned64 => {}
                                    }
                                } else {}
                            }

                            Address::Memory(memory) => {}
                            _ => {
                                write!(f, "{}@{}", symbol.sym_type, symbol.id);
                            }
                        }
                    }
                    _ => {}
                }

                write!(f, ") goto ");
                
                if let Some(DFIOperand::Number(number)) = &self.operand3 {
                    write!(f, "label@0x{:x}", number.value);
                }

            } 
            // DataFlowIrOpcode::JCC
            
            DataFlowIrOpcode::Mul => {
                let operand1 = match &self.operand1 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand2 = match &self.operand2 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand3 = match &self.operand3 {
                    Some(o) => o,
                    None => panic!("error"),
                };


                let size = match &operand1 {
                    DFIOperand::Number(number) => {number.size.clone()}
                    DFIOperand::Symbol(symbol1) => {
                        match &symbol1.address {
                            Address::Stack(stack) => {}
                            Address::Memory(memory) => {}
                            _ => {
                                write!(f, "{} {}@{} = ", symbol1.size, symbol1.sym_type, symbol1.id); 
                            }
                        }

                        symbol1.size.clone()
                    }
                    _ => {panic!("error")}
                };

                match &operand2 {
                    DFIOperand::Number(number) => {}
                    DFIOperand::Symbol(symbol2) => {
                        match &symbol2.address {
                            Address::Stack(stack) => {
                                if symbol2.value {
                                    write!(f, "*({}*)(stack@{}) * ", symbol2.size, stack);
                                }
                            }
                            Address::Memory(memory) => {}
                            _ => {
                                write!(f, "{}@{} * ", symbol2.sym_type, symbol2.id); 
                            }
                        }

                    }
                    _ => {}
                }

                match &operand3 {
                    DFIOperand::Number(number) => {
                        write!(f, "{};", number.value);
                    }
                    DFIOperand::Symbol(symbol3) => {
                        match &symbol3.address {
                            Address::Stack(stack) => {}
                            Address::Memory(memory) => {}
                            _ => {
                                write!(f, "{}@{} * ", symbol3.sym_type, symbol3.id); 
                            }
                        }

                    }
                    _ => {}
                }

            }
            // DataFlowIrOpcode::MUL

            DataFlowIrOpcode::Load => {
                let operand1 = match &self.operand1 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let operand2 = match &self.operand2 {
                    Some(o) => o,
                    None => panic!("error"),
                };

                let size = match &operand1 {
                    DFIOperand::Number(number) => {number.size.clone()}
                    DFIOperand::Symbol(symbol) => {
                        match &symbol.address {
                            Address::Stack(stack) => {}
                            Address::Memory(memory) => {}
                            _ => {
                                write!(f, "{} {}@{} = ", symbol.size, symbol.sym_type, symbol.id);
                            }
                        }
                        symbol.size.clone()
                    }
                    _ => panic!("error"),
                };

                match &operand2 {
                    DFIOperand::Number(number) => {}
                    DFIOperand::Symbol(symbol) => {
                        match &symbol.address {
                            Address::Stack(stack) => {
                                if symbol.value {
                                    write!(f, "*({}*){}@{};", size, symbol.sym_type, stack);
                                } else {

                                }
                            }
                            Address::Memory(memory) => {}
                            _ => {
                                if symbol.value {
                                    write!(f, "*({}*){}@{};", size, symbol.sym_type, symbol.id);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            // DataFlowIrOpcode::Load


            DataFlowIrOpcode::Function => {
                write!(f, "function begin(",);
                if let Some(DFIOperand::Parameter(parameter)) = &self.operand1 {
                    if parameter.len() != 0 {
                        for i in 0..parameter.len() - 1 {
                            let p = &parameter[i];
                            match &p {
                                RegisterRecord::Number(number) => {}
                                RegisterRecord::Symbol(symbol) => {
                                    write!(f, "param@{}, ", symbol.id);
                                }
                            }
                        }
                    
                        let p = &parameter[parameter.len() - 1];
                        match &p {
                            RegisterRecord::Number(number) => {}
                            RegisterRecord::Symbol(symbol) => {
                                write!(f, "param@{})", symbol.id);
                            }
                        }
                    } else {
                        write!(f, ")");
                    }
                }
            }
            // DataFlowIrOpcode::Function

            _ => {}
        }

        write!(f, "")
    }
}

fn get_size(operand: &DFIOperand) -> Size {
    match operand {
        DFIOperand::Symbol(symbol) => symbol.size.clone(),
        DFIOperand::Number(number) => number.size.clone(),
        _ => panic!("error"),
    }
}


impl DataFlowIr {
    pub fn nop(address: u64) -> Self {
        Self {
            address,
            opcode: DataFlowIrOpcode::Nop,
            operand1: None,
            operand2: None,
            operand3: None,
        }
    }
}

mod add_w;
mod add_d;
mod sub_w;
mod addi_d;
mod addi_w;
mod st_d;
mod st_w;
mod stptr_d;
mod stptr_w;
mod ld_d;
mod ld_w;
mod bl;
mod b;
mod beq;
mod bge;
mod bne;
mod blt;
mod bltu;
mod bgeu;
mod beqz;
mod bnez;
mod or;
mod andi;
mod ldptr_d;
mod ldptr_w;
mod pcaddu12i;
mod jirl;
mod slli_w;
mod slli_d;

use add_w::*;
use add_d::*;
use sub_w::*;
use addi_d::*;
use addi_w::*;
use st_d::*;
use st_w::*;
use ld_d::*;
use ld_w::*;
use stptr_d::*;
use stptr_w::*;
use bl::*;
use b::*;
use beq::*;
use bne::*;
use bge::*;
use blt::*;
use bgeu::*;
use bltu::*;
use beqz::*;
use bnez::*;
use or::*;
use andi::*;
use ldptr_w::*;
use ldptr_d::*;
use pcaddu12i::*;
use jirl::*;
use slli_w::*;
use slli_d::*;
