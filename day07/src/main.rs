use std::borrow::ToOwned;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::result::Result::{Err, Ok};
use std::string::{String, ToString};

fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);

    let edges: Vec<(String, (u32, String))> = input_buffer.lines().map(
        |rline| {
            parse_edge_set(rline.unwrap())
        }
    ).flatten().collect();

    let reverse_graph: Vec<(String, (u32, String))> = Vec::from_iter(edges.iter().map(
        |(parent, (count, child))| (child.clone(), (count.clone(), parent.clone()))
    ));
    let forward_graph: Vec<(String, (u32, String))> = Vec::from_iter(edges);

    let dn = downstream_nodes("shiny gold".to_string(),
                              &reverse_graph,
                              HashSet::new());

    println!("Parents: {}", dn.len());
    println!("All bags: {}", upstream_nodes("shiny gold".to_string(), &forward_graph));
}

fn parse_edge_set(line: String) -> Vec<(String, (u32, String))> {
    let splot: Vec<&str> = line[0..line.len() - 1].split("contain ").collect();

    let parent = splot[0][0..splot[0].len() - 5 - 1].to_string();
    println!("{}", parent);

    let children: Vec<(u32, String)> = parse_children(splot[1].to_string());

    children.iter().map(|child| (parent.clone(), child.to_owned())).collect()
}

fn parse_children(children: String) -> Vec<(u32, String)> {
    children.split(", ")
        .filter_map(parse_child)
        .collect()
}

fn parse_child(child: &str) -> Option<(u32, String)> {
    let splot: Vec<&str> = child.split(" ").collect();
    match splot[0].parse() {
        Ok(count) => Some((count, splot[1].to_string() + " " + splot[2])),
        Err(_e) => None
    }
}

fn downstream_nodes(node: String,
                    rv_graph: &Vec<(String, (u32, String))>,
                    mut nodes: HashSet<String>) -> HashSet<String> {
    for (_child, (_count, parent)) in rv_graph.iter().filter(|(c, _p)| *c == node) {
        nodes.insert(parent.clone());
        println!("{}", parent);
        nodes = downstream_nodes(parent.clone(), rv_graph, nodes)
    }

    nodes
}

fn upstream_nodes(node: String,
                  fw_graph: &Vec<(String, (u32, String))>) -> u32 {
    let mut count = 0;
    for (_parent, (bag_count, child)) in fw_graph.iter().filter(|(p, _c)| *p == node) {
        count += bag_count * (1 + upstream_nodes(child.clone(), fw_graph))
    }

    count
}
