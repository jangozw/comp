// use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SortValue {
    pub ori: String,
    pub num: f64,
}

impl SortValue {
    pub fn new(ori: String) -> Self {
        let num = ori.parse::<f64>().expect(&*format!("parse {} to f64 failed!", ori.clone()));
        Self {
            ori: ori,
            num: num,
        }
    }
}


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    // let file = File::open(filename)?;
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}

/*
pub fn get_uniq_vec<T: Copy+std::hash::Hash + std::cmp::Eq>(list: Vec<T>) -> Vec<T> {
    let mut map = HashMap::new();
    for (i, v) in list.iter().enumerate() {
        let key = v.clone();
        map.insert(key, i);
    }
    // keep order after uniq by map
    let mut idx = Vec::new();
    for (_, v) in map {
        idx.push(v);
    }
    idx.sort_by(|a, b| {
        a.cmp(b)
    });
    let mut ret:Vec<T> = Vec::new();
    for i in &idx {
        let uv = list.get(*i).unwrap();
        ret.push(uv.clone());
    }
    return ret;
}*/