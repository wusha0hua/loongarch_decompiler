mod graph;

pub use graph::*;

//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

#[derive(Debug, Clone)]
pub struct ControlFlowTree {
    pub id: usize,
    pub is_sink: bool,
    pub node_type: NodeType,
    pub access_condiction: Vec<Vec<isize>>,
    pub condiction: Option<isize>,
    pub true_next: Option<Box<ControlFlowTree>>,
    pub false_next: Option<Box<ControlFlowTree>>,
    pub loop_region: Option<Box<ControlFlowTree>>,
    pub next: Vec<Box<ControlFlowTree>>,
}

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub nodes: HashMap<usize, CFGNode>,
    pub edges: HashMap<usize, CFGEdge>,
    pub condiction: HashMap<Condiction, isize>,
    pub topo_index: Vec<usize>,
    pub _topo_index: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct CFGNode {
    pub id: usize,
    pub index: usize,
    pub node_type: NodeType,
    pub irs: Vec<DataFlowIr>,
}

#[derive(Debug, Clone)]
pub struct CFGEdge {
    pub id: usize,
    pub from: usize,
    pub to: u64,
    pub edge_type: EdgeType,
    pub condiction: Option<Condiction>,
    pub _true: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Branch {
    True,
    False,
    Next,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    None,
    Back,
    Break,
    Loop,
    Set,
    If,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeType {
    None,
    Break,
    LoopEnter,
    LoopExit,
    Back,
    If,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            condiction: HashMap::new(),
            topo_index: Vec::new(),
            _topo_index: Vec::new(),
        }
    }

    pub fn new_node_id(&self) -> usize {
        let mut id = 0;
        while self.nodes.iter().any(|(i, _)| *i == id) {
            id += 1;
        }

        id
    }

    pub fn info(&self) {
        println!("vertexs({}): ", self.nodes.len());
        for v in self.nodes.iter() {
            let v = v.1;
            println!("id: {} \t type: {:?}", v.id, v.node_type);
        }
        println!("edges: ");
        for edge in self.edges.iter() {
            let edge = edge.1;
            if let Some(cond) = edge.condiction.as_ref() {
                println!("id: {}\t {} -> {} \ttype: {:?} \tcondiction: {:?} \tis_true: {:?}", edge.id, edge.from, edge.to, edge.edge_type, self.condiction.get(cond), edge._true);
            } else {
                println!("id: {}\t {} -> {} \ttype: {:?} \tcondiction: {:?} \tis_true: {:?}", edge.id, edge.from, edge.to, edge.edge_type, edge.condiction, edge._true);
            }
        }
    }

    pub fn build_control_flow_graph(blocks: HashMap<usize, Block>) -> ControlFlowGraph {
        let mut cfg = ControlFlowGraph::new(); 
        let mut edge_counter = Counter::new();
        let mut condiction_id = 1;
        for block in blocks {
            let id = block.0;
            let block = block.1;
            
            let node = CFGNode {
                id,
                index: 0,
                node_type: NodeType::None,
                irs: block.irs,
            };
            cfg.nodes.insert(id, node);
    
            if let Some(next) = block.next {
                let edge = CFGEdge {
                    id: edge_counter.get(),
                    from: id,
                    to: next,
                    edge_type: EdgeType::None,
                    condiction: block.condiction.clone(),
                    _true: None,
                };
                cfg.edges.insert(edge.id, edge);
            }
    
            if let Some(true_next) = block.true_next {
                let edge = CFGEdge {
                    id: edge_counter.get(),
                    from: id,
                    to: true_next,
                    edge_type: EdgeType::None,
                    condiction: block.condiction.clone(),
                    _true: Some(true),
                };
                cfg.edges.insert(edge.id, edge);
                if let Some(condiction) = &block.condiction {
                    if let None = cfg.condiction.get(condiction) {
                        cfg.condiction.insert(condiction.clone(), condiction_id);
                        condiction_id += 1;
                    }
                }
            }
    
            if let Some(false_next) = block.false_next {
                let edge = CFGEdge {
                    id: edge_counter.get(),
                    from: id,
                    to: false_next,
                    edge_type: EdgeType::None,
                    condiction: block.condiction.clone(),
                    _true: Some(false),
                };
                cfg.edges.insert(edge.id, edge);
                if let Some(condiction) = &block.condiction {
                    if let None = cfg.condiction.get(condiction) {
                        cfg.condiction.insert(condiction.clone(), condiction_id);
                        condiction_id += 1;
                    }
                }
            }
    
        }   
    
        //println!("{:#?}", cfg.condiction);
        cfg
    }

    pub fn restruct_from_cycle(&mut self, structure_counter: &mut Counter, temps: &mut HashMap<usize, DFISymbolRecord>) -> HashMap<usize, Condiction> {
        let mut paths = get_cycle_paths(&self); 
        let mut entrys_map = HashMap::<usize, Vec<usize>>::new(); 
        let mut exits_map = HashMap::<usize, Vec<usize>>::new();
        let mut first_loop = Vec::<usize>::new();
        //paths.first().unwrap().clone();
        for _loop in paths.iter() {
            (entrys_map, exits_map) = get_entry_exit(&self, _loop);
            //println!("loop: {:?}", _loop);
            //println!("entrys_map: {:?}", entrys_map);
            //println!("exits_map: {:?}", exits_map);
            if exits_map.len() != 0 && entrys_map.len() != 0 {
                first_loop = _loop.clone();
                break;
            }
        }
        if first_loop.len() == 0 {
            panic!("error");
        }
        if paths.len() != 0 {
            let mut delete_node = HashSet::<CFGNode>::new();
            let mut delete_edge = HashSet::<CFGEdge>::new();

            let graph = simplify(&self);
            //let first_loop = paths.first().unwrap();
            let mut loop_head = 0;
            let mut loop_back = 0;
            let mut loop_break = Vec::<usize>::new();
            let mut loop_exit = 0;

            let mut nid = self.nodes.len();

            let loop_id = nid;
            let loop_node = CFGNode {
                id: loop_id,
                index: 0,
                node_type: NodeType::Loop,
                irs: Vec::new(),
            };
            nid += 1;
            self.nodes.insert(loop_id, loop_node);


            //let structure_id = structure_counter.get();
            if entrys_map.len() > 1 {
                let mut structure_condictions_map = HashMap::<usize, Condiction>::new();
                let structure_id = structure_counter.get();
                /*
                println!("\n---------- control flow graph information ----------");
                for node in self.nodes.iter() {
                    print!("{} ", &node.0);
                }
                println!("");
                println!("nodes: ");
                for node in self.nodes.iter() {
                    let self_node = node.1;
                    println!("id: {}\ttype: {:?} ", self_node.id, self_node.node_type);
                    for ir in self_node.irs.iter() {
                        println!("{}\t", ir);
                    }
                    println!("\n");
                }
                println!("edges: ");
                for edge in self.edges.iter() {
                    let self_edge = &edge.1;
                    if let Some(cond) = self_edge.condiction.as_ref() {
                        println!("id: {}\t{} -> {} type: {:?}\tcondiction: {}\tis_true: {:?}", edge.0, self_edge.from, self_edge.to, self_edge.edge_type, self.condiction[cond], self_edge._true);
                    } else {
                        println!("id: {}\t{} -> {} type: {:?}\tcondiction: {:?}\tis_true: {:?}", edge.0, self_edge.from, self_edge.to, self_edge.edge_type, self_edge.condiction, self_edge._true);
                    }
                    //println!("id: {}\t{} -> {} type: {:?}\ncondiction: {:?}\tis_true: {:?}\n", edge.0, self_edge.from, self_edge.to, self_edge.edge_type, , self_edge._true);
                }
                println!("------------------------------------------------------\n");
                */

                let mut entrys_vec = Vec::<(usize, Vec<usize>)>::new();
                for entry in entrys_map.iter() {
                    entrys_vec.push((*entry.0, entry.1.clone()));
                } 
                entrys_vec.sort_by(|a, b| a.0.cmp(&b.0));

                loop_head = entrys_vec[0].0;
                let mut loop_head_old = loop_head;
                //self.info();
                
                let mut pre_node_id = loop_id;
                let mut pre_cond = Condiction {
                    relation: Relation::L,
                    operand1: DFIOperand::Number(Number::from(0, false, Size::Unsigned64)),
                    operand2: DFIOperand::Number(Number::from(0, false, Size::Unsigned64)),
                };
                let mut set_ir = DataFlowIr {
                    address: 0,
                    opcode: DataFlowIrOpcode::Add,
                    operand1: None,
                    operand2: None,
                    operand3: None,
                };
                let mut back_value = 0;

                let mut new_tmp = new_temp(temps);
                let mut structure_val_counter = Counter::new();
                for (i, (v, entryv)) in entrys_vec.iter().enumerate() {
                    for entry in entryv.iter() {
                        let from = entry;
                        let to = v;

                        let mut old_egde = match self.get_cfg_edge(*from, *to as u64) {
                            Some(e) => e,
                            None => {
                                println!("{} -> {}", from, to);
                                self.info();
                                panic!("error")
                            }
                        };
                        self.edges.remove(&old_egde.id);

                        let value = structure_val_counter.get();
                        let cond = create_condiction(&new_tmp, value);
                        let cond_id = get_condiction_id(self, cond.clone());
                        structure_condictions_map.insert(cond_id, cond.clone());
                        let ir = create_set_ir(&new_tmp, value);
                        
                        let mut irs = Vec::new();
                        irs.push(ir);

                        let set_node = CFGNode {
                            id: nid,
                            index: 0,
                            node_type: NodeType::Set,
                            irs,
                        };
                        nid += 1;
                        
                        if i == 0 {
                            back_value = value;
                        }

                        old_egde.to = set_node.id as u64;
                        self.edges.insert(old_egde.id, old_egde);

                        let entry_loop_edge = CFGEdge {
                            id: self.new_edge_id(),
                            from: set_node.id,
                            to: loop_id as u64,
                            edge_type: EdgeType::None,
                            condiction: None,
                            _true: None,
                        };
                        self.edges.insert(entry_loop_edge.id, entry_loop_edge);
                        self.nodes.insert(set_node.id, set_node);

                        if i == 0 {
                            let if_node = CFGNode {
                                id: nid,
                                index: 0,
                                node_type: NodeType::If,
                                irs: Vec::new(),
                            };
                            nid += 1;
                            let edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: loop_id,
                                to: if_node.id as u64,
                                edge_type: EdgeType::LoopEnter,
                                condiction: None,
                                _true: None,
                            }; 
                            self.edges.insert(edge.id, edge);
                            let true_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: if_node.id,
                                to: *to as u64,
                                edge_type: EdgeType::None,
                                condiction: Some(cond.clone()),
                                _true: Some(true),
                            };

                            self.edges.insert(true_edge.id, true_edge);

                            pre_cond = cond;
                            pre_node_id = if_node.id;
                            self.nodes.insert(if_node.id, if_node);
                        } else if i != entrys_vec.len() - 1 {
                            let if_node = CFGNode {
                                id: nid,
                                index: 0,
                                node_type: NodeType::If,
                                irs: Vec::new(),
                            };
                            nid += 1;

                            let false_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: pre_node_id,
                                to: if_node.id as u64,
                                edge_type: EdgeType::None,
                                condiction: Some(pre_cond.clone()),
                                _true: Some(false),
                            };
                            self.edges.insert(false_edge.id, false_edge);

                            let true_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: if_node.id,
                                to: *to as u64,
                                edge_type: EdgeType::None,
                                condiction: Some(cond.clone()),
                                _true: Some(true),
                            };
                            self.edges.insert(true_edge.id, true_edge);

                            pre_cond = cond;
                            pre_node_id = if_node.id;

                            self.nodes.insert(if_node.id, if_node);
                        } else {
                            let false_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: pre_node_id,
                                to: *to as u64,
                                edge_type: EdgeType::None,
                                condiction: Some(pre_cond.clone()),
                                _true: Some(false),
                            };

                            self.edges.insert(false_edge.id, false_edge);
                        }

                    }
                    /*
                    if i != entrys_vec.len() - 1 {
                        for entry in entryv.iter() {
                            let from = entry;
                            let to = v;

                            let old_egde = match self.get_cfg_edge(*from, *to) {
                                Some(old_egde) => old_egde,
                                None => panic!("error"),
                            };

                            let set_node = CFGNode {
                                id: nid,
                                index: 0,
                                node_type: NodeType::None,
                                irs: Vec::new(),
                            };
                            nid += 1;

                            let mut set_edge = old_egde.clone();
                            set_edge.to = set_node.id;
                            set_edge.edge_type = EdgeType::None;
                            println!("set edge: from: {} to: {}", set_edge.from, set_edge.to);
                            self.edges.insert(set_edge.id, set_edge);

                            
                            let entry_loop_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: set_node.id,
                                to: loop_id,
                                edge_type: EdgeType::None,
                                condiction: None,
                                _true: None,
                            };

                            let new_temp = new_temp(temps);
                            let value = structure_val_counter.get();
                            let ir = create_set_ir(&new_temp, value);
                            let mut irs = Vec::new();
                            if i == 0 {
                                set_ir = ir.clone();
                            }
                            irs.push(ir);

                            let if_node = CFGNode {
                                id: nid,
                                index: 0,
                                node_type: NodeType::None,
                                irs,
                            };
                            nid += 1;

                            //self.edges.remove(&old_egde.id);
                            //self.edges.insert(set_edge.id, set_edge);
                            self.nodes.insert(set_node.id, set_node);
                            self.edges.insert(entry_loop_edge.id, entry_loop_edge);
                            //self.edges.insert(entry_loop_redirect.id, entry_loop_redirect);
                            
                            
                            let cond = create_condiction(&new_temp, value);
                            let cond_id = get_condiction_id(self, cond.clone());

                            let true_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: if_node.id,
                                to: old_egde.to,
                                edge_type: EdgeType::None,
                                condiction: Some(cond.clone()),
                                _true: Some(true),
                            };

                            self.edges.insert(true_edge.id, true_edge);

                            if i == 0 {
                                //println!("i == 0: {} -> {}", pre_node_id, if_node.id);
                                for (i, v) in first_loop.iter().enumerate() {
                                    if first_loop[(i + 1) % first_loop.len()] == loop_head {
                                        loop_back = *v;
                                        first_loop.insert((i + 1) % first_loop.len(), if_node.id);
                                        loop_head = if_node.id;
                                        /*
                                        println!("{}", if_node.id);
                                        println!("head: {}", loop_head);
                                        println!("back: {}", loop_back);
                                        println!("{:?}", first_loop);
                                        panic!("{}", i);
                                        */
                                        break;
                                    }
                                }
                                let pre_edge = CFGEdge {
                                    id: self.new_edge_id(),
                                    from: pre_node_id,
                                    to: if_node.id,
                                    edge_type: EdgeType::LoopEnter,
                                    condiction: None,
                                    _true: None,
                                };

                                self.edges.insert(pre_edge.id, pre_edge);
                            } else {
                                //println!("false: {} -> {}", pre_node_id, if_node.id);
                                let pre_edge = CFGEdge {
                                    id: self.new_edge_id(),
                                    from: pre_node_id,
                                    to: if_node.id,
                                    edge_type: EdgeType::None,
                                    condiction: Some(pre_cond.clone()), 
                                    _true: Some(false),
                                }; 

                                self.edges.insert(pre_edge.id, pre_edge);
                            }

                            pre_node_id = if_node.id;
                            pre_cond = cond;

                            self.nodes.insert(if_node.id, if_node);
                        }
                    } else {
                        for entry in entryv.iter() {
                            let from = entry;
                            let to = v;

                            let old_egde = match self.get_cfg_edge(*from, *to) {
                                Some(old_egde) => old_egde,
                                None => panic!("error"),
                            };

                            let set_node = CFGNode {
                                id: nid,
                                index: 0,
                                node_type: NodeType::None,
                                irs: Vec::new(),
                            };
                            nid += 1;

                            let mut set_edge = old_egde.clone();
                            set_edge.to = set_node.id;
                            set_edge.edge_type = EdgeType::None;
                            self.edges.insert(set_edge.id, set_edge);

                            let entry_loop_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: set_node.id,
                                to: loop_id,
                                edge_type: EdgeType::None,
                                condiction: None,
                                _true: None,
                            };


                            //self.edges.remove(&old_egde.id);
                            //self.edges.insert(set_edge.id, set_edge);
                            self.nodes.insert(set_node.id, set_node);
                            self.edges.insert(entry_loop_edge.id, entry_loop_edge);


                            let last_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: pre_node_id,
                                to: old_egde.to,
                                edge_type: EdgeType::None,
                                condiction: Some(pre_cond.clone()),
                                _true: Some(false),
                            };
                            //println!("last {} -> {}", last_edge.from, last_edge.to);

                            self.edges.insert(last_edge.id, last_edge);

                            /*
                            let mut entry_loop_redirect = old_egde.clone();
                            entry_loop_redirect.to = loop_id;
                            entry_loop_redirect.edge_type = EdgeType::LoopEnter;

                            self.edges.remove(&old_egde.id);
                            self.edges.insert(entry_loop_redirect.id, entry_loop_redirect);
                            */
                        }
                    }
                */
                }
                //self.info();
                //panic!("");

                //println!("old head: {}", loop_head_old);
                //println!("head: {}", loop_head);
                //println!("back: {}", loop_back);
                //println!("break: {:?}", loop_break);

                for (i, v) in first_loop.iter().enumerate() {
                    if first_loop[(i + 1) % first_loop.len()] == loop_head {
                        loop_back = *v;
                        break;
                    }
                }

                let mut irs = Vec::new();
                irs.push(create_set_ir(&new_tmp, back_value));
                let mut back_node = CFGNode {
                    id: nid,
                    index: 0,
                    node_type: NodeType::Back,
                    irs,
                };
                nid += 1;
                self.nodes.insert(back_node.id, back_node.clone());
                let back_id = back_node.id;

                let mut old_egde = match self.get_cfg_edge(loop_back, loop_head_old as u64) {
                    Some(old_egde) => old_egde,
                    None => panic!("error"),
                };

                old_egde.to = back_node.id as u64;
                old_egde.edge_type = EdgeType::Back;
                self.edges.remove(&old_egde.id);
                self.edges.insert(old_egde.id, old_egde);

                let sink_node = CFGNode {
                    id: nid,
                    index: 0,
                    node_type: NodeType::None,
                    irs: Vec::new(),
                };
                nid += 1;
                let sink_id = sink_node.id;
                self.nodes.insert(sink_id, sink_node);

                let back_sink_edge = CFGEdge {
                    id: self.new_edge_id(),
                    from: back_id,
                    to: sink_id as u64,
                    edge_type: EdgeType::None,
                    condiction: None,
                    _true: None,
                };

                self.edges.insert(back_sink_edge.id, back_sink_edge);

                for exit in exits_map.iter() {
                    loop_exit = *exit.0;
                    break;
                }

                for v in exits_map[&loop_exit].iter() {
                    let from = *v;
                    let to = loop_exit;

                    let old_egde = match self.get_cfg_edge(*v, loop_exit as u64) {
                        Some(old_egde) => old_egde,
                        None => panic!("error"),
                    };               

                    let break_node = CFGNode {
                        id: nid,
                        index: 0,
                        node_type: NodeType::Break,
                        irs: Vec::new(),
                    };
                    nid += 1;
                    let break_id = break_node.id;

                    let break_edge = CFGEdge {
                        id: self.new_edge_id(),
                        from,
                        to: break_node.id as u64,
                        edge_type: EdgeType::Break,
                        condiction: old_egde.condiction,
                        _true: old_egde._true,
                    };


                    self.edges.remove(&old_egde.id);
                    self.edges.insert(break_edge.id, break_edge);
                    self.nodes.insert(break_node.id, break_node);
                    
                    let break_sink_edge = CFGEdge {
                        id: self.new_edge_id(),
                        from: break_id,
                        to: sink_id as u64,
                        edge_type: EdgeType::None,
                        condiction: None,
                        _true: None,
                    };

                    self.edges.insert(break_sink_edge.id, break_sink_edge);
                }


                let exit_edge = CFGEdge {
                    id: self.new_edge_id(),
                    from: loop_id,
                    to: loop_exit as u64,
                    edge_type: EdgeType::LoopExit,
                    condiction: None,
                    _true: None,
                };

                self.edges.insert(exit_edge.id, exit_edge);


                /*
                println!("{}", loop_head);
                println!("{}", loop_back);
                
                self.info();
                
                println!("{:?}", first_loop);
                println!("{:?}", entrys_map);
                panic!("error");
                */

                //self.info();
                //panic!("finish");

                return structure_condictions_map;

            } else {
                for entry in entrys_map.iter() {
                    loop_head = *entry.0;
                    break;
                }
            }

            for v in first_loop.iter() {
                if graph.childs[v].iter().any(|c| *c == loop_head) {
                    loop_back = *v;
                    break;
                }
            }


            /*
            if exits_map.len() > 1 {
                let exit_sink_id = nid;
                let mut exit_sink = CFGNode {
                    id: exit_sink_id,
                    index: 0,
                    node_type: NodeType::None,
                    irs: Vec::new(),
                };
                nid += 1;
                self.nodes.insert(exit_sink_id, exit_sink);


                //println!("{:?}", exits_map);
                //println!("{:?}", first_loop);
                let mut exits_vec = Vec::<(usize, Vec<usize>)>::new();
                for v in first_loop.iter() {
                    for map in exits_map.iter() {
                        if map.1.iter().any(|e| *e == *v) {
                            exits_vec.push((*map.0, map.1.clone()));
                            break;
                        }
                    }
                }
                //println!("{:?}", exits_vec);

                let mut pre_edge = CFGEdge {
                    id: 0,
                    from: 0,
                    to: 0,
                    edge_type: EdgeType::None,
                    condiction: None,
                    _true: None,
                };

                for (i, (exit, breakv)) in exits_vec.iter().enumerate() {
                    if i != exits_map.len() -1 {
                        for b in breakv.iter() {
                            let from = b;
                            let to = exit;

                            let old_egde = match self.get_cfg_edge(*from, *to) {
                                Some(old_egde) => old_egde,
                                None => panic!("error"),
                            };

                            self.edges.remove(&old_egde.id);

                            let mut edge = old_egde;
                            edge.to = exit_sink_id;
                            //----------------------------------------edge.edge_type = EdgeType::Break;
                            edge.edge_type = EdgeType::None;
                            
                            self.edges.insert(edge.id, edge.clone());

                            if i == 0 {
                                edge.from = exit_sink_id;
                                edge.to = *to;
                                edge.id = self.new_edge_id();

                                self.edges.insert(edge.id, edge.clone());

                                pre_edge.id = self.new_edge_id();
                                pre_edge.from = exit_sink_id;
                                pre_edge.condiction = edge.condiction.clone();
                                pre_edge.edge_type = EdgeType::None;
                                
                                match edge._true.as_ref() {
                                    Some(true) => pre_edge._true = Some(false),
                                    Some(false) => pre_edge._true = Some(true),
                                    None => panic!("error"),
                                }

                            } else {
                                let mut node = CFGNode {
                                    id: nid,
                                    index: 0,
                                    node_type: NodeType::None,
                                    irs: Vec::new(),
                                };
                                nid += 1;

                                pre_edge.to = node.id;
                                self.edges.insert(pre_edge.id, pre_edge.clone());

                                let edge = CFGEdge {
                                    id: self.new_edge_id(),
                                    from: node.id,
                                    to: *to,
                                    edge_type: EdgeType::None,
                                    condiction: edge.condiction.clone(),
                                    _true: edge._true.clone(),
                                };

                                self.edges.insert(edge.id, edge.clone());


                                pre_edge.id = self.new_edge_id();
                                pre_edge.from = node.id;
                                pre_edge.condiction = edge.condiction.clone();
                                match edge._true.as_ref() {
                                    Some(true) => pre_edge._true = Some(false),
                                    Some(false) => pre_edge._true = Some(true),
                                    None => panic!("error"),
                                }

                                self.nodes.insert(node.id, node);
                            }

                        }
                    } else {
                        for b in breakv.iter() {
                            let from = b;
                            let to = exit;

                            let old_egde = match self.get_cfg_edge(*from, *to) {
                                Some(old_egde) => old_egde,
                                None => panic!("error"),
                            };

                            self.edges.remove(&old_egde.id);

                            let mut edge = old_egde;
                            edge.to = exit_sink_id;
                            edge.edge_type = EdgeType::Break;

                            self.edges.insert(edge.id, edge);

                            pre_edge.to = *to;

                            self.edges.insert(pre_edge.id, pre_edge.clone());
                        }
                    } 
                }

                for (i, (exit, breakv)) in exits_map.iter().enumerate() {
                    if i != exits_map.len() -1 {
                        for b in breakv.iter() {
                            let from = b;
                            let to = exit;

                            let old_egde = match self.get_cfg_edge(*from, *to) {
                                Some(old_egde) => old_egde,
                                None => panic!("error"),
                            };

                            self.edges.remove(&old_egde.id);

                            let mut edge = old_egde;
                            edge.to = exit_sink_id;
                            //----------------------------------------edge.edge_type = EdgeType::Break;
                            edge.edge_type = EdgeType::None;
                            
                            self.edges.insert(edge.id, edge.clone());

                            if i == 0 {
                                edge.from = exit_sink_id;
                                edge.to = *to;
                                edge.id = self.new_edge_id();

                                self.edges.insert(edge.id, edge.clone());

                                pre_edge.id = self.new_edge_id();
                                pre_edge.from = exit_sink_id;
                                pre_edge.condiction = edge.condiction.clone();
                                pre_edge.edge_type = EdgeType::None;
                                
                                match edge._true.as_ref() {
                                    Some(true) => pre_edge._true = Some(false),
                                    Some(false) => pre_edge._true = Some(true),
                                    None => panic!("error"),
                                }

                            } else {
                                let mut node = CFGNode {
                                    id: nid,
                                    index: 0,
                                    node_type: NodeType::None,
                                    irs: Vec::new(),
                                };
                                nid += 1;

                                pre_edge.to = node.id;
                                self.edges.insert(pre_edge.id, pre_edge.clone());

                                let edge = CFGEdge {
                                    id: self.new_edge_id(),
                                    from: node.id,
                                    to: *to,
                                    edge_type: EdgeType::None,
                                    condiction: edge.condiction.clone(),
                                    _true: edge._true.clone(),
                                };

                                self.edges.insert(edge.id, edge.clone());


                                pre_edge.id = self.new_edge_id();
                                pre_edge.from = node.id;
                                pre_edge.condiction = edge.condiction.clone();
                                match edge._true.as_ref() {
                                    Some(true) => pre_edge._true = Some(false),
                                    Some(false) => pre_edge._true = Some(true),
                                    None => panic!("error"),
                                }

                                self.nodes.insert(node.id, node);
                            }

                        }
                    } else {
                        for b in breakv.iter() {
                            let from = b;
                            let to = exit;

                            let old_egde = match self.get_cfg_edge(*from, *to) {
                                Some(old_egde) => old_egde,
                                None => panic!("error"),
                            };

                            self.edges.remove(&old_egde.id);

                            let mut edge = old_egde;
                            edge.to = exit_sink_id;
                            edge.edge_type = EdgeType::Break;

                            self.edges.insert(edge.id, edge);

                            pre_edge.to = *to;

                            self.edges.insert(pre_edge.id, pre_edge.clone());
                        }
                    } 
                }

                /*
                println!("-----------------------------");
                println!("nodes: ");
                for v in self.nodes.iter() {
                    print!("{} ", v.0);
                }
                println!("\nedges: ");
                for e in self.edges.iter() {
                    println!("{} -> {}", e.1.from, e.1.to);
                }
                */
                //panic!("error");
                (entrys_map, exits_map) = get_entry_exit(&self, &first_loop);
                for exit in exits_map.iter() {
                    loop_exit = *exit.0;
                    loop_break = exit.1.clone();
                }

                //println!("exits_map: {:?}", exits_map);

                //panic!("error");
            } else {
                for exit in exits_map.iter() {
                    loop_break = exit.1.clone();
                    loop_exit = *exit.0;
                    break;
                }
            }
            */

            if exits_map.len() > 1 {
                let mut structure_condictions_map = HashMap::<usize, Condiction>::new();
                let structure_id = structure_counter.get();
                let mut loop_entry = 0;
                let mut loop_back = 0;
                for (_, edge) in self.edges.iter() {
                    if edge.to == loop_head as u64 {
                        if first_loop.iter().any(|v| *v == edge.from) {
                            loop_back = edge.from;
                        } else {
                            loop_entry = edge.from; 
                        }
                    }
                }

                let mut old_egde = match self.get_cfg_edge(loop_entry, loop_head as u64) {
                    Some(e) => e,
                    None => panic!("error"),
                };

                self.edges.remove(&old_egde.id);
                old_egde.to = loop_id as u64;
                self.edges.insert(old_egde.id, old_egde);

                let enter_edge = CFGEdge {
                    id: self.new_edge_id(),
                    from: loop_id,
                    to: loop_head as u64,
                    edge_type: EdgeType::LoopEnter,
                    condiction: None,
                    _true: None,
                };

                self.edges.insert(enter_edge.id, enter_edge);

                let exit_sink_id = nid;
                let mut exit_sink = CFGNode {
                    id: exit_sink_id,
                    index: 0,
                    node_type: NodeType::None,
                    irs: Vec::new(),
                };
                nid += 1;
                self.nodes.insert(exit_sink_id, exit_sink);

                let loop_exit_edge = CFGEdge {
                    id: self.new_edge_id(),
                    from: loop_id,
                    to: exit_sink_id as u64,
                    edge_type: EdgeType::LoopExit,
                    condiction: None,
                    _true: None,
                };
                self.edges.insert(loop_exit_edge.id, loop_exit_edge);

                let mut exits_vec = Vec::<(usize, Vec<usize>)>::new();
                for v in first_loop.iter() {
                    for map in exits_map.iter() {
                        if map.1.iter().any(|e| *e == *v) {
                            exits_vec.push((*map.0, map.1.clone()));
                            break;
                        }
                    }
                }

                let back_node = CFGNode {
                    id: nid,
                    index: 0,
                    node_type: NodeType::Back,
                    irs: Vec::new(),
                };
                let back_id = back_node.id;
                nid += 1;
                let mut old_egde = self.get_cfg_edge(loop_back, loop_head as u64).unwrap();
                old_egde.to = back_node.id as u64;
                self.nodes.insert(back_node.id, back_node);
                self.edges.insert(old_egde.id, old_egde);
                

                let tmp = new_temp(temps);

                let mut structure_val_counter = Counter::new();
                let mut pre_node_id = exit_sink_id;
                let mut pre_cond = Condiction {
                    relation: Relation::L,
                    operand1: DFIOperand::Number(Number::from(0, false, Size::Unsigned64)),
                    operand2: DFIOperand::Number(Number::from(0, false, Size::Unsigned64)),
                };

                let sink_node = CFGNode {
                    id: nid,
                    index: 0,
                    node_type: NodeType::None,
                    irs: Vec::new(),
                };
                nid += 1;
                let sink_id = sink_node.id;
                self.nodes.insert(sink_id, sink_node);

                for (i, (exit, breakv)) in exits_vec.iter().enumerate() {
                    for b in breakv.iter() {
                        let mut old_egde = match self.get_cfg_edge(*b, *exit as u64) {
                            Some(e) => e,
                            None => panic!("error"),
                        };

                        self.edges.remove(&old_egde.id);
                        
                        let value = structure_val_counter.get(); 
                        let ir = create_set_ir(&tmp, value);
                        let mut irs = Vec::new();
                        irs.push(ir);

                        let break_node = CFGNode {
                            id: nid,
                            index: 0,
                            node_type: NodeType::Break,
                            irs,
                        };
                        nid += 1;
                        let break_id = break_node.id;

                        //let cond = create_condiction(&tmp, value); 
                        let old_to = old_egde.to;
                        old_egde.to = break_node.id as u64;
                        self.edges.insert(old_egde.id, old_egde.clone());

                        if i == 0 {
                            let cond = create_condiction(&tmp, value);
                            let cond_id = get_condiction_id(self, cond.clone());
                            structure_condictions_map.insert(cond_id, cond.clone());
                            let true_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: exit_sink_id,
                                to: old_to,
                                edge_type: EdgeType::If,
                                condiction: Some(cond.clone()),
                                _true: Some(true),
                            };
                            self.edges.insert(true_edge.id, true_edge);
                            pre_node_id = exit_sink_id;
                            pre_cond = cond; 
                        } else if i == exits_vec.len() - 1 {
                            let false_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: pre_node_id,
                                to: old_to,
                                edge_type: EdgeType::If,
                                condiction: Some(pre_cond.clone()),
                                _true: Some(false),
                            };
                            self.edges.insert(false_edge.id, false_edge);
                        } else {
                            let cond = create_condiction(&tmp, value);
                            let cond_id = get_condiction_id(self, cond.clone());

                            structure_condictions_map.insert(cond_id, cond.clone());

                            let if_node = CFGNode {
                                id: nid,
                                index: 0,
                                node_type: NodeType::If,
                                irs: Vec::new(),
                            };
                            nid += 1;

                            let false_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: pre_node_id,
                                to: if_node.id as u64,
                                edge_type: EdgeType::If,
                                condiction: Some(pre_cond.clone()),
                                _true: Some(false),
                            };
                            self.edges.insert(false_edge.id, false_edge);

                            pre_node_id = if_node.id;
                            pre_cond = cond.clone();

                            let true_edge = CFGEdge {
                                id: self.new_edge_id(),
                                from: if_node.id,
                                to: old_to,
                                edge_type: EdgeType::If,
                                condiction: Some(cond.clone()),
                                _true: Some(true),
                            };
                            self.edges.insert(true_edge.id, true_edge);
                            
                            self.nodes.insert(if_node.id, if_node);
                        }

                        self.nodes.insert(break_node.id, break_node);

                        let break_sink_edge = CFGEdge {
                            id: self.new_edge_id(),
                            from: break_id,
                            to: sink_id as u64,
                            edge_type: EdgeType::None,
                            condiction: None,
                            _true: None,
                        };
                        self.edges.insert(break_sink_edge.id, break_sink_edge);
                        
                    } 
                }

                let back_sink_edge = CFGEdge {
                    id: self.new_edge_id(),
                    from: back_id,
                    to: sink_id as u64,
                    edge_type: EdgeType::None,
                    condiction: None,
                    _true: None,
                };
                self.edges.insert(back_sink_edge.id, back_sink_edge);

                return structure_condictions_map;
              
            } else {
                for exit in exits_map.iter() {
                    loop_break = exit.1.clone();
                    loop_exit = *exit.0;
                    break;
                }
            }
            
            let mut entrys = HashSet::<usize>::new();
            let mut exits = HashSet::<usize>::new();

            //println!("entrys: {:?}", entrys_map);
            //println!("exits: {:?}", entrys_map);
            //println!("{:?}", paths);
            //println!("head: {}", loop_head);
            for v in entrys_map[&loop_head].iter() {
                let old_egde = match self.get_cfg_edge(*v, loop_head as u64) {
                    Some(old_egde) => old_egde,
                    None => panic!("error"),
                };
                let edge_id = self.new_edge_id();
                let new_edge = CFGEdge {
                    id: edge_id,
                    from: *v,
                    to: loop_id as u64,
                    edge_type: EdgeType::None,
                    condiction: old_egde.condiction,
                    _true: old_egde._true,
                }; 

                self.edges.remove(&old_egde.id);
                self.edges.insert(edge_id, new_edge);
            }
            
            /*
            let break_node = CFGNode {
                id: nid,
                index: 0,
                node_type: NodeType::Break,
                irs: Vec::new(),
            };
            nid += 1;
            */

            let sink_node = CFGNode {
                id: nid,
                index: 0,
                node_type: NodeType::None,
                irs: Vec::new(),
            };
            nid += 1;
            let sink_node_id = sink_node.id;
            self.nodes.insert(sink_node_id, sink_node);


            for v in exits_map[&loop_exit].iter() {
                let from = *v;
                let to = loop_exit;

                let old_egde = match self.get_cfg_edge(*v, loop_exit as u64) {
                    Some(old_egde) => old_egde,
                    None => panic!("error"),
                };               

                let break_node = CFGNode {
                    id: nid,
                    index: 0,
                    node_type: NodeType::Break,
                    irs: Vec::new(),
                };
                nid += 1;
                let break_id = break_node.id;

                let break_edge = CFGEdge {
                    id: self.new_edge_id(),
                    from,
                    to: break_node.id as u64,
                    edge_type: EdgeType::Break,
                    condiction: old_egde.condiction,
                    _true: old_egde._true,
                };


                self.edges.remove(&old_egde.id);
                self.edges.insert(break_edge.id, break_edge);
                self.nodes.insert(break_node.id, break_node);

                let break_sink_edge = CFGEdge {
                    id: self.new_edge_id(),
                    from: break_id,
                    to: sink_node_id as u64,
                    edge_type: EdgeType::None,
                    condiction: None,
                    _true: None,
                };

                self.edges.insert(break_sink_edge.id, break_sink_edge);

            }


            let exit_edge = CFGEdge {
                id: self.new_edge_id(),
                from: loop_id,
                to: loop_exit as u64,
                edge_type: EdgeType::LoopExit,
                condiction: None,
                _true: None,
            };

            self.edges.insert(exit_edge.id, exit_edge);


            let old_back_edge = match self.get_cfg_edge(loop_back, loop_head as u64) {
                Some(edge) => edge,
                None => panic!("error"),
            };

            let back_node = CFGNode {
                id: nid,
                index: 0,
                node_type: NodeType::Back,
                irs: Vec::new(),
            };
            nid += 1;
            let back_id = back_node.id;

            let back_edge = CFGEdge {
                id: self.new_edge_id(),
                from: loop_back,
                to: back_node.id as u64,
                edge_type: EdgeType::Back,
                condiction: old_back_edge.condiction,
                _true: old_back_edge._true,
            };

            self.edges.remove(&old_back_edge.id);
            self.edges.insert(back_edge.id, back_edge);

            let enter_edge = CFGEdge {
                id: self.new_edge_id(), 
                from: loop_id,
                to: loop_head as u64,
                edge_type: EdgeType::LoopEnter,
                condiction: None,
                _true: None,
            };

            self.edges.insert(enter_edge.id, enter_edge);

            //self.nodes.insert(break_node.id, break_node);
            self.nodes.insert(back_node.id, back_node);

            let back_sink_edge = CFGEdge {
                id: self.new_edge_id(),
                from: back_id,
                to: sink_node_id as u64,
                edge_type: EdgeType::None,
                condiction: None,
                _true: None,
            };
            self.edges.insert(back_sink_edge.id, back_sink_edge);

            /*
            for e in self.edges.iter() {
                println!("{} -> {}", e.1.from, e.1.to);
            }
            */
            /*
            println!("-----------------------------");
            println!("nodes: ");
            for v in self.nodes.iter() {
                println!("{}: {:?}", v.0, v.1.node_type);
            }
            println!("\nedges: ");
            for e in self.edges.iter() {
                println!("{} -> {}", e.1.from, e.1.to);
            }
            */


            /*
            println!("loop: {:?}", paths);
            println!("entrys: {:?}", entrys_map);
            println!("exits: {:?}", exits_map);
            println!("head {}", loop_head);
            println!("break {}", loop_break);
            prointln!("back {}\n", loop_back);
            */

        }
        HashMap::new()
    }

    fn get_cfg_edge(&self, from: usize, to: u64) -> Option<CFGEdge> {
        for edge in self.edges.iter() {
            if edge.1.from == from && edge.1.to == to{
                return Some(edge.1.clone());
            }
        } 
        None
    }

    fn new_edge_id(&self) -> usize {
        let mut i = 0;
        while i < usize::MAX {
            if let None = self.edges.get(&i) {
                break;
            } 
            i += 1;
        }

        return i;
    }
}

impl ControlFlowTree {
    pub fn new(root: usize) -> Self {
        Self {
            id: root,
            is_sink: false,
            node_type: NodeType::None,
            access_condiction: Vec::new(),
            condiction: None,
            true_next: None,
            false_next: None,
            loop_region: None,
            next: Vec::new(),
        }
    }

    pub fn merge(cft_trees_map: &HashMap<usize, ControlFlowTree>, loop_slices_map: &HashMap<usize, usize>, topo: &Vec<usize>) -> Self {
        let mut trees = cft_trees_map.clone();

        let mut cft = match trees.remove(&0) {
            Some(t) => t,
            None => panic!("error"),
        };

        /*
        for (i, tree) in cft_trees_map.iter() {
            println!("{}: {:#?}", i, tree);
        } 
        */
        for v in topo.iter() {
            if let Some(loop_head) = loop_slices_map.get(v) {
                let tree = &cft_trees_map[loop_head];
                cft.insert_loop_tree(*v, tree);
            }
        }

        //println!("{:#?}", cft);


        cft
    }

    pub fn insert_loop_tree(&mut self, parent: usize, tree: &ControlFlowTree) {
        if self.id == parent {
            //println!("pre tree: \n{:#?}", self);
            self.loop_region = Some(Box::new(tree.clone()));
            //println!("after tree: \n{:#?}", self);
        } else {
            if let Some(true_next) = self.true_next.as_mut() {
                true_next.insert_loop_tree(parent, tree);
            }
            if let Some(false_next) = self.false_next.as_mut() {
                false_next.insert_loop_tree(parent, tree);
            }
            if let Some(loop_region) = self.loop_region.as_mut() {
                loop_region.insert_loop_tree(parent, tree);
            }
            for next in self.next.iter_mut() {
                next.insert_loop_tree(parent, tree);
            }
        }
    }


    pub fn insert(&mut self, parent: usize, id: usize, branch: Branch, condiction: Option<isize>, node_type: &NodeType) {
        if self.id  == parent {
            let mut tree = ControlFlowTree::new(id);
            self.condiction = condiction;
            //let mut c = self.access_condiction[0].clone();
            /*
            let mut c = match self.access_condiction.get(0) {
                Some(c) => c.clone(),
                None => Vec::new(),
            };
            */
            let mut cond_id = 0;
            if let Some(id) = condiction {
                cond_id = id;
            }
            match &branch {
                Branch::Next => {
                    /*
                    let mut cond = Vec::new();
                    cond.push(c);
                    */
                    //let mut cond = self.access_condiction.clone();
                    //tree.access_condiction = cond;
                    match node_type {
                        NodeType::None | NodeType::Set => self.next.push(Box::new(tree)),
                        NodeType::Loop => {
                            tree.node_type = NodeType::Loop;
                            self.next.push(Box::new(tree));
                        }
                        NodeType::Back => {
                            tree.node_type = NodeType::Back;
                            self.next.push(Box::new(tree));
                        }
                        NodeType::Break => {
                            tree.node_type = NodeType::Break;
                            self.next.push(Box::new(tree));
                        }
                        _ => {}
                    }
                    //self.next = Some(Box::new(tree));  
                }
                Branch::True => {
                    /*
                    let mut cond = Vec::new();
                    c.push(cond_id);
                    cond.push(c);
                    */
                    let mut cond;
                    if self.access_condiction.len() == 0 {
                        cond = Vec::new();
                        cond.push(Vec::new());
                    } else {
                        cond = self.access_condiction.clone();
                    }
                    for c in cond.iter_mut() {
                        c.push(cond_id);
                    }
                    tree.access_condiction = cond;
                    match node_type {
                        NodeType::Break => tree.node_type = NodeType::Break,
                        NodeType::Back => tree.node_type = NodeType::Back,
                        _ => {}
                    }
                    self.true_next = Some(Box::new(tree));
                }
                Branch::False => {
                    /*
                    let mut cond = Vec::new();
                    c.push(-cond_id);
                    cond.push(c);
                    */
                    let mut cond;
                    if self.access_condiction.len() == 0 {
                        cond = Vec::new();
                        cond.push(Vec::new());
                    } else {
                        cond = self.access_condiction.clone();
                    }
                    for c in cond.iter_mut() {
                        c.push(-cond_id);
                    }
                    tree.access_condiction = cond;
                    match node_type {
                        NodeType::Break => tree.node_type = NodeType::Break,
                        NodeType::Back => tree.node_type = NodeType::Back,
                        _ => {}
                    }
                    self.false_next = Some(Box::new(tree));
                }
            }
            return;
        } else {
            /*
            if let Some(next) = self.next.as_mut() {
                next.insert(parent, id, branch.clone(), condiction);
            }
            */
            if let Some(true_next) = self.true_next.as_mut() {
                true_next.insert(parent, id, branch.clone(), condiction, node_type);
            } 
            if let Some(false_next) = self.false_next.as_mut() {
                false_next.insert(parent, id, branch.clone(), condiction, node_type);
            }
            for next in self.next.iter_mut() {
                next.insert(parent, id, branch.clone(), condiction, node_type);
            }

            if let Some(loop_region) = self.loop_region.as_mut() {
                loop_region.insert(parent, id, branch.clone(), condiction, node_type);
            }
        } 
    }

    pub fn attach(&mut self, parent: usize, id: usize, node_type: &NodeType) {
        if self.id == parent {
            let mut tree = ControlFlowTree::new(id);
            tree.is_sink = true;
            match node_type {
                NodeType::Loop => {
                    tree.node_type = NodeType::Loop;
                }
                _ => {
                    tree.node_type = NodeType::None;
                }
            }
            self.next.push(Box::new(tree));
            return;
        } else {
            if let Some(true_next) = self.true_next.as_mut() {
                true_next.attach(parent, id, node_type);
            } 
            if let Some(false_next) = self.false_next.as_mut() {
                false_next.attach(parent, id, node_type);
            }
            for next in self.next.iter_mut() {
                next.attach(parent, id, node_type);
            }
        }
    }
    
    pub fn attach_tree(&mut self, target: usize, tree: &ControlFlowTree, id: &Option<usize>) {
        if self.id == target {
            //self.next.push(Box::new(tree.clone()));
            self.true_next = tree.true_next.clone();
            self.false_next = tree.false_next.clone();
            let next = self.next.clone();
            self.next = tree.next.clone();
            for next in next {
                self.next.push(next);
            }
            if let Some(true_next) = tree.true_next.as_ref() {
                let cond_id = *true_next.access_condiction.first().unwrap().first().unwrap();
                self.condiction = Some(cond_id);
            } else if let Some(false_next) = tree.false_next.as_ref() {
                let cond_id = isize::abs(*false_next.access_condiction.first().unwrap().first().unwrap());
                self.condiction = Some(cond_id);
            }
            if let Some(id) = id {
                self.id = *id;
            }
            return;
        } else {
            if let Some(true_next) = self.true_next.as_mut() {
                true_next.attach_tree(target, tree, id);
            } 
            if let Some(false_next) = self.false_next.as_mut() {
                false_next.attach_tree(target, tree, id);
            }
            for next in self.next.iter_mut() {
                next.attach_tree(target, tree, id);
            }
        }
    }

    pub fn put_outer(&mut self, id: usize, access_condiction: Vec<Vec<isize>>, node_type: &NodeType) {
        let mut tree = ControlFlowTree::new(id); 
        tree.is_sink = true;
        tree.access_condiction = quine_mccluskey(access_condiction);
        if let NodeType::Loop = node_type {
            tree.node_type = NodeType::Loop;
        }
        self.next.push(Box::new(tree));
        //self.next.push(Box::new(tree));
    }

    pub fn set_loop(&mut self, loop_id: usize, loop_start: usize) {
        if self.id == loop_id {
            self.node_type = NodeType::Loop;
            let mut tree = ControlFlowTree::new(loop_start);
            self.loop_region = Some(Box::new(tree));
        } else {
            if let Some(true_next) = self.true_next.as_mut() {
                true_next.set_loop(loop_id, loop_start);
            }
            if let Some(false_next) = self.false_next.as_mut() {
                false_next.set_loop(loop_id, loop_start);
            }
            for next in self.next.iter_mut() {
                next.set_loop(loop_id, loop_start);
            }
            if let Some(loop_region) = self.loop_region.as_mut() {
                loop_region.set_loop(loop_id, loop_start);
            }
        }
    }

    pub fn set_break(&mut self, parent: usize, id: usize) {
        if self.id == parent {
            self.node_type = NodeType::Break;
            let mut tree = ControlFlowTree::new(id);
            self.loop_region = Some(Box::new(tree));
        } else {
            if let Some(true_next) = self.true_next.as_mut() {
                true_next.set_break(parent, id);
            }
            if let Some(false_next) = self.false_next.as_mut() {
                false_next.set_break(parent, id);
            }
            for next in self.next.iter_mut() {
                next.set_break(parent, id);
            }
            if let Some(loop_region) = self.loop_region.as_mut() {
                loop_region.set_break(parent, id);
            }
        }
    }

    pub fn set_back(&mut self, parent: usize, id: usize) {
        if self.id == parent {
            let mut tree = ControlFlowTree::new(id);
            tree.node_type = NodeType::Break;
            self.next.push(Box::new(tree));
        } else {
            if let Some(true_next) = self.true_next.as_mut() {
                true_next.set_back(parent, id);
            }
            if let Some(false_next) = self.false_next.as_mut() {
                false_next.set_back(parent, id);
            }
            for next in self.next.iter_mut() {
                next.set_back(parent, id);
            }
            if let Some(loop_region) = self.loop_region.as_mut() {
                loop_region.set_back(parent, id);
            }
        }
    }

    pub fn search(&self, id: usize) -> Option<ControlFlowTree> {
        if self.id == id {
            return Some(self.clone());
        } else {
            if let Some(loop_region) = self.loop_region.as_ref() {
                if let Some(res) = loop_region.search(id) {
                    return Some(res);
                }
            }
            if let Some(true_next) = self.true_next.as_ref() {
                if let Some(res) = true_next.search(id) {
                    return Some(res);
                }
            }   
            if let Some(false_next) = self.false_next.as_ref() {
                 if let Some(res) = false_next.search(id) {
                    return Some(res);
                }   
            }
            for next in self.next.iter() {
                if let Some(res) = next.search(id) {
                    return Some(res);
                }
            }
        }
        None
    }

    pub fn get_parent(&self, id: usize) -> Option<usize> {
        //println!("{} {}", id, self.id);
        if self.next.iter().any(|next| next.id == id) {
            return Some(self.id);
        } else {
             if let Some(true_next) = self.true_next.as_ref() {
                return true_next.get_parent(id);
            }   
            if let Some(false_next) = self.false_next.as_ref() {
                return false_next.get_parent(id);
            }
            for next in self.next.iter() {
                return next.get_parent(id);
            }
        }
        None
    }

    pub fn replace(&mut self, id: usize, tree: &ControlFlowTree) {
        if let Some(true_next) = self.true_next.as_mut() {
            if true_next.id == id {
                let mut tree = tree.clone(); 
                tree.is_sink = true_next.is_sink;
                tree.access_condiction = true_next.access_condiction.clone();
                self.true_next = Some(Box::new(tree));
                return;
            } else {
                true_next.replace(id, tree);
            }
        } 
        if let Some(false_next) = self.false_next.as_mut() {
            if false_next.id == id {
                let mut tree = tree.clone();
                tree.is_sink = false_next.is_sink;
                tree.access_condiction = false_next.access_condiction.clone();
                self.false_next = Some(Box::new(tree));
                return;
            } else {
                false_next.replace(id, tree);
            }
        }

        for next in self.next.iter_mut() {
            if next.id == id {
                let mut tree = tree.clone();
                tree.is_sink = next.is_sink;
                tree.access_condiction = next.access_condiction.clone();
                *next = Box::new(tree);
            } else {
                next.replace(id, tree);
            }
        }
    }

    
    pub fn to_string(&self) -> String {
        let mut tree_str = String::new();
        let mut indent = 0;     
        self.to_string_recursion(&mut tree_str, &mut indent); 
        tree_str
    } 

    fn to_string_recursion(&self, tree_str: &mut String, indent: &mut usize) {
        /*
        if self.node_type == NodeType::Loop {
            *tree_str += &format!("{}loop\n", ControlFlowTree::indent(*indent));
            *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "{");
            *indent += 1;
            /*
            match self.loop_region.as_ref() {
                Some(loop_region) => loop_region.to_string_recursion(tree_str, indent),
                None => panic!("error"),
            }
            */
            *indent -= 1;
            *tree_str += &format!("{}{}", ControlFlowTree::indent(*indent), "\n}");
        } else */
        if self.node_type == NodeType::Break {
            *tree_str += &format!("{}break;\n", ControlFlowTree::indent(*indent));
        } else if self.node_type == NodeType::Back {
            *tree_str += &format!("{}continue;\n", ControlFlowTree::indent(*indent));
        } else if self.node_type == NodeType::Loop {
            *tree_str += &format!("{}loop\n", ControlFlowTree::indent(*indent));
            *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "{");
            *indent += 1;
            if let Some(loop_region) = self.loop_region.as_ref() {
                loop_region.to_string_recursion(tree_str, indent);
            }
            *indent -= 1;
            *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "}");

            for next in self.next.iter() {
                next.to_string_recursion(tree_str, indent);
            }
        } else { 
            *tree_str += &format!("{}b{}\n", ControlFlowTree::indent(*indent), self.id);
            if let Some(c_id) = self.condiction {
                if let Some(true_next) = &self.true_next {
                    //*tree_str += &format!("{}if(c{}) \n{}{}\n", ControlFlowTree::indent(*indent), isize::abs(c_id), ControlFlowTree::indent(*indent), "{");
                    if c_id < 0 {
                        *tree_str += &format!("{}if(!c{})\n", ControlFlowTree::indent(*indent), isize::abs(c_id));
                    } else {
                        *tree_str += &format!("{}if(c{})\n", ControlFlowTree::indent(*indent), isize::abs(c_id));
                    }
                    *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "{");
                    *indent += 1;
                    true_next.to_string_recursion(tree_str, indent);
                    *indent -= 1;
                    *tree_str += &format!("{}{}\n",ControlFlowTree::indent(*indent), "}");
                    if let Some(false_next) =&self.false_next {
                        *tree_str += &format!("{}else\n", ControlFlowTree::indent(*indent));
                        *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "{");
                        *indent += 1;
                        false_next.to_string_recursion(tree_str, indent);
                        *indent -= 1;
                        *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "}");
                    }
                } else if let Some(false_next) = &self.false_next {
                    //*tree_str += &format!("if(!c{}) \n{}{}\n", -c_id, ControlFlowTree::indent(*indent), "{");
                    if c_id < 0 {
                        *tree_str += &format!("{}if(!c{})\n", ControlFlowTree::indent(*indent), isize::abs(c_id));
                    } else {
                        *tree_str += &format!("{}if(c{})\n", ControlFlowTree::indent(*indent), isize::abs(c_id));
                    }
                    *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "{");
                    *indent += 1;
                    false_next.to_string_recursion(tree_str, indent);
                    *indent -= 1;
                    *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "}");
                }
            }        

            for next in self.next.iter() {
                if next.is_sink && next.access_condiction.len() != 0 {
                    *tree_str += &format!("{}if(", ControlFlowTree::indent(*indent));
                    for i in 0..next.access_condiction.len() - 1 {
                        let andset = &next.access_condiction[i];
                        if andset.len() > 1 {
                            *tree_str += "(";
                            for j in 0..andset.len() - 1 {
                                if andset[j] < 0 {
                                    *tree_str += &format!("!c{} && ", isize::abs(andset[j]));
                                } else {
                                    *tree_str += &format!("c{} && ", andset[j]);
                                }
                            }
                            
                            if andset[andset.len() - 1] < 0 {
                                *tree_str += &format!("!c{}", isize::abs(andset[andset.len() - 1]));
                            } else {
                                *tree_str += &format!("c{}", andset[andset.len() - 1]);
                            }

                        } else {
                            let cid = next.access_condiction[next.access_condiction.len() - 1][0];
                            if cid < 0 {
                                *tree_str += &format!("!c{}", isize::abs(cid));
                            } else {
                                *tree_str += &format!("c{}", cid);
                            }
                        }

                        *tree_str += ") || ";
                    }

                    let andset = &next.access_condiction[next.access_condiction.len() - 1];
                    if andset.len() > 1 {
                        *tree_str += "(";
                        for j in 0..andset.len() - 1 {
                            if andset[j] < 0 {
                                *tree_str += &format!("!c{} && ", isize::abs(andset[j]));
                            } else {
                                *tree_str += &format!("c{} && ", andset[j]);
                            }
                        }
                            
                        if andset[andset.len() - 1] < 0 {
                            *tree_str += &format!("!c{})", isize::abs(andset[andset.len() - 1]));
                        } else {
                            *tree_str += &format!("c{})", andset[andset.len() - 1]);
                        }

                    } else {
                        let cid = next.access_condiction[next.access_condiction.len() - 1][0];
                        if cid < 0 {
                            *tree_str += &format!("!c{})", isize::abs(cid));
                        } else {
                            *tree_str += &format!("c{})", cid);
                        }
                    }

                    
                    *tree_str += &format!(")\n");
                    *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "{");
                    *indent += 1;
                    next.to_string_recursion(tree_str, indent);
                    *indent -= 1;
                    *tree_str += &format!("{}{}\n", ControlFlowTree::indent(*indent), "}");
                } else {
                    next.to_string_recursion(tree_str, indent);
                }
            }
        }
    }

    fn indent(indet: usize) -> String {
        let mut str = String::new();
        for _ in 0..indet {
            str += "\t";
        }
        str
    }

    pub fn travel_cft_with_ast(&self, ast: &mut abstract_syntax_tree::AbstractSyntaxTree, cfg: &ControlFlowGraph, ast_symbol_map: &mut HashMap<usize, abstract_syntax_tree::ASTSymbol>, address_symbol_map: &mut HashMap<(Address, usize), usize>, counter: &mut Counter, params: &mut Vec<AbstractSyntaxTree>) {
        let bid = self.id;
        let dfi_ir = &cfg.nodes[&bid].irs;

        if SHOW_CONTROL_FLOW_IN_DATA_FLOW_IR.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n-------------------------------block: {}---------------------------", bid);
            for ir in dfi_ir.iter() {
                //println!("{}", ir);
            }
        }

        for ir in dfi_ir.iter() {
            let (ast_ir, ast_param) = AbstractSyntaxTree::parse_dfi(ir, ast_symbol_map, address_symbol_map, counter);
            if let Some(ast_ir) = ast_ir {
                ast.next.push(Box::new(ast_ir));
            } 
            if let Some(ast_param) = ast_param {
                params.push(ast_param);
            }
            //println!("{}", ir);            
            /*
            match AbstractSyntaxTree::parse_dfi(ir, ast_symbol_map, address_symbol_map, counter) {
                (Some(ast_ir_node), )
                /*
                () => {
                    //println!("{:?}", ast_symbol_map);
                    //println!("print from control_flow::travel_cft_with_ast:\n\t{:?}\t{}", ast_ir_node, ir);
                    ast.next.push(Box::new(ast_ir_node));
                }
                None => {}
                */
            }
            */
        }

        


        /*
        if OPTIMIZATION.load(std::sync::atomic::Ordering::SeqCst) {
            optimization(ast);
        }
        */
        //optimization::constant_folding(ast);
        //optimization::constant_propagation(ast);

        if self.node_type == NodeType::Loop {
            let mut loop_ast = AbstractSyntaxTree::new();
            loop_ast.ast_type = ASTType::Loop;
            match self.loop_region.as_ref() {
                Some(loop_region) => loop_region.travel_cft_with_ast(&mut loop_ast, cfg, ast_symbol_map, address_symbol_map, counter, params),
                None => panic!("error"),
            }
            ast.next.push(Box::new(loop_ast));
            for next in self.next.iter() {
                next.travel_cft_with_ast(ast, cfg, ast_symbol_map, address_symbol_map, counter, params);
            }
        } else if self.node_type == NodeType::Back {
            let mut continue_ast = AbstractSyntaxTree::new();
            continue_ast.ast_type = ASTType::Continue;
            ast.next.push(Box::new(continue_ast)); 
        } else if self.node_type == NodeType::Break {
            let mut break_ast = AbstractSyntaxTree::new();
            break_ast.ast_type = ASTType::Break;
            ast.next.push(Box::new(break_ast));
        } else {
            if let Some(c_id) = self.condiction {
                let mut if_ast = AbstractSyntaxTree::new();
                if_ast.ast_type = ASTType::If;
                let condiction = get_key_from_value(&cfg.condiction, c_id).unwrap();
                let cond_ast = AbstractSyntaxTree::parse_condiction(condiction, address_symbol_map, ast_symbol_map, counter);
                if_ast.next.push(Box::new(cond_ast));
                if let Some(true_next) = self.true_next.as_ref() {
                    let mut true_ast = AbstractSyntaxTree::new();
                    true_ast.ast_type = ASTType::True;
                    true_next.travel_cft_with_ast(&mut true_ast, cfg, ast_symbol_map, address_symbol_map, counter, params);
                    if true_ast.next.len() != 0 {
                        if_ast.next.push(Box::new(true_ast));
                    }
                }
                if let Some(false_next) = self.false_next.as_ref() {
                    let mut false_ast = AbstractSyntaxTree::new();
                    false_ast.ast_type = ASTType::False;
                    false_next.travel_cft_with_ast(&mut false_ast, cfg, ast_symbol_map, address_symbol_map, counter, params);
                    if false_ast.next.len() != 0 {
                        if_ast.next.push(Box::new(false_ast));
                    }
                }
                ast.next.push(Box::new(if_ast));
            }


            for next in self.next.iter() {
                if next.access_condiction.len() == 0 {
                    next.travel_cft_with_ast(ast, cfg, ast_symbol_map, address_symbol_map, counter, params);
                } else {
                    let mut if_ast = AbstractSyntaxTree::new();
                    if_ast.ast_type = ASTType::If;
                    let mut access_condiction = Vec::<Vec<Condiction>>::new();
                    for andset in next.access_condiction.iter() {
                        let mut andv = Vec::<Condiction>::new();
                        for cid in andset.iter() {
                            let mut condiction = get_key_from_value(&cfg.condiction, isize::abs(*cid)).unwrap();
                            if *cid < 0 {
                                condiction.relation = !condiction.relation;
                            }
                            andv.push(condiction);
                        }
                        access_condiction.push(andv);
                    }
                    let mut access_ast = AbstractSyntaxTree::parse_access_condiction(access_condiction, address_symbol_map, ast_symbol_map, counter);
                    if_ast.next.push(Box::new(access_ast));
                    let mut true_body_ast = AbstractSyntaxTree::new();
                    true_body_ast.ast_type = ASTType::True;
                    next.travel_cft_with_ast(&mut true_body_ast, cfg, ast_symbol_map, address_symbol_map, counter, params);
                    if_ast.next.push(Box::new(true_body_ast));
                    ast.next.push(Box::new(if_ast));
                }
            }
        }


    }


}

fn new_temp(temps: &mut HashMap<usize, DFISymbolRecord>) -> DFISymbolRecord {
    let mut i = 0;
    loop {
        if temps.iter().any(|t| *t.0 == i) {
            i += 1;
        } else {
            break;
        }
    }

    let new_tmp = DFISymbolRecord {
        address: Address::GR(0),
        sym_type: DFISymbolType::Temp,
        id: i,
        size: Size::Unsigned64,
        value: false,
    };

    temps.insert(i, new_tmp.clone());

    new_tmp
}

fn create_set_ir(temp: &DFISymbolRecord, value: usize) -> DataFlowIr {
    let mut number = Number {
        value: value as i64,
        signed: false,
        size: Size::Unsigned64,
    };

    let mut ir = DataFlowIr {
        address: 0,
        opcode: DataFlowIrOpcode::Add,
        operand1: Some(DFIOperand::Symbol(temp.clone())),
        operand2: Some(DFIOperand::Number(number)),
        operand3: Some(DFIOperand::Number(Number::from(0, false, Size::Unsigned64))),
    };

    ir
}

fn create_condiction(temp: &DFISymbolRecord, value: usize) -> Condiction {
    let number = Number {
        value: value as i64,
        signed: false,
        size: Size::Unsigned64,
    };
    let mut condiction = Condiction {
        relation: Relation::EQ,
        operand1: DFIOperand::Symbol(temp.clone()), 
        operand2: DFIOperand::Number(number),
    };

    condiction
}


fn get_condiction_id(cfg: &mut ControlFlowGraph, condiction: Condiction) -> usize {
    match cfg.condiction.get(&condiction) {
        Some(id) => {
            *id as usize
        }
        None => {
            let mut id = 1;
            loop {
                if cfg.condiction.iter().any(|(_, i)| *i == id) {
                    id += 1;
                } else {
                    break;
                }
            }
            cfg.condiction.insert(condiction, id as isize);
            id as usize
        }
    }
}

pub fn get_cfg_loop_slices(_cfg: &ControlFlowGraph) -> (HashMap<usize, ControlFlowGraph>, HashMap<usize, usize>) {
    let mut loop_slices = HashMap::new();
    let mut slices_map = HashMap::new();
    let mut cfg = _cfg.clone();
    //_cfg.info();
    for (eid, edge) in _cfg.edges.iter() {
        if edge.edge_type == EdgeType::LoopEnter {
            cfg.edges.remove(eid);
            let from = edge.from;
            let to = edge.to;
            loop_slices.insert(to as usize, ControlFlowGraph::new());
            slices_map.insert(from, to as usize);
        } 
    }
    loop_slices.insert(0, ControlFlowGraph::new());
    
    for loop_slice in loop_slices.clone().iter() {
        let start = *loop_slice.0;
        let mut sub_cfg = ControlFlowGraph::new();
       
        let mut travel_node = HashSet::<usize>::new();
        let mut travel_edge = HashSet::<usize>::new();

        cfg_dfs(&cfg, start, &mut travel_node, &mut travel_edge);

        for node in travel_node.iter() {
            sub_cfg.nodes.insert(*node, _cfg.nodes[node].clone());
        }

        for edge in travel_edge.iter() {
            sub_cfg.edges.insert(*edge, _cfg.edges[edge].clone());
        }
        
        sub_cfg.condiction = _cfg.condiction.clone();

        loop_slices.insert(start, sub_cfg);
    }

    (loop_slices, slices_map)
} 

fn cfg_dfs(cfg: &ControlFlowGraph, start: usize,  travel_node: &mut HashSet<usize>, travel_edge: &mut HashSet<usize>) {
    travel_node.insert(start);
    for (eid, edge) in cfg.edges.iter() {
        if start == edge.from {
            let from = edge.from;
            let to = edge.to;
            travel_edge.insert(*eid);
            cfg_dfs(cfg, to as usize, travel_node, travel_edge);
        }
    } 
}





impl std::fmt::Display for ControlFlowTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "{}", self.to_string())
    } 
}
