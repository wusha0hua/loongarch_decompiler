//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;
use std::sync::atomic::*;

pub static SHOW_DATA_FLOW_IR_BY_DEBUG: AtomicBool = AtomicBool::new(true);
pub static SHOW_DATA_FLOW_IR: AtomicBool = AtomicBool::new(true);
pub static SHOW_BLOCK_WITH_IR: AtomicBool = AtomicBool::new(true);
pub static SHOW_CONTROL_FLOW_GRAPH_INFORMATION: AtomicBool = AtomicBool::new(true);
pub static SHOW_CONTROL_FLOW_TREE_BY_DEBUG: AtomicBool = AtomicBool::new(true);
pub static SHOW_CONTROL_FLOW_TREE_AS_GRAPH: AtomicBool = AtomicBool::new(true);
pub static SHOW_CONTROL_FLOW_IN_DATA_FLOW_IR: AtomicBool = AtomicBool::new(true);
pub static SHOW_ABSTRACT_SYNTAX_TREE_BY_DEBUG: AtomicBool = AtomicBool::new(true);
pub static SHOW_ABSTRACT_SYNTAX_TREE: AtomicBool = AtomicBool::new(true);

pub static OPTIMIZATION: AtomicBool = AtomicBool::new(true);

pub static MAX_OPERAND_IN_ASSIGN: AtomicU8 = AtomicU8::new(4);


