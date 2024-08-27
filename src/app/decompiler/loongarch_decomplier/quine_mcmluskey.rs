//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BooleanState {
    True,
    False,
    None,
}

pub fn quine_mccluskey(f: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut item_set = HashSet::<usize>::new();
    for row in f.iter() {
        for n in row.iter() {
            item_set.insert(isize::abs(*n) as usize);
        }
    }

    let mut max = 0;
    for i in item_set.iter() {
        if *i > max {
            max = *i
        }
    }

    for i in 1..max + 1 {
        item_set.insert(i);
    }


    let mut expression = Vec::<HashMap<usize, BooleanState>>::new();
    for row in f.iter() {
        let mut e = HashMap::<usize, BooleanState>::new();
        for item in item_set.iter() {
            if row.iter().any(|x| *x == *item as isize) {
                e.insert(*item, BooleanState::True);
            } else if row.iter().any(|x| -*x == *item as isize) {
                e.insert(*item, BooleanState::False);
            } else {
                e.insert(*item, BooleanState::None);
            }
        }
        expression.push(e);
    }

    let mut group = HashMap::<usize, Vec<HashMap<usize, BooleanState>>>::new();
    for i in 0..item_set.len() {
        group.insert(i, Vec::new());
    }
    for e in expression {
        let mut i = 0;
        for _e in e.iter() {
            if *_e.1 == BooleanState::True {
                i += 1;
            }
        }
        let g = group.entry(i).or_insert(Vec::new());
        g.push(e);
    }

    let group = quine_mccluskey_simplfy(group);
    let mut dump = HashSet::<Vec<isize>>::new();
    let mut f = Vec::<Vec<isize>>::new();

    for group in group.iter() {
        for expression in group.1.iter() {
            let mut v = Vec::<isize>::new();
            for item in expression.iter() {
                let i = item.0;
                let state = item.1;
                match state {
                    BooleanState::None => {}
                    BooleanState::True => v.push(*i as isize),
                    BooleanState::False => v.push(-(*i as isize)),
                }
            }
            if let None = dump.get(&v) {
                f.push(v.clone());
                dump.insert(v);
            }
        }
    }
    f
}

fn quine_mccluskey_simplfy(mut group: HashMap<usize, Vec<HashMap<usize, BooleanState>>>) -> HashMap<usize, Vec<HashMap<usize, BooleanState>>> {
    let mut flag = false;
    let mut delete = HashSet::<(usize, usize)>::new();
    let mut add = Vec::<HashMap<usize, BooleanState>>::new();
    for i in 0..group.len() - 1 {
        let group1 = &group[&i];
        let group2 = &group[&(i + 1)];
        let mut index1 = 0;
        let mut index2 = 0;
        for (i1, g1) in group1.iter().enumerate() {
            index1 = i1;
            let len = g1.len();
            let mut index = 0;
            for (i2, g2) in group2.iter().enumerate() {
                index2 = i2;
                let mut eq = 0;
                for i in 1..len + 1 {
                    if g1[&i] == g2[&i] {
                        eq += 1;
                    } else {
                        index = i;
                    }
                }
                if eq == len - 1 {
                    flag = true;
                    //println!("{:?} - {:?}, {}", g1, g2, index);
                    //println!("{:?} {:?}", (i, index1), (i + 1, index2));
                    delete.insert((i, index1));
                    delete.insert((i + 1, index2));
                    let mut new = g1.clone();
                    new.insert(index, BooleanState::None);
                    add.push(new);
                }
            }
        }
    }

    let mut _group = HashMap::<usize, Vec<HashMap<usize, BooleanState>>>::new();
    for group in group.iter() {
        let gid = group.0;
        let gv = group.1;
        let _g = _group.entry(*gid).or_insert(Vec::new());
        for i in 0..gv.len() {
            let item = &gv[i];
            if !delete.iter().any(|(g, ind)| *g == *gid && i == *ind ) {
                _g.push(item.clone());
            }
        }
    }

    for new in add {
        let mut index = 0;
        for item in new.iter() {
            if *item.1 == BooleanState::True {
                index += 1;
            }
        }
        let g = _group.entry(index).or_insert(Vec::new());
        g.push(new);
    }


    if flag {
        return quine_mccluskey_simplfy(_group);
    } else {
        return _group;
    }
}

