struct Node {
    value: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

trait Tree {
    fn add_node(&mut self, value: usize, parent: Option<usize>) -> Option<usize>;
    fn node(&self, index: Option<usize>) -> &Node;
    fn update_size(&mut self, index: Option<usize>, size: usize);
}

impl Tree for Vec<Node> {
    fn add_node(&mut self, value: usize, parent: Option<usize>) -> Option<usize> {
        let index = self.len();
        if let Some(parent) = parent {
            self[parent].children.push(index);
            self.update_size(Some(parent), value);
        }
        self.push(Node {
            value,
            parent,
            children: vec![],
        });
        Some(index)
    }

    fn node(&self, index: Option<usize>) -> &Node {
        match index {
            Some(index) => &self[index],
            None => unreachable!(),
        }
    }

    fn update_size(&mut self, index: Option<usize>, size: usize) {
        let mut current = index;
        while let Some(index) = current {
            let node = &mut self[index];
            node.value += size;
            current = node.parent;
        }
    }
}

fn main() {
    let mut tree = vec![];
    let mut current: Option<usize> = None;
    let raw = include_str!("../data/day_2022_7.data");
    for line in raw.lines() {
        let splited: Vec<&str> = line.split(' ').collect();
        match splited.first() {
            Some(&"$") => {
                if splited.get(1) == Some(&"cd") {
                    if let Some(&"..") = splited.get(2) {
                        current = tree.node(current).parent;
                    } else {
                        current = tree.add_node(0, current);
                    }
                }
            }
            Some(&"dir") => {}
            _ => {
                tree.add_node(splited[0].parse::<usize>().unwrap(), current);
            }
        }
    }

    let dirs: Vec<_> = tree
        .iter()
        .filter(|node| !node.children.is_empty())
        .collect();

    let part1: usize = dirs
        .iter()
        .filter(|node| node.value <= 100_000)
        .map(|node| node.value)
        .sum();
    println!("Part1 : {part1}");

    let to_delete = 30_000_000 - (70_000_000 - tree[0].value);
    let part2 = dirs
        .iter()
        .filter(|node| node.value >= to_delete)
        .map(|node| node.value)
        .min()
        .unwrap();
    println!("Part2 : {part2}");
}
