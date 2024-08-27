//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

#[derive(Debug, Clone)]
pub struct Graph {
    pub root: usize,
    pub vertexs: HashSet<usize>,
    pub edges: HashMap<usize, (usize, usize)>,
    pub childs: HashMap<usize, Vec<usize>>,
    pub parents: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            root: 0,
            vertexs: HashSet::new(),
            edges: HashMap::new(),
            childs: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    fn info(&self) {
        println!("------------ graph info ---------");
        println!("root: {}", self.root);
        println!("nodes: ");
        for v in self.vertexs.iter() {
            print!("{} ", v);
        }
        println!("\nedges: ");
        for (id, (from, to)) in self.edges.iter() {
            println!("{}: {} -> {}", id, from, to);
        }
        println!("--------------------------------");
    }

    fn add_edge(&mut self, (from, to): (usize, usize)) -> usize {
        self.vertexs.insert(from);
        self.vertexs.insert(to);
        
        let child = self.childs.entry(from).or_insert(Vec::new());
        child.push(to);

        let parent = self.parents.entry(to).or_insert(Vec::new());
        parent.push(from);

        let mut id = 0;

        println!("id: {}", id);
        while self.edges.iter().any(|(i, (_, _))| *i == id) {
            id += 1;
            println!("id: {}", id);
        }

        self.edges.insert(id, (from, to));
        
        let (topo, _) = graph_topo_sort(&self);
        self.root = *topo.first().unwrap();

        id
    }

    fn add_node(&mut self) -> usize {
        let mut id = 0;
        while self.vertexs.iter().any(|v| *v == id) {
            id += 1;
        }

        self.vertexs.insert(id);

        id
    }

    fn remove_vertex(&mut self, id: usize) {
        self.vertexs.remove(&id);
        let mut vec = Vec::<usize>::new();
        for (i, (from, to)) in self.edges.iter_mut() {
            if *from == id || *to == id {
                vec.push(*i);
            }
        }

        for v in vec.iter() {
            self.edges.remove(v);
        }

        for v in self.vertexs.clone() {
            let pv = self.parents.entry(v).or_default();
            let mut index: Option<usize> = None;
            for (i, p) in pv.iter().enumerate() {
                if *p == id {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                pv.remove(i);
            }
        }

        for v in self.vertexs.clone() {
            let cv = self.childs.entry(v).or_default();
            let mut index: Option<usize> = None;
            for (i, c) in cv.iter().enumerate() {
                if *c == id {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                cv.remove(i);
            }
        }

        let (topo, _) = graph_topo_sort(&self);
        self.root = *topo.first().unwrap();
    }


    fn remove_edge(&mut self, (from, to): (usize, usize)) {
        let mut index = usize::MAX;
        for (id, (f, t)) in self.edges.iter() {
            if *f == from && *t == to {
                index = *id;
                break;
            }
        }
        if index == usize::MAX {
            return;
        } else {
            self.edges.remove(&index);
        }

        let child = self.childs.entry(from).or_default();
        for (i, v) in child.iter_mut().enumerate() {
            if *v == to {
                child.remove(i);
                break;
            }
        }
        let parent = self.parents.entry(to).or_default();
        for (i, v) in parent.iter_mut().enumerate() {
            if *v == from {
                parent.remove(i);
                break;
            }
        }

        if self.parents[&from].len() == 0 && self.childs[&from].len() == 0 {
            self.vertexs.remove(&from);
        }
        if self.parents[&to].len() == 0 && self.childs[&to].len() == 0 {
            self.vertexs.remove(&to);
        }

        let (topo, _) = graph_topo_sort(&self);
        self.root = *topo.first().unwrap();
    }

    fn update(&mut self) {
        self.parents.clear();
        self.childs.clear();
        for v in self.vertexs.iter() {
            self.parents.insert(*v, Vec::new());
            self.childs.insert(*v, Vec::new());
        }

        for (id, (from, to)) in self.edges.iter() {
            let parent = self.parents.entry(*to).or_default();
            parent.push(*from);

            let child = self.childs.entry(*from).or_default();
            child.push(*to);
        }

        let (topo, _) = graph_topo_sort(&self);
        self.root = *topo.first().unwrap();
    }

    fn redirect(&mut self, from: usize, old_to: usize, new_to: usize) {
        for edge in self.edges.iter_mut() {
            if edge.1.0 == from && edge.1.1 == old_to {
                edge.1.1 = new_to;
                break;
            }
        }
        let parent = self.parents.entry(old_to).or_insert(Vec::new());
        for (i, v) in parent.iter_mut().enumerate() {
            if *v == from {
                parent.remove(i);
                break;
            } 
        }
        
        let parent = self.parents.entry(new_to).or_insert(Vec::new());
        if !parent.iter().any(|v| *v == from) {
            parent.push(from);
        }

        let child = self.childs.entry(from).or_default();
        if !child.iter().any(|v| *v == new_to) {
            child.push(new_to);
        }

        let child = self.childs.entry(from).or_default();
        for (i, v) in child.iter_mut().enumerate() {
            if *v == old_to {
                child.remove(i);
                break;
            }
        }

    }

    fn check_region(&self, start: usize, end: usize, now: usize, target: &mut HashSet<usize>, marked: &mut HashSet<usize>, region_set: &mut HashSet<usize>, finish: &mut bool, topo_map: &HashMap<usize, usize>) {
        if now != start {
            for p in self.parents[&now].iter() {
                if let None = marked.get(p) {
                    target.insert(*p);

                }
            }
        }    

        if now != end {
            for c in self.childs[&now].iter() {
                if let None = marked.get(c) {
                    target.insert(*c);
                }
            }
        }

        marked.insert(now);

        if let Some(_) = target.get(&now) {
            target.remove(&now);
            region_set.insert(now);
            marked.insert(now);
        }

        if target.len() == 0 {
            *finish = true;
            return;
        }

        if now != end {
            for c in self.childs[&now].iter() {
                if topo_map[&end] < topo_map[c] {
                    *finish = true;
                    return;
                }
            }
        }

        if *finish {
            return;
        } else {
            region_set.insert(now);
            marked.insert(now);
        }

        if now != end {
            for c in self.childs[&now].iter() {
                self.check_region(start, end, *c, target, marked, region_set, finish, topo_map);
                if *finish {
                    return;
                }
            }
        }



    }

    fn get_edge_id(&self, (from, to): (usize, usize)) -> Option<usize> {
        for (id, (f, t)) in self.edges.iter() {
            if *f == from && *t == to {
                return Some(*id);
            } 
        } 
        None
    }

}

pub fn get_cycle_paths(cfg: &ControlFlowGraph) -> Vec<Vec<usize>> {
    let graph = simplify(cfg);
    let mut paths = Vec::new();
    let mut path_stack = Vec::<usize>::new();
    let mut instack = HashMap::<usize, bool>::new();
    let mut marked = HashMap::<usize, bool>::new();

    for v in graph.vertexs.iter() {
        instack.insert(*v, false);
        marked.insert(*v, false);
    }
    
    for v in 0..graph.vertexs.len() {
        if !marked[&v] {
            cycle_path_dfs(&graph, v, &mut paths, &mut path_stack, &mut instack, &mut marked);
        }
    } 
      
    paths
}

fn cycle_path_dfs(graph: &Graph, v: usize, paths: &mut Vec<Vec<usize>>, path_stack: &mut Vec<usize>, instack: &mut HashMap<usize, bool>, marked: &mut HashMap<usize, bool>) {
    instack.insert(v, true);
    marked.insert(v, true);
    path_stack.push(v);
    for c in &graph.childs[&v] {
        if !instack[c] {
            cycle_path_dfs(graph, *c, paths, path_stack, instack, marked);
        } else {
            let mut cycle = false;
            let mut path = Vec::<usize>::new();
            for v in path_stack.iter() {
                if *v == *c {
                    cycle = true;
                }
                if cycle {
                    path.push(*v);
                }
            }
            paths.push(path);
        }
    }
    instack.insert(v, false);
    path_stack.pop();
}

fn graph_topo_sort(graph: &Graph) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut topo_index = Vec::<usize>::new();
    let mut _topo_index = Vec::<Vec<usize>>::new();
    
    let mut graph = graph.clone(); 

    while graph.vertexs.len() != 0 {
        //println!("parents: {:?}", graph.parents);
        let mut t = Vec::<usize>::new();
        for v in graph.vertexs.iter() {
            if graph.parents[v].len() == 0 {
                t.push(*v);
                topo_index.push(*v);
            }
        }

        for v in t.iter() {
            graph.vertexs.remove(v);
            
            for p in graph.parents.iter_mut() {
                let mut i = 0;
                while i < p.1.len() {
                    if p.1[i] == *v {
                        p.1.remove(i);
                        break;
                    }
                    i += 1;
                }
            }
        }

        _topo_index.push(t);
    }

    //println!("{:?}\n{:?}", topo_index, _topo_index);
    (topo_index, _topo_index)
}

pub fn topo_sort(cfg: &ControlFlowGraph) -> (Vec<usize>, Vec<Vec<usize>>){
    let mut topo_index = Vec::<usize>::new();
    let mut _topo_index = Vec::<Vec<usize>>::new();
    
    let mut graph = simplify(cfg); 

    //let mut old_topo = topo_index.clone();
    //println!("all :{:?}", graph.vertexs);
    while graph.vertexs.len() != 0 {
        //println!("{:?}", graph.vertexs);
        //println!("{:?}", graph.parents);
        let mut t = Vec::<usize>::new();
        for v in graph.vertexs.iter() {
            if graph.parents[v].len() == 0 {
                t.push(*v);
                topo_index.push(*v);
            }
        }

        for v in t.iter() {
            graph.vertexs.remove(v);
            
            for p in graph.parents.iter_mut() {
                let mut i = 0;
                while i < p.1.len() {
                    if p.1[i] == *v {
                        p.1.remove(i);
                        break;
                    }
                    i += 1;
                }
            }
        }
        /*
        if old_topo == topo_index {
            break;
        } else {
            old_topo = topo_index.clone();
        }
        */

        _topo_index.push(t);

    }

    (topo_index, _topo_index)
}

/*
pub fn get_control_flow_trees(cfg: &ControlFlowGraph) -> ControlFlowTree {
    let graph = simplify(cfg);
    let (_topo_, _topo) = graph_topo_sort(&graph);
    let root = _topo_.first().unwrap();
    let mut tree = ControlFlowTree::new(*root);
    
    tree
}

pub fn dfs_search_path(cfg: &Graph, now: usize, target: usize, result: &mut bool) {
    if now == target {
        *result = true;
    } 
}
*/

pub fn get_control_flow_trees(cfg: &ControlFlowGraph, n: &mut usize, structure_condiction_map: &HashMap<usize, Condiction>) -> ControlFlowTree {
    let mut graph = simplify(cfg);
    let (topo, _topo_) = graph_topo_sort(&graph);
    let mut topo_ = topo.clone();
    graph.root = *topo.first().unwrap();

    /*
    println!("{:?}", topo);
    cfg.info();
    */
    let mut topo_map = HashMap::<usize, usize>::new();
    let mut root = usize::MAX;
    for (i, vec) in _topo_.iter().enumerate() {
        if i == 0 {
            root = vec[0]; 
        }
        for v in vec.iter() {
            topo_map.insert(*v, i);
        }
    }


    let mut regions = HashMap::<usize, (HashSet<usize>, usize)>::new();
    let mut region_replace_map = HashMap::<usize, usize>::new();
    let mut region_map = HashMap::<usize, Graph>::new();
    let mut root_map = HashMap::<usize, usize>::new();
    let mut before_append_map = HashMap::<usize, usize>::new();
    let mut after_append_map = HashMap::<usize, usize>::new();

    let mut pending_graphs = Vec::<Graph>::new();
    region_map.insert(graph.root, graph.clone());

    pending_graphs.push(graph.clone());
    
    while pending_graphs.len() != 0 {
        let mut pending_graph = pending_graphs.pop().unwrap();
        let (topo, _topo_) = graph_topo_sort(&pending_graph);

        let mut topo_map = HashMap::<usize, usize>::new(); 
        let mut root = topo.first().unwrap();
        for (i, vec) in _topo_.iter().enumerate() {
            for v in vec.iter() {
                topo_map.insert(*v, i);
            }
        }

        let mut flag = false;
        //println!("{:?}", topo);
        for from in topo.iter() {
            for to in topo.iter() {
                /*
                if pending_graph.parents[to].len() < 2 || pending_graph.childs[from].len() != 2 {
                    continue;
                }
                */
                let mut target = HashSet::<usize>::new();
                let mut marked = HashSet::<usize>::new();
                let mut region = HashSet::<usize>::new();
                let mut region_edges = HashMap::<usize, (usize, usize)>::new();
                let mut finish = false;

                target.insert(*to);

                pending_graph.check_region(*from, *to, *from, &mut target, &mut marked, &mut region, &mut finish, &topo_map);

                 
                if target.len() == 0 && region.len() > 2 && region != pending_graph.vertexs {
                    //println!("-----------------------------------------------");
                    let pre_pending_root = pending_graph.root;
                    if pending_graph.childs[to].len() < 3 {
                        let mut sub_graph = Graph {
                            root: *from,
                            vertexs: region.clone(),
                            edges: HashMap::new(),
                            childs: HashMap::new(),
                            parents: HashMap::new(),
                        };

                        for v in region.iter() {
                            sub_graph.childs.insert(*v, Vec::new());
                            sub_graph.parents.insert(*v, Vec::new());

                            for p in pending_graph.parents[v].iter() {
                                if sub_graph.vertexs.iter().any(|v| *v == *p) {
                                    let parent = sub_graph.parents.entry(*v).or_default();
                                    parent.push(*p);

                                    let id = pending_graph.get_edge_id((*p, *v)).unwrap();
                                    sub_graph.edges.insert(id, (*p, *v));
                                } 

                            }

                            for c in pending_graph.childs[v].iter() {
                                if sub_graph.vertexs.iter().any(|v| *v == *c) {
                                    let child = sub_graph.childs.entry(*v).or_default();
                                    child.push(*c);

                                    let id = pending_graph.get_edge_id((*v, *c)).unwrap();
                                    sub_graph.edges.insert(id, (*v, *c));
                                } 


                            }
                        }

                        let (topo, _) = graph_topo_sort(&sub_graph);
                        sub_graph.root = *topo.first().unwrap();

                        *n += 1;
                        let nid = *n;
                        pending_graph.vertexs.insert(nid);

                        let mut index = 0;
                        let mut n = 0;
                        for (i, v) in topo_.iter().enumerate() {
                            if *v == *from {
                                index = i;
                                n += 1;
                            } else if *v == *to {
                                n += 1;
                                break;
                            } else if i > index {
                                n += 1;
                            }
                        }

                        
                        for v in pending_graph.parents[from].iter() {
                            let id = pending_graph.get_edge_id((*v, *from)).unwrap();
                            pending_graph.edges.insert(id, (*v, nid));
                        } 
                        
                        for v in pending_graph.childs[to].iter() {
                            let id = pending_graph.get_edge_id((*to, *v)).unwrap();
                            pending_graph.edges.insert(id, (nid, *v));
                        }

                        pending_graph.vertexs.insert(nid);
                        for v in sub_graph.vertexs.iter() {
                            pending_graph.remove_vertex(*v);
                        }

                        pending_graph.update();
                        sub_graph.update();
                        /*
                        println!("pending_graph: {:?}", pending_graph);
                        println!("sub_graph: {:?}", sub_graph);
                        println!("pre_pending_root: {}", pre_pending_root);
                        println!("pending_graph.root: {}", pending_graph.root);

                        println!("to: {}", to);
                        println!("nid: {}", nid);
                        */
                        if pre_pending_root != pending_graph.root {
                            before_append_map.insert(*to, pending_graph.root);
                        } else {
                            after_append_map.insert(*from, nid);
                        }
                        //append_map.insert(nid, sub_graph.root);

                        root_map.insert(nid, pending_graph.root);
                        region_replace_map.insert(nid, sub_graph.root);
                        pending_graphs.push(pending_graph.clone());
                        pending_graphs.push(sub_graph.clone());
                        region_map.insert(pending_graph.root, pending_graph.clone());
                        region_map.insert(sub_graph.root, sub_graph);

                        flag = true;

                    } else {
                        let mut sub_graph = Graph {
                            root: *from,
                            vertexs: region.clone(),
                            edges: HashMap::new(),
                            childs: HashMap::new(),
                            parents: HashMap::new(),
                        };

                        for v in region.iter() {
                            sub_graph.childs.insert(*v, Vec::new());
                            sub_graph.parents.insert(*v, Vec::new());

                            for p in pending_graph.parents[v].iter() {
                                if sub_graph.vertexs.iter().any(|v| *v == *p) {
                                    let parent = sub_graph.parents.entry(*v).or_default();
                                    parent.push(*p);

                                    let id = pending_graph.get_edge_id((*p, *v)).unwrap();
                                    sub_graph.edges.insert(id, (*p, *v));
                                } 

                            }

                            for c in pending_graph.childs[v].iter() {
                                if sub_graph.vertexs.iter().any(|v| *v == *c) {
                                    let child = sub_graph.childs.entry(*v).or_default();
                                    child.push(*c);

                                    let id = pending_graph.get_edge_id((*v, *c)).unwrap();
                                    sub_graph.edges.insert(id, (*v, *c));
                                } 


                            }
                        }

                        let (topo, _) = graph_topo_sort(&sub_graph);
                        sub_graph.root = *topo.first().unwrap();

                        *n += 1;
                        let nid1 = *n;
                        pending_graph.vertexs.insert(nid1);
                        pending_graph.parents.insert(nid1, Vec::new());
                        pending_graph.childs.insert(nid1, Vec::new());
                        *n += 1;
                        let nid2 = *n;
                        pending_graph.vertexs.insert(nid2);
                        pending_graph.parents.insert(nid2, Vec::new());
                        pending_graph.childs.insert(nid2, Vec::new());

                        pending_graph.add_edge((nid1, nid2));

                        
                        for v in pending_graph.parents[from].iter() {
                            let id = pending_graph.get_edge_id((*v, *from)).unwrap();
                            pending_graph.edges.insert(id, (*v, nid1));
                        } 
                        
                        for v in pending_graph.childs[to].iter() {
                            let id = pending_graph.get_edge_id((*to, *v)).unwrap();
                            pending_graph.edges.insert(id, (nid2, *v));
                        }

                        for v in sub_graph.vertexs.iter() {
                            pending_graph.remove_vertex(*v);
                        }

                        pending_graph.update();
                        sub_graph.update();
                        //println!("pending_graph: {:?}", pending_graph);
                        //println!("sub_graph: {:?}", sub_graph);

                        root_map.insert(nid1, pending_graph.root);
                        root_map.insert(sub_graph.root, sub_graph.root);
                        region_replace_map.insert(nid1, sub_graph.root);
                        pending_graphs.push(pending_graph.clone());
                        pending_graphs.push(sub_graph.clone());
                        region_map.insert(pending_graph.root, pending_graph.clone());
                        region_map.insert(sub_graph.root, sub_graph);

                        flag = true;
                    }

                }

                if flag {
                    break;
                }
            }

            if flag {
                break;
            }

        }
        
    }

    /*
    println!("region_replace_map: {:?}", region_replace_map);
    println!("region_map: {:?}", region_map);
    println!("root_map: {:?}", root_map);
    */

    /*
    for (r, graph) in region_map.iter() {
        println!("{:?}", graph);
    }
    */

    let mut cfg_map = HashMap::<usize, ControlFlowGraph>::new();
    for (root, graph) in region_map.iter() {
        cfg_map.insert(*root, get_cfg_from_graph(graph, cfg));
    }
    //println!("{:?}", region_replace_map);

    /*
    for (_, cfg) in cfg_map.iter() {
        println!("------------------------------------");
        println!("cfg nodes: ");
        for (n, _) in cfg.nodes.iter() {
            print!("{} ", n);
        }
        println!("\n edges: ");
        for (_, edge) in cfg.edges.iter() {
            println!("{} -> {}", edge.from, edge.to);
        }
    }
    */

    let mut cft_map = HashMap::<usize, ControlFlowTree>::new();
    for (root, region_cfg) in cfg_map.iter() {
        let cft = get_cft_from_region(*root, region_cfg, structure_condiction_map);
        /*
        if *root == 5 {
            println!("print from graph.rs");
            println!("------------------\n{:#?}\n-----------------------\n\n\n", cft);
        }
        */
        //println!("{:#?}", cft);
        cft_map.insert(*root, cft);
    }

    let mut region_replace_map_reverse = HashMap::<usize, usize>::new();
    for (from, to) in region_replace_map.iter() {
        region_replace_map_reverse.insert(*to, *from);
    }
    //println!("region_replace_map_reverse: {:?}", region_replace_map_reverse);
    
    
    let mut cft = cft_map[&root].clone();
    /*
    for (root, tree) in cft_map.iter() {
        println!("root: {}", root);
        println!("{:#?}", tree);
        println!("---------------------");
    }
    */
    /*
    println!("topo: {:?}", topo);
    println!("_topo: {:?}", topo_);
    println!("root: {}", root);
    print!("cft_map: ");
    for (root, _) in cft_map.iter() {
        print!("{} ", root);
    }
    println!("");
    println!("ast root: {}", cft.id);
    println!("append_map: {:?}", append_map);
    */
    
    //println!("append_map: {:?}", append_map);
    for v in topo.iter() {
        /*
        println!("v: {}", v);
        println!("{:?}", append_map);
        */
        if let Some(sub_tree_root) = before_append_map.get(v) {
            //println!("sub_tree_root: {}", sub_tree_root);
            cft.attach_tree(*v, &cft_map[sub_tree_root], &None);
        }

        if let Some(id) = after_append_map.get(v) {
            let tree = &cft_map[v];
            cft.attach_tree(*id, tree, &Some(tree.id));
        }
    }

    //println!("{:#?}", cft);

    /*
    for v in topo_.iter() {
        println!("v: {}", v);
        if let None = region_replace_map.get(v) {
            continue;
        }
        if let Some(sub_cft) = cft_map.get(&region_replace_map[v]) {
            //println!("{}", v);
            //println!("region_replace_map_reverse: {:?}", region_replace_map_reverse);
            //println!("region_replace_map: {:?}", region_replace_map);
            cft.replace(*v, sub_cft);
        }
    }
    */

    //println!("print from graph.rs:\n{:#?}", cft);
    cft
}

fn get_cft_from_region(root: usize, cfg: &ControlFlowGraph, structure_condiction_map: &HashMap<usize, Condiction>) -> ControlFlowTree {
    let mut cft = ControlFlowTree::new(root);
    let mut graph = simplify(cfg);
    graph.update();
    let root_node = &cfg.nodes[&root];
    cft.node_type = root_node.node_type.clone();
    
    let (topo, _) = topo_sort(cfg);
    let mut marked = HashSet::<usize>::new(); 
    for from in topo.iter() {
        if *from == *topo.last().unwrap() && *from != *topo.first().unwrap() {
            if let None = marked.get(from) {
                let node = &cfg.nodes[from];
                cft.attach(root, *from, &node.node_type);
            }
        } else {
            for (eid, edge) in cfg.edges.iter() {
                if edge.from == *from {
                    let to = edge.to as usize;
                    let to_node = &cfg.nodes[&to];
                    if graph.parents[&to].len() == 1 {
                        marked.insert(to);
                        match edge.condiction.as_ref() {
                            Some(condiction) => {
                                let cond_id = isize::abs(cfg.condiction[condiction]);
                                if let Some(true) = edge._true {
                                    cft.insert(*from, to, Branch::True, Some(cond_id), &to_node.node_type); 
                                } else if let Some(false) = edge._true {
                                    cft.insert(*from, to, Branch::False, Some(cond_id), &to_node.node_type); 
                                } else {
                                    panic!("error");
                                }
                            }
                            None => {
                                cft.insert(*from, to, Branch::Next, None, &to_node.node_type); 
                            }
                        }
                    } else if let None = marked.get(&to) {
                        if to == *topo.last().unwrap() {
                            continue;
                        }
                        marked.insert(to);
                        let mut access_condiction = Vec::<Vec<isize>>::new();
                        for p in graph.parents[&to].iter() {
                            let mut ac = Vec::<isize>::new();
                            /*
                            println!("{:#?}", cft);
                            println!("from: {}, to: {}", from, to);
                            println!("{} -> {}", p, to);
                            */
                            
                            let tree = match cft.search(*p) {
                                Some(t) => t,
                                None => {
                                    marked.remove(&to);
                                    continue;
                                }
                            };
                            let mut ac = tree.access_condiction.clone();

                            let from = *p;
                            
                            let mut eid = 0;
                            for (id, edge) in graph.edges.iter() {
                                if edge.0 == from && edge.1 == to {
                                    eid = *id;
                                    break;
                                }
                            }

                            let edge = &cfg.edges[&eid];
                            if let Some(condiction) = &edge.condiction {
                                let cond_id = cfg.condiction[condiction];
                                match &edge._true {
                                    Some(true) => {
                                        for ac in ac.iter_mut() {
                                            ac.push(cond_id);
                                        }
                                    }
                                    Some(false) => {
                                        for ac in ac.iter_mut() {
                                            ac.push(-cond_id);
                                        }
                                    }
                                    None => {}
                                }
                            }

                            for ac in ac {
                                access_condiction.push(ac); 
                            }

                        }
                        let mut is_all_structure_condiciton = true;
                        for vec in access_condiction.iter() {
                            for c in vec.iter() {
                                if let None = structure_condiction_map.get(&(isize::abs(*c) as usize)) {
                                    is_all_structure_condiciton = false;
                                    break;
                                }
                            }
                            if !is_all_structure_condiciton {
                                break;
                            }
                        }
                        if is_all_structure_condiciton {
                            //access_condiction = refine_structure_condiciton(access_condiction, structure_condiction_map);
                            let mut ac = Vec::<Vec<isize>>::new();
                            for vec in access_condiction {
                                ac.push(vec![*vec.last().unwrap()]);
                            }
                            access_condiction = ac;
                        }
                        cft.put_outer(to, access_condiction, &to_node.node_type);
                    }
                }
            }            
        }
    }
    cft
}

/*
fn refine_structure_condiciton(ac: Vec<Vec<isize>>, structure_condiction_map: &HashMap<usize, Condiction>) -> Vec<Vec<isize>>{
    println!("{:?}", ac);
    let mut ac_refined = Vec::new();
    let mut map = HashMap::new();
    for ac in ac.iter() {
        map.insert(ac.len(), ac.clone());
    }
    
    let mut ac_sort_map: Vec<(usize, Vec<isize>)> = map.into_iter().collect();
    ac_sort_map.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ac_sort = Vec::new();
    for (i, ac) in ac_sort_map {
        ac_sort.push(ac);
    }
    
    for i in 0..ac_sort.len() - 1 {
        if i == 0 {
            ac_refined.push(ac_sort[0].clone());
        }
        ac_refined.push(vec![*ac_sort[i + 1].last().unwrap()]);
    }

    println!("{:?}", &ac_refined);
    
    ac_refined
}
*/


fn different_set(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    let mut different_set = Vec::new();
     
    different_set
}

fn get_cfg_from_graph(graph: &Graph, cfg: &ControlFlowGraph) -> ControlFlowGraph {
    let (topo, _topo) = graph_topo_sort(graph);
    let mut region_cfg = ControlFlowGraph {
        nodes: HashMap::new(),
        edges: HashMap::new(),
        condiction: cfg.condiction.clone(),
        topo_index: topo,
        _topo_index: _topo,
    };

    for (id, node) in cfg.nodes.iter() {
        if graph.vertexs.iter().any(|v| *v == *id) {
            region_cfg.nodes.insert(*id, node.clone());
        }
    }

    for (id, edge) in cfg.edges.iter() {
        if graph.edges.iter().any(|(i, _)| *i == *id) {
            let mut edge = edge.clone();
            edge.from = graph.edges[id].0;
            edge.to = graph.edges[id].1 as u64;
            region_cfg.edges.insert(*id, edge.clone());
        }
    }

    for (id, (from, to)) in graph.edges.iter() {
        if !region_cfg.edges.iter().any(|(i, _)| *i == *id) {
            let edge = CFGEdge {
                id: *id,
                from: *from,
                to: *to as u64,
                edge_type: EdgeType::None,
                condiction: None,
                _true: None,
            };

            region_cfg.edges.insert(*id, edge);

            if !region_cfg.nodes.iter().any(|(i, _)| *i == *from) {
                let node = CFGNode {
                    id: *from,
                    index: 0,
                    node_type: NodeType::None,
                    irs: Vec::new(),
                };
                region_cfg.nodes.insert(node.id, node);
            }

            if !region_cfg.nodes.iter().any(|(i, _)| *i == *to) {
                let node = CFGNode {
                    id: *to,
                    index: 0,
                    node_type: NodeType::None,
                    irs: Vec::new(),
                };
                region_cfg.nodes.insert(node.id, node);
            }

        }
    }

    for v in graph.vertexs.iter() {
        if !region_cfg.nodes.iter().any(|(i, _)| *i == *v) {
            let node = CFGNode {
                id: *v,
                index: 0,
                node_type: NodeType::None,
                irs: Vec::new(),
            };
            region_cfg.nodes.insert(node.id, node);           
        }
    }
    
    /*
    for id in graph.vertexs.iter() {
        if !region_cfg.nodes.iter().any(|(i, _)| *i == *id) {
            let node = CFGNode {
                id: region_cfg.new_node_id(),
                index: 0,
                node_type: NodeType::None,
                irs: Vec::new(),
            };

            region_cfg.nodes.insert(node.id, node);
        }
    }
    */

    region_cfg
}




/*
pub fn get_control_flow_trees(cfg: &ControlFlowGraph) -> ControlFlowTree {
    let graph = simplify(cfg);
    /*
    for v in graph.vertexs.iter() {
        print!("{} ", v);
    }
    println!("");
    for e in graph.edges.iter() {
        println!("{} -> {}", e.1.0, e.1.1);
    }
    println!("{:?}", graph.parents);
    println!("{:?}", graph.childs);
    */

    let (_topo_, _topo) = graph_topo_sort(&graph);

    let mut topo = _topo_.clone();
    topo.reverse();
    let root = topo.pop().unwrap();
    topo.reverse();

    let (slices, slice_parents, slice_childs, belong) = get_graph_slices(&graph);
    

    /*
    println!("\n---------------------------------");
    for slice in slices {
        println!("vertex: {:?}\n", slice.vertexs);
        for v in slice.vertexs.iter() {
            let node = &cfg.nodes[v];
            println!("{}: {:?}", v, node.node_type);
        }
        for edge in slice.edges {
            println!("{} -> {}", edge.1.0, edge.1.1);
        }
    }
    */


    let mut tree = ControlFlowTree::new(root);
    let mut put_outer_vertex_map = HashMap::<usize, bool>::new();

    println!("print from graph.rs");
    println!("{:#?}", graph);
    for vertex in topo {
        put_outer_vertex_map.insert(vertex, false);
        let node = &cfg.nodes[&vertex];
        if graph.parents[&vertex].len() == 1 {
            for p in graph.parents[&vertex].iter() {
                let from = *p;
                let to = vertex;
                for e in graph.edges.iter() {
                    let eid = e.0;
                    let e_from = e.1.0;
                    let e_to = e.1.1;
                    if e_from == from && e_to == to {
                        let edge = &cfg.edges[eid]; 
                        let node = &cfg.nodes[&to];
                        if let Some(condiction) = &cfg.edges[&eid].condiction {
                            let cid = cfg.condiction[condiction];
                            let branch = match &cfg.edges[&eid]._true {
                                Some(true) => Branch::True,
                                Some(false) => Branch::False,
                                None => Branch::Next,
                            };
                            tree.insert(from, to, branch, Some(cid), &node.node_type);
                        } else if let Some(is_true) = &cfg.edges[&eid]._true {
                            panic!("");
                            println!("edge id: {} bool: {}", &eid, is_true);
                        } else {
                            tree.insert(from, to, Branch::Next, None, &node.node_type);
                        }
                    }
                }
            }
        } else {
            //println!("v: {}", vertex);
            let mut marked = HashSet::<usize>::new();
            let mut queue = Vec::<usize>::new();
            let mut select = HashSet::<usize>::new();
            let mut finish = false;
            let mut slice_mode = true;
            let mut roots = HashSet::<usize>::new();
            for (_, v2) in belong.iter() {
                roots.insert(*v2);
            }
            select.insert(vertex);
            queue.push(vertex);
            marked.insert(vertex);
            //println!("{}: ", vertex);
            while queue.len() != 0 {
                //println!("{:?}", queue);
                queue.reverse();
                let v = queue.pop().unwrap();
                queue.reverse();
                if slice_mode {
                    for p in graph.parents[&v].iter() {
                        let slice_root = belong[p];
                        let mut flag = true;
                        for sc in slice_childs[&slice_root].iter() {
                            if select.iter().any(|v| *v == *sc) {
                                //println!("sc:{}", sc);
                            } else {
                                queue.push(v);
                                //println!("change mode sc: {}", sc);
                                //println!("chang mode queue: {:?}", queue);
                                flag = false;
                                slice_mode = false;
                                break;
                            } 
                        }
                        if flag {
                            if let None = marked.get(&slice_root) {
                                select.insert(slice_root);
                                queue.push(slice_root);
                                marked.insert(slice_root);
                            }
                        }
                        if !slice_mode {
                            break;
                        }
                    }
                    if finish {
                        break;
                    }
                } else {
                    //println!("changed mode queue: {:?}", queue);
                    for p in graph.parents[&v].iter() {
                        //println!("parent: {}", p);
                        let mut flag = true;
                        for c in graph.childs[p].iter() {
                            if select.iter().any(|v| *v == *c) {
                                     
                            } else {
                                flag = false;
                                break; 
                            }
                        }
                        if flag {
                            if let None = marked.get(&p) {
                                if vertex == 9 {
                                    println!("p is {}", p);
                                    println!("{:?}", put_outer_vertex_map);
                                }
                                println!("{}", put_outer_vertex_map[p]);
                                if !put_outer_vertex_map[p] {
                                    select.insert(*p);
                                    queue.push(*p);
                                    marked.insert(*p);
                                } else {
                                    panic!("no");
                                }
                            }
                        }
                    }
                }
            }

            let mut select_sort = Vec::<usize>::new();
            for t in _topo_.iter() {
                if select.iter().any(|v| *v == *t) {
                    select_sort.push(t.clone());
                } 
            }
            select_sort.reverse();

            //println!("\nsort: {:?}\n", select_sort);

            //println!("{:?}", _topo);
            let mut topo_index = HashMap::<usize, usize>::new();
            for v in select_sort.iter() {
                for i in 0.._topo.len() {
                    for t in _topo[i].iter() {
                        if *t == *v {
                            topo_index.insert(*v, i);
                            break;
                        }
                    }
                }
            }

            println!("{:?}", select_sort);
            let mut node = select_sort.pop().unwrap();
            let mut index = usize::MAX;
            //println!("{}", select_sort.len());
            //println!("{:?}", select_sort);
            println!("{}", node);
            println!("{:?}", put_outer_vertex_map);
            while select_sort.len() > 0 {
                if select_sort.len() == 1 && !roots.iter().any(|r| *r == node) {
                    node = select_sort.pop().unwrap();
                    break;
                }
                if index == topo_index[&node] {
                    node = select_sort.pop().unwrap();
                    continue;
                }
                let mut flag = true;
                for v in select_sort.iter() {
                    let mut result = false;
                    graph_dfs_search(&graph, node, *v, &mut result);
                    //println!("from {} search {}: {}", node, v, result);
                    if !result {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    break;
                } else {
                    index = topo_index[&node];
                    node = select_sort.pop().unwrap();
                }
                /*
                if select_sort.len() == 2 {
                    select_sort.pop();
                    node = select_sort.pop().unwrap();
                    break;
                }
                */

            }


            if vertex == 9 {
                panic!("{}", node);
            }

           // println!("{}", node);

            //println!("vertex: {}", vertex);
            //println!("node: {}", node);
            if node == vertex {
                put_outer_vertex_map.insert(vertex, true);
                if vertex == 9 {
                    println!("{:?}",put_outer_vertex_map);
                    panic!("");
                }
                //tree.insert(0, vertex, Branch::Next, None);
                let mut access_condiction = Vec::<Vec<isize>>::new();
                for p in graph.parents[&vertex].iter() {
                    //println!("{:#?}", tree);
                    let t = tree.search(*p).unwrap();
                    let mut ac = t.access_condiction.clone();
                    
                    let from = *p;
                    let to = vertex;

                    let mut e_id = 0;
                    for e in graph.edges.iter() {
                        if e.1.0 == from && e.1.1 == to {
                            e_id = *e.0;
                            break;
                        }
                    }

                    let edge = &cfg.edges[&e_id];
                    if let Some(condiction) = &edge.condiction {
                        let cid = cfg.condiction[condiction];
                        match &edge._true {
                            Some(true) => {
                                for ac in ac.iter_mut() {
                                    ac.push(cid);
                                }
                            }
                            Some(false) => {
                                for ac in ac.iter_mut() {
                                    ac.push(-cid)
                                }
                            }
                            None => {}
                        }
                    }

                    for ac in ac {
                        access_condiction.push(ac);
                    }

                }

                let node = &cfg.nodes[&vertex];
                tree.put_outer(vertex, access_condiction, &node.node_type);
            } else {
                //tree.insert(node, vertex, Branch::Next, None);
                let n = &cfg.nodes[&vertex];
                tree.attach(node, vertex, &n.node_type);
            }

        }
    }

    //println!("{:#?}", tree);
    tree

}
*/


/*
pub fn get_control_flow_trees(cfg: &ControlFlowGraph) -> Vec<ControlFlowTree> {
    let mut control_flow_trees = Vec::new();
    let mut graph = simplify(cfg);
    
    let (slices, parents, childs, belong) = get_graph_slices(&graph);
    //println!("{:#?}", slices);
    
    let heads_index = build_control_flow_tree(&slices, &mut control_flow_trees, cfg, &parents, &childs);

    let trees = control_flow_trees.clone();
    for tree in control_flow_trees.iter_mut() {
        let to = tree.id;
        for e in graph.edges.iter() {
            let (_from, _to) = e.1;
            if to == *_to {
                let root = belong[_from]; 
                let index = heads_index[&root];
                match trees[index].search(*_from) {
                    Some(t) => {
                        //println!("{:?}", t);
                        if let Some(v) = t.access_condiction.get(0) {
                            tree.access_condiction.push(v.clone());
                        }

                    }
                    None => {
                        panic!("{}", _from);
                    }
                }
            }
        }
    }
    //println!("{:#?}", control_flow_trees);
    
    control_flow_trees
}

fn build_control_flow_tree(slices: &Vec<Graph>, control_flow_trees: &mut Vec<ControlFlowTree>, cfg: &ControlFlowGraph, parents: &HashMap<usize, Vec<usize>>, childs: &HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
    for slice in slices.iter() {
        let root = slice.root;
        let mut tree = ControlFlowTree::new(root);
        let mut stack = Vec::<usize>::new();
        stack.push(root);

        while !stack.is_empty() {
            //println!("{:?}", stack);
            stack.reverse();
            let v = stack.pop().unwrap();
            stack.reverse();
            let mut edge = Vec::<(usize, usize)>::new();
            for c in slice.childs[&v].iter() {
                stack.push(*c);
                edge.push((v, *c));
            }
    
            for edge in edge {
                let from = edge.0;
                let to = edge.1;
            
                for e in cfg.edges.iter() {
                    let edge = e.1;
                    if from == edge.from && to == edge.to {
                        if let None = &edge.condiction {
                            tree.insert(from, to, Branch::Next, None); 
                        } else if let Some(cond) = &edge.condiction {
                            let cond_id = cfg.condiction[cond];
                            if let Some(_true) = &edge._true {
                                if *_true {
                                    tree.insert(from, to, Branch::True, Some(cond_id));
                                } else {
                                    tree.insert(from, to, Branch::False, Some(cond_id));
                                }
                            }
                        }
                    }
                }    

            }
        } 

        //println!("{:#?}", tree);
        control_flow_trees.push(tree);
    }

    let mut heads = Vec::<usize>::new();
    let mut heads_index = HashMap::<usize, usize>::new();
    let mut i = 0;
    for tree in control_flow_trees.iter() {
        heads.push(tree.id);
        heads_index.insert(tree.id, i);
        i += 1;
    }

    
    let mut sink = HashSet::<usize>::new();
    for tree in control_flow_trees.iter_mut() {
        let root = tree.id;
        let mut is_sink = true;
        for parent in parents.iter() {
            //check_sink_childs(root, &heads_index, childs, &mut is_sink);
        }
    }

    heads_index
    
    /*
    for tree in control_flow_trees.iter_mut() {
        let root = tree.id;
        if childs[&root].len() == 1 {
            let mut cid = 0;
            for c in childs[&root].iter() {
                cid = *c;
            }
            if parents[&cid].len() == 1 {
                tree.sink = Some(cid);
                sink.insert(cid);
            }
        }
    }
    */

    //let (topo, _topo) = topo_sort(cfg);
    //println!("{:?}", _topo);

    /*
    println!("{:?}", parents);
    println!("{:?}", childs);
    println!("{:?}", heads);
    */

    //println!("{:#?}", control_flow_trees);
}

fn check_sink_childs(root: usize, heads_index: &HashMap<usize, usize>, childs: &HashMap<usize, Vec<usize>>, is_sink: &mut bool) {
       
}

*/
fn get_graph_slices(_graph: &Graph) -> (Vec<Graph>, HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>, HashMap<usize, usize>) {
    let mut graph = _graph.clone();
    let mut slices = Vec::<Graph>::new();

    for pv in _graph.parents.iter() {
        if pv.1.len() > 1 {
            let to = pv.0;
            let parents = pv.1.clone();

            graph.parents.insert(to.clone(), Vec::new());
            
            for from in parents.iter() {
                let edge = (from.clone(), to.clone());   
                let i = get_key_from_value(&graph.edges, edge).unwrap();
                graph.edges.remove(&i);

                let child = graph.childs.entry(from.clone()).or_insert(Vec::new());
                if let Some(i) = get_index_at_vec(&child, to.clone()) {
                    child.remove(i);
                }
            }
        } 
    }

    let mut marked = HashMap::<usize, bool>::new();
    for v in _graph.vertexs.iter() {
        marked.insert(*v, false);
    }

    let (topo, _topo) = graph_topo_sort(&graph);
    let _heads = _topo[0].clone();
    let (topo, _topo) = graph_topo_sort(_graph);

    let mut heads = Vec::<usize>::new(); 
    for t in topo {
        if _heads.iter().any(|v| *v == t) {
            heads.push(t);
        }
    }


    /*
    for head in heads.iter() {
        let mut path = Vec::new();
        graph_dfs(_graph, *head, &mut path);
        for h in heads.iter() {
            if *h != *head {
                if path.iter().any(|v| *v == *h) {
                    let child = childs.entry(*head).or_insert(HashSet::new());
                    child.insert(*h);
                    let parent = parents.entry(*h).or_insert(HashSet::new());
                    parent.insert(*head);
                }
            }
        }
    }

    for head in heads.iter() {
        childs.entry(*head).or_insert(HashSet::new());
        parents.entry(*head).or_insert(HashSet::new());
    }
    */

    for head in heads {
        let mut slice = Graph::new();
        slice.root = head;
        build_slice(&mut slice, &graph); 
        //println!("{:#?}", slice);

        slices.push(slice);
    }

    let mut belong = HashMap::<usize, usize>::new();
    for vectex in _graph.vertexs.iter() {
        for slice in slices.iter() {
            let root = slice.root;
            if slice.vertexs.iter().any(|v| *v == *vectex) {
                belong.insert(*vectex, root);    
            }
        }
    }

    let mut _childs = HashMap::<usize, HashSet<usize>>::new();
    let mut _parents = HashMap::<usize, HashSet<usize>>::new();

    let mut childs = HashMap::<usize, Vec<usize>>::new();
    let mut parents = HashMap::<usize, Vec<usize>>::new();

    for slice in slices.iter() {
        let cs = &_graph.childs;
        let ps = &_graph.parents;
        let root = slice.root;
        let child = _childs.entry(root).or_insert(HashSet::new());
        let parent = _parents.entry(root).or_insert(HashSet::new());
        childs.entry(root).or_insert(Vec::new());
        parents.entry(root).or_insert(Vec::new());

        for v in slice.vertexs.iter() {
            for c in cs[v].iter() {
                if belong[c] != root {
                    child.insert(belong[c]);
                }
            }

            for p in ps[v].iter() {
                if belong[p] != root {
                    parent.insert(belong[p]);
                }
            }
        }
    }

        
    let (topo, _topo) = graph_topo_sort(_graph);
    for v in topo.iter() {
        for (k, set) in _childs.iter() {
            if set.iter().any(|x| *x == *v) {
                let mut child = childs.entry(*k).or_insert(Vec::new());
                child.push(*v);
            }  
        }
        for (k, set) in _parents.iter() {
            if set.iter().any(|x| *x == *v) {
                let mut parent = parents.entry(*k).or_insert(Vec::new());
                parent.push(*v);
            }  
        }
    }

    /*
    println!("-----------------------");
    println!("{:?}", _parents);
    println!("{:?}", parents);
    println!("{:?}", _childs);
    println!("{:?}", childs);
    println!("-----------------------");
    */

    //println!("{:?}", parents);
    //println!("{:?}", childs);
    
    (slices, parents, childs, belong)
}

fn build_slice(slice: &mut Graph, graph: &Graph) {
    let root = slice.root;
    let mut stack = Vec::<usize>::new(); 
    stack.push(root);
    let mut dfs = Vec::<usize>::new();
    graph_dfs(graph, root, &mut dfs);
    
    for v in dfs.iter() {
        slice.vertexs.insert(*v);
        /*
        slice.childs.insert(*v, graph.childs[v].clone());
        slice.parents.insert(*v, graph.parents[v].clone());
        */
        let childs = slice.childs.entry(*v).or_default();
        let parents = slice.parents.entry(*v).or_default();

        if let Some(gc) = graph.childs.get(v) {
            *childs = gc.clone();
        }
        if let Some(gp) = graph.parents.get(v) {
            *parents = gp.clone();
        }

    }

    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        
        let from = v;
        for c in slice.childs[&v].iter() {
            let to = c.clone();
            let e = (from, to);
            let i = get_key_from_value(&graph.edges, e).unwrap();
            slice.edges.insert(i, (from, to));
            stack.push(*c);
        }
    }
}

fn graph_dfs(graph: &Graph, root: usize, dfs: &mut Vec<usize>) {
    dfs.push(root);
    /*
    for c in graph.childs[&root].iter() {
        graph_dfs(graph, *c, dfs);
    }
    */
    if let Some(childs) = graph.childs.get(&root) {
        for c in childs.iter() {
            graph_dfs(graph, *c, dfs);
        } 
    }
}

fn graph_dfs_search(graph: &Graph, root: usize, target: usize, result: &mut bool) {
    if root == target {
        *result = true;
    } else {
        for child in graph.childs[&root].iter() {
            graph_dfs_search(graph, *child, target, result);
            if *result {
                return;
            }
        }
    }
}


pub fn get_entry_exit(cfg: &ControlFlowGraph, _loop: &Vec<usize>) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut entrys = HashMap::<usize, Vec<usize>>::new();
    let mut exits = HashMap::<usize, Vec<usize>>::new();
    let mut paths = get_cycle_paths(cfg);

    //println!("loop: {:?}", _loop);
    for (i, _loop_) in paths.iter_mut().enumerate() {
        if _loop_ == _loop {
            paths.remove(i);
            break;
        }
    }
    
    let mut excluded_entry_vertexs = HashSet::<usize>::new();
    let mut excluded_exit_vertexs = HashSet::<usize>::new();

    let mut other_loop_edge = HashSet::<(usize, usize)>::new();
    for _loop_ in paths.iter() {
        for i in 0.._loop_.len() {
            let from = _loop_[i];
            let to = _loop_[(i + 1) % _loop_.len()];
            other_loop_edge.insert((from, to));
        }
    }


    
    
    let graph = simplify(cfg);
    for v in _loop.iter() {
        for p in graph.parents[v].iter() {
            if !_loop.iter().any(|l| *l == *p) {
                if let None = other_loop_edge.get(&(*p, *v)) {
                    let entry = entrys.entry(*v).or_insert(Vec::new());
                    entry.push(*p);
                }
            }
        }

        for c in graph.childs[v].iter() {
            if !_loop.iter().any(|l| *l == *c) {
                if cfg.nodes[c].node_type != NodeType::Loop {
                    if let None = other_loop_edge.get(&(*v, *c)) {
                        let exit = exits.entry(*c).or_insert(Vec::new());
                        exit.push(*v);
                    }
                }
            }
        }
    }

    

    (entrys, exits)
}



pub fn simplify(cfg: &ControlFlowGraph) -> Graph {
    let mut graph = Graph::new(); 
    for v in cfg.nodes.iter() {
        graph.vertexs.insert(*v.0);
        graph.childs.insert(*v.0, Vec::new());
        graph.parents.insert(*v.0, Vec::new());
    }

    for e in cfg.edges.iter() {
        let id = e.0;
        let from = e.1.from;
        let to = e.1.to as usize;
        graph.edges.insert(*id, (from, to));
        let child = graph.childs.entry(from).or_insert(Vec::new());
        child.push(to);
        let parent = graph.parents.entry(to).or_insert(Vec::new());
        parent.push(from);
    }

    //let (topo, _) = graph_topo_sort(&graph);
    //graph.root = *topo.first().unwrap();
    graph
}

