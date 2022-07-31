use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<String, Vec<String>>;

fn is_big(input: &String) -> bool {
    input.chars().into_iter().all(char::is_uppercase)
}

fn parse_to_graph(lines: Vec<(String, String)>) -> Graph {
    let mut graph = HashMap::new();
    lines.into_iter().for_each(|(left, right)| {
        graph.entry(left.to_owned()).or_insert(HashSet::new()).insert(right.to_owned());
        graph.entry(right).or_insert(HashSet::new()).insert(left);
    });
    graph.into_iter()
        .map(|(key, val)| (key, val.into_iter().collect::<Vec<_>>()))
        .collect()
}

fn paths(graph: &Graph, root: String, to: String, seen: HashSet<String>) -> u16 {
    if root == to {
        return 1;
    }

    graph.get(&root).unwrap().into_iter().map(|node|
        if is_big(node) || !seen.contains(node) {
            let seen = seen.clone().into_iter()
                .chain(HashSet::from([root.to_owned()]))
                .collect();
            paths(graph, node.to_owned(), to.to_owned(), seen)
        } else {
            0
        }
    ).sum()
}

fn update_seen(item: &String, seen_once: &HashSet<String>, seen: &HashSet<String>)
               -> (HashSet<String>, HashSet<String>) {
    if seen_once.contains(item) {
        let seen = seen.clone().into_iter()
            .chain(HashSet::from([item.to_owned()]))
            .collect::<HashSet<String>>();
        return (seen_once.to_owned(), seen.to_owned());
    }
    let seen_once = seen_once.clone().into_iter()
        .chain(HashSet::from([item.to_owned()]))
        .collect::<HashSet<String>>();
    return (seen_once.to_owned(), seen.to_owned());
}

fn paths2(graph: &Graph, root: String, to: String,
          seen: HashSet<String>, can_revisit: bool) -> u32 {
    if root == to {
        return 1;
    }

    graph.get(&root).unwrap().into_iter().map(|node| {
        let contains = seen.contains(node);
        let seen = seen.clone().into_iter()
            .chain(HashSet::from([root.to_owned()]))
            .collect();
        return if is_big(node) || !contains {
            paths2(graph, node.to_owned(), to.to_owned(), seen, can_revisit)
        } else if can_revisit && node != "start" && node != "end" {
            paths2(graph, node.to_owned(), to.to_owned(), seen, false)
        } else {
            0
        };
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Graph {
        let parsed = parse("resources/day12_2.in")
            .into_iter()
            .map(|line| {
                if let [left, right] = line.split("-").collect::<Vec<&str>>().as_slice() {
                    return (left.to_string(), right.to_string());
                }
                panic!("Cannot parse");
            })
            .collect();
        parse_to_graph(parsed)
    }

    #[test]
    fn task01_test() {
        let graph = input_data();
        let mut seen = HashSet::new();
        seen.insert("start".to_string());
        println!("{:?}", graph);
        println!("{:?}", paths(&graph, "start".to_string(), "end".to_string(), seen));
    }

    #[test]
    fn task02_test() {
        let graph = input_data();
        let mut seen = HashSet::new();
        seen.insert("start".to_string());
        println!("{:?}", graph);
        println!("{:?}", paths2(&graph, "start".to_string(), "end".to_string(), seen, true));
    }
}