use std::path::Display;

//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

#[derive(Debug, Clone)]
pub struct Block {
    pub address: u64,
    pub irs: Vec<DataFlowIr>,
    pub next: Option<u64>, 
    pub condiction: Option<Condiction>,
    pub true_next: Option<u64>,
    pub false_next: Option<u64>,
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct Condiction {
    pub relation: Relation,
    pub operand1: DFIOperand,
    pub operand2: DFIOperand,
}

impl std::ops::Not for Condiction {
    type Output = Self;
    fn not(self) -> Self::Output {
        Condiction {
            relation: !self.relation,
            operand1: self.operand2,
            operand2: self.operand1,
        }
    }
}

impl PartialEq for Condiction {
    fn eq(&self, other: &Self) -> bool {
        if (self.relation == other.relation) && (self.operand1 == other.operand1) && (self.operand2 == self.operand2) {
            true
        } else {
            let not_self = !self.clone();
            (not_self.relation == other.relation) && (not_self.operand1 == other.operand1) && (not_self.operand2 == other.operand2)
        }
    }
}

pub fn get_blocks(irs: &Vec<DataFlowIr>) -> HashMap<usize, Block> {
    let mut blocks = Vec::<Block>::new();
    let mut block = Block::new();
    let mut start = true;
    let function_start_address = irs.first().unwrap().address;

    let mut start_address = HashSet::<u64>::new();
    let mut end_address = HashSet::<u64>::new();

    let min = irs.first().unwrap().address;
    let max = irs.last().unwrap().address;
    for ir in irs {
        match &ir.opcode {
            DataFlowIrOpcode::Function => {
                //block.address = ir.address;
                start_address.insert(ir.address);
                start = false;
            } 

            DataFlowIrOpcode::Jmp => {
                if let Some(DFIOperand::Number(number)) = &ir.operand1 {
                    if number.value as u64 <= max && number.value as u64 >= min {
                        start_address.insert(number.value as u64);
                        start_address.insert(ir.address + 4);
                        end_address.insert(number.value as u64 - 4);
                        end_address.insert(ir.address);
                    }
                    //block.next = Some(number.value as usize);
                } else {
                    panic!("jmp operand error");
                }
                //blocks.push(block.clone());
                start = true;
                //block = Block::new();
            }

            DataFlowIrOpcode::Jcc(relation) => {
                if let Some(DFIOperand::Number(number)) = &ir.operand3 {
                    if number.value as u64 <= max && number.value as u64 >= min {
                        start_address.insert(number.value as u64);
                        start_address.insert(ir.address + 4);
                        end_address.insert(number.value as u64 - 4);
                        end_address.insert(ir.address);
                    }
                    //block.false_next = Some(number.value as usize);
                } else {
                    panic!("jcc operand error");
                }

                if let Some(operand1) = &ir.operand1 {
                    if let Some(operand2) = &ir.operand2 {
                        /*
                        let condiction = Condiction {
                            relation: relation.clone(),
                            operand1: operand1.clone(),
                            operand2: operand2.clone(),
                        };

                        block.condiction = Some(condiction);
                        */
                    } else {
                        panic!("operand2 error");
                    }
                } else {
                    panic!("operand1 error");
                }

                start = true;
                //blocks.push(block.clone());
                //block = Block::new();
            }

            _ => {
                if start {
                    start_address.insert(ir.address);
                    //block.address = ir.address;
                    //block.irs.push(ir.clone());
                    start = false;
                } else {
                    //block.irs.push(ir.clone());
                }
            }
        }            
    }
    end_address.insert(max);

    /*
    println!("start_address:");
    let mut s: Vec<&usize> = _start_address.iter().collect();
    s.sort();
    for s in s {
        println!("{:x}", s);
    }

    println!("end_address:");
    let mut s: Vec<&usize> = _end_address.iter().collect();
    s.sort();
    for s in s {
        println!("{:x}", s);
    }
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    */


    //blocks.push(block);

    /*
    let mut start_address = HashSet::<usize>::new();
    let mut end_address = HashSet::<usize>::new();
    let max = irs.last().unwrap().address;
    let mut modify_map = HashMap::<usize, usize>::new();

    /*
    for s in _start_address {
        let mut start = s;
        loop {
            if irs.iter().any(|ir| ir.address == start) {
                modify_map.insert(s, start);
                start_address.insert(start); 
                break;
            } else {
                if max < s {
                    panic!("start address not found");
                }
                start += 4;
            }
        }
    }
    */

    end_address.insert(max);
    modify_map.insert(max, max);
    for e in _end_address {
        let mut end = e;
        loop {
            if irs.iter().any(|ir| ir.address == end) {
                modify_map.insert(e, end);
                end_address.insert(end); 
                break;
            } else {
                if max < end {
                    panic!("start address not found");
                }
                end += 4;
            }
        }
    }
    */
    let mut start_address: Vec<u64> = start_address.into_iter().collect();
    let mut end_address: Vec<u64> = end_address.into_iter().collect();
    start_address.sort();
    end_address.sort();

    /*
    println!("print from block.rs");
    for i in 0..start_address.len() {
        println!("block {}", i);
        println!("start: {:x}", start_address[i]);
        println!("end: {:x}", end_address[i]);
        println!("--------------------");
    }
    */

    start = true;
    /*
    println!("\n--------------------------------------------\nstart:");
    for s in &start_address {
        println!("{:x}", s);
    }
    println!("\nend:");
    for e in &end_address {
        println!("{:x}", e);
    }
    */
    /*
    println!("modify_map:");
    for m in modify_map.iter() {
        println!("{:x}: {:x}", m.0, m.1);
    }
    */

    let mut si = 0;
    let mut ei = 0;
    block = Block::new();
    for ir in irs {
        let mut flag = false;
        if start_address.len() > si && ir.address == start_address[si] {
            block.irs.clear();
            block.address = ir.address;
            match &ir.opcode {
                DataFlowIrOpcode::Function => {
                    block.irs.push(ir.clone());
                    flag = true;
                }

                DataFlowIrOpcode::Jmp => {
                    //if let Some(DFIOperand::Number(number)) = &ir.operand1 {
                    //    block.next = Some(number.value as usize);
                    //}
                }

                DataFlowIrOpcode::Jcc(relation) => {
                    
                }
                
                _ => {
                    block.irs.push(ir.clone());
                    flag = true;
                }
            } 

            si += 1;
        }
        
        if end_address.len() > ei && ir.address == end_address[ei] {
            match &ir.opcode {
                DataFlowIrOpcode::Jmp => {
                    if let Some(DFIOperand::Number(number)) = &ir.operand1 {
                        block.next = Some(number.value as u64);
                    } else {
                        panic!("jmp operand error");
                    } 
                    flag = true;
                }

                DataFlowIrOpcode::Jcc(relation) => {
                    if let Some(DFIOperand::Number(number)) = &ir.operand3 {
                        block.true_next = Some(number.value as u64);
                        //block.false_next = Some(modify_map[&(number.value as usize + 4)]);
                        if si < start_address.len() {
                            block.false_next = Some(start_address[si]);
                        }
                    } else {
                        panic!("jcc operand error");
                    }

                    if let Some(operand1) = &ir.operand1 {
                        if let Some(operand2) = &ir.operand2 {
                            let condiction = Condiction {
                                relation: relation.clone(),
                                operand1: operand1.clone(),
                                operand2: operand2.clone(),
                            };

                            block.condiction = Some(condiction);
                        } else {
                            panic!("operand2 error");
                        }
                    } else {
                        panic!("operand1 error");
                    }

                }

                _ => {
                    if !flag {
                        block.irs.push(ir.clone());
                    }
                    if si < start_address.len() {
                        block.next = Some(start_address[si]);
                    }
                }
            }
            /*
            if block.irs.len() == 1 {
                println!("--------------------------------------------");
                println!("{}", block);
                println!("{:x}", end_address[ei]);
                panic!("print from block.rs");
            }
            */
            blocks.push(block.clone());
            block = Block::new();
            ei += 1;
        }

        if !flag {
            block.irs.push(ir.clone());
        }
    }

    /*
    println!("-------------------------------\n{}", blocks.len());
    for block in blocks.iter() {
        println!("block start:");
        println!("{}", block);
    } 
    */
    //blocks
    
    let mut blocks_map = HashMap::<u64, Block>::new();
    let mut modify_map = HashMap::<u64, u64>::new();
    let mut i = 1;
    for s in &start_address {
        if *s == function_start_address {
            modify_map.insert(*s, 0);
        } else {
            modify_map.insert(*s, i);
            i += 1;
        }
    }

    //println!("{:#?}", modify_map);

    


    for block in blocks.iter_mut() {
        //println!("{}", block);
        //println!("{}", block.address);
        block.address = modify_map[&block.address];
        if let Some(next) = &block.next {
            block.next = Some(modify_map[next]);
        }
        if let Some(true_next) = &block.true_next {
            block.true_next = Some(modify_map[true_next]);
        }
        if let Some(false_next) = &block.false_next {
            block.false_next = Some(modify_map[false_next]);
        }
    }

    for block in blocks {
        /*
        println!("id: {}", block.address);
        if let Some(next) = block.next {
            println!("next: {}", next);
        }
        if let Some(true_next) = block.true_next {
            println!("true next: {}", true_next);
        }

        if let Some(false_next) = block.false_next {
            println!("false next: {}", false_next);
        }
        println!("\n");
        */
        blocks_map.insert(block.address, block);
    }

    let mut blocks = HashMap::<usize, Block>::new();
    for (id, block) in blocks_map {
        blocks.insert(id as usize, block);
    }

    blocks
}

impl Block {
    fn new() -> Self {
        Self {
            address: 0,
            irs: Vec::new(),
            next: None,
            condiction: None,
            true_next: None,
            false_next: None,
        }
    }
}


impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address: {:x}\n", self.address);
        for ir in &self.irs {
            write!(f, "{}\n", ir);
        }

        if let Some(next) = &self.next {
            write!(f, "next {:x}\n", next);
        }

        if let Some(condiction) = &self.condiction {
            write!(f, "condiction: {:?} {:?} {:?}\n", condiction.operand1, condiction.relation, condiction.operand2);
        }

        if let Some(true_next) = &self.true_next {
            write!(f, "true_next: {:x}\n", true_next);
        }

        if let Some(false_next) = &self.false_next {
            write!(f, "false_next: {:x}\n", false_next);
        }
        write!(f, "")
    }
}
