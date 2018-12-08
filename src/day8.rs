use arrayvec::ArrayVec;
use failure::{bail, err_msg, Error};
use petgraph::prelude::*;
use std::str::FromStr;

struct NodeWeight {
    ch_num: usize,
    md_num: usize,
    md_sum: u32,
    md: ArrayVec<[u8; 16]>,
}

impl NodeWeight {
    pub fn new(ch_num: u8, md_num: u8) -> Self {
        NodeWeight {
            ch_num: ch_num as usize,
            md_num: md_num as usize,
            md_sum: 0,
            md: ArrayVec::new(),
        }
    }

    pub fn ingest_md(&mut self, data: &mut Iterator<Item = u8>) -> Result<(), Error> {
        if self.md_num > 0 {
            if self.md.len() > 0 {
                bail!("metadata alredy assigned to node");
            };
            self.md.extend(data.take(self.md_num));
            if self.md.len() < self.md_num {
                bail!("insufficient metadata values for node");
            }
            self.md_sum = self.md.iter().map(|&v| v as u32).sum();
        }
        Ok(())
    }
}

pub struct Tree {
    graph: Graph<NodeWeight, ()>,
    stack: Vec<NodeIndex>,
    root: Option<NodeIndex>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            graph: Graph::new(),
            stack: Vec::new(),
            root: None,
        }
    }

    pub fn md_sum(&self) -> u32 {
        self.graph
            .node_indices()
            .map(|idx| {
                let weight = self.graph.node_weight(idx).unwrap();
                weight.md_sum
            })
            .sum()
    }

    pub fn root_value(&self) -> Result<u32, Error> {
        let root_idx = self.root.ok_or(err_msg("tree has no root node"))?;
        Ok(self.node_value(root_idx))
    }

    fn node_value(&self, idx: NodeIndex) -> u32 {
        let weight = self.graph.node_weight(idx).unwrap();

        match weight.ch_num {
            0 => weight.md_sum,
            _ => weight.md.iter().map(|&mdval| {
                if mdval == 0 {
                    0
                } else {
                    let mut children: ArrayVec<[NodeIndex; 16]> =
                        self.graph.neighbors_directed(idx, Outgoing).collect();
                    children.reverse();
                    if let Some(cnode_idx) = children.get(mdval as usize - 1) {
                        self.node_value(*cnode_idx)
                    } else {
                        0
                    }
                }
            }).sum(),
        }
    }

    pub fn ingest_data(&mut self, data: &mut impl Iterator<Item = u8>) -> Result<(), Error> {
        let mut data = data.peekable();
        while data.peek().is_some() {
            match self.stack.last() {
                None => {
                    self.ingest_node(&mut data)?;
                }
                Some(idx) => {
                    let child_count = self.graph.edges_directed(*idx, Outgoing).count();
                    let weight = self.graph.node_weight_mut(*idx).unwrap();

                    if child_count < weight.ch_num {
                        self.ingest_node(&mut data)?;
                        continue;
                    }

                    weight.ingest_md(&mut data)?;
                    self.stack.pop();
                }
            }
        }

        Ok(())
    }

    fn ingest_node(&mut self, data: &mut Iterator<Item = u8>) -> Result<(), Error> {
        let ch_num = data.next().ok_or(err_msg("expected missing child count"))?;
        let md_num = data
            .next()
            .ok_or(err_msg("missing metadata record count"))?;

        self.stack
            .push(self.graph.add_node(NodeWeight::new(ch_num, md_num)));

        if self.stack.len() > 1 {
            let pidx = self.stack.get(self.stack.len() - 2).unwrap();
            self.graph.add_edge(*pidx, *self.stack.last().unwrap(), ());
        } else {
            self.root = self.stack.last().cloned();
        }

        Ok(())
    }
}

impl FromStr for Tree {
    type Err = Error;
    fn from_str(s: &str) -> Result<Tree, Self::Err> {
        let mut t = Tree::new();
        let mut data_iter = s.split(' ').map(|v| v.parse::<u8>().unwrap());
        t.ingest_data(&mut data_iter)?;
        Ok(t)
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Result<Box<Tree>, Error> {
    Ok(Box::new(input.parse()?))
}

#[aoc(day8, part1)]
pub fn solve_part1(tree: &Tree) -> Result<u32, Error> {
    Ok(tree.md_sum())
}

#[test]
fn test_part1_sample() {
    let tree = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2").unwrap();
    assert_eq!(solve_part1(&tree).unwrap(), 138);
}

#[aoc(day8, part2)]
pub fn solve_part2(tree: &Tree) -> Result<u32, Error> {
    Ok(tree.root_value()?)
}

#[test]
fn test_part2_sample() {
    let tree = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2").unwrap();
    assert_eq!(solve_part2(&tree).unwrap(), 66);
}
