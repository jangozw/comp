extern crate clap;

use std::collections::btree_set::BTreeSet;
use std::collections::HashMap;

use clap::{Arg, ArgMatches, Command};

use util::{read_lines, SortValue};

mod util;

const CMD_DIFF: &str = "diff";
const CMD_INTER: &str = "inter";
const CMD_UNION: &str = "union";
const CMD_SORT_ASC: &str = "sort-asc";
const CMD_SORT_DESC: &str = "sort-desc";
const CMD_UNIQ: &str = "uniq";


fn main() {
    let args = vec![
        Arg::new("a")
            //.short('a')
            // .long("a")
            .index(1)
            .help("filepath1")
            .takes_value(true)
            .required(true),
        Arg::new("b")
            // .short('b')
            // .long("b")
            .index(2)
            .help("filepath2")
            .takes_value(true)
            .required(true),
        Arg::new("u")
            .short('u')
            .long("uniq")
            .help("optional arg. make unique, remove duplicate elems from result")
            .takes_value(true)
            .required(false),
        Arg::new("s")
            .short('s')
            .long("sort")
            .help("optional arg. sort options: asc, desc. will sort result elems by this flag, note: all elems must enable to covert to number!")
            .takes_value(true)
            .required(false),
    ];

    let sort_args = vec![
        Arg::new("a")
            .index(1) // without arg name, must start with 1
            .help("filepath, all elems must enable to covert to number!")
            .takes_value(true)
            .required(true),
        Arg::new("u")
            .short('u')
            .long("uniq")
            .help("optional arg. bool value, remove duplicate elem")
            .takes_value(true)
            .required(false),
    ];

    let uniq_args = vec![
        Arg::new("a")
            .index(1)
            .help("filepath")
            .takes_value(true)
            .required(true),
        Arg::new("s")
            .short('s')
            .long("sort")
            .help("optional arg. sort options: asc,desc. default none")
            .takes_value(true)
            .required(false),
    ];
    let app = Command::new("comp")
        .version("1.0.1")
        .author("Django")
        .about("Compute intersection,difference,union set between two files, sort or remove duplicate elem from file")
        .subcommands(vec![
            Command::new(CMD_DIFF).args(&args).about("display difference, the result is filepath1 - filepath2"),
            Command::new(CMD_INTER).args(&args).about("display intersection between filepath1 and filepath2"),
            Command::new(CMD_UNION).args(&args).about("display union between filepath1 and filepath2"),
            Command::new(CMD_SORT_ASC).args(&sort_args).about("display sorted file elems by asc"),
            Command::new(CMD_SORT_DESC).args(&sort_args).about("display sorted file elems by desc"),
            Command::new(CMD_UNIQ).args(&uniq_args).about("display unique elems from input file"),
        ]).get_matches();

    match app.subcommand() {
        Some((CMD_DIFF, arg)) => {
            process(CMD_DIFF, arg)
        }
        Some((CMD_INTER, arg)) => {
            process(CMD_INTER, arg)
        }
        Some((CMD_UNION, arg)) => {
            process(CMD_UNION, arg)
        }
        Some((CMD_SORT_ASC, arg)) => {
            sort_uniq_process(CMD_SORT_ASC, arg)
        }
        Some((CMD_SORT_DESC, arg)) => {
            sort_uniq_process(CMD_SORT_DESC, arg)
        }
        Some((CMD_UNIQ, arg)) => {
            sort_uniq_process(CMD_UNIQ, arg)
        }
        _ => { println!("invalid subcommand") }
    }
}

fn process(name: &str, arg: &ArgMatches) {
    // 1. parse args
    let filea = arg.value_of("a").expect("get filepath1 value failed").trim();
    let fileb = arg.value_of("b").expect("get filepath2 value failed").trim();
    let mut sort = "";
    if arg.is_present("s") {
        sort = arg.value_of("s").expect("get sort flag failed").trim();
    }
    let mut uniq = false;
    if arg.is_present("u") {
        uniq = arg.value_of("u").expect("get uniq value failed").trim().parse::<bool>().unwrap();
    }
    // 2. compute
    let mut seta = BTreeSet::new();
    let mut setb = BTreeSet::new();
    if let Ok(lines) = read_lines(filea) {
        lines.for_each(|line| {
            if let Ok(l) = line {
                let  value = l.trim().to_string();
                if value.len() > 0 {
                    seta.insert(value);
                }
            }
        });
    }
    if let Ok(lines) = read_lines(fileb) {
        lines.for_each(|line| {
            if let Ok(l) = line {
                let  value = l.trim().to_string();
                if value.len() > 0 {
                    setb.insert(value);
                }
            }
        });
    }
    match name {
        CMD_DIFF => {
            print_ret(seta.difference(&setb).cloned().collect(), sort, uniq);
        }
        CMD_UNION => {
            print_ret(seta.union(&setb).cloned().collect(), sort, uniq);
        }
        CMD_INTER => {
            print_ret(seta.intersection(&setb).cloned().collect(), sort, uniq);
        }
        _ => {}
    }
}

fn sort_uniq_process(name: &str, arg: &ArgMatches) {
    // 1. parse args
    let filename = arg.value_of("a").expect("get filepath value err");
    let mut sort = "";
    let mut uniq = false;
    match name {
        CMD_SORT_DESC => {
            sort = "desc";
            if arg.is_present("u") {
                uniq = arg.value_of("u").expect("get uniq value failed").trim().parse::<bool>().unwrap();
            }
        }
        CMD_SORT_ASC => {
            sort = "asc";
            if arg.is_present("u") {
                uniq = arg.value_of("u").expect("get uniq value failed").trim().parse::<bool>().unwrap();
            }
        }
        CMD_UNIQ => {
            uniq = true;
            if arg.is_present("s") {
                sort = arg.value_of("s").unwrap();
            }
        }
        _ => {}
    }

    // 2. read file
    let mut list: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                let value = line.trim().to_string();
                if value.len() > 0 {
                    list.push(value);
                }
            }
        });
    }

    // 3. print
    print_ret(list, sort, uniq);
}

fn print_ret(mut ret: Vec<String>, sort: &str, uniq: bool) {
    if uniq {
       // ret = get_uniq_vec(ret);
        ret = get_uniq_list(ret);
    }
    if sort != "" {
        let mut sort_list = Vec::new();
        for (_, v) in ret.iter().enumerate() {
            sort_list.push(SortValue::new(v.clone()));
        }
        match sort {
            "asc" => {
                sort_list.sort_by(|a, b| {
                    a.num.partial_cmp(&b.num).unwrap()
                });
            }
            "desc" => {
                sort_list.sort_by(|a, b| {
                    b.num.partial_cmp(&a.num).unwrap()
                });
            }
            _ => {}
        }
        for x in sort_list {
            println!("{}", x.ori);
        }
        return;
    } else {
        for x in ret {
            println!("{}", x);
        }
    }
}

fn get_uniq_list(list: Vec<String>) -> Vec<String> {
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
    let mut ret:Vec<String> = Vec::new();
    for i in &idx {
        let uv = list.get(*i).unwrap();
        ret.push(uv.clone());
    }
    return ret;
}
