use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;

pub struct Solution;

impl Solution {
    fn construct_graph(connections: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut graph = HashMap::new();

        for conn in connections {
            graph.entry(conn[0]).or_insert(Vec::new()).push(conn[1]);
            graph.entry(conn[1]).or_insert(Vec::new()).push(conn[0]);
        }

        graph
    }

    fn bfs(start: i32, graph: &HashMap<i32, Vec<i32>>, seen: &mut Vec<bool>, connected_sets: &mut HashMap<i32, Rc<RefCell<BTreeSet<i32>>>>) {
        let mut conn_set = Rc::new(RefCell::new(BTreeSet::new()));
        let mut queue = VecDeque::new();
        queue.push_back(start);
        seen[start as usize] = true;

        while let Some(node) = queue.pop_front() {
            conn_set.borrow_mut().insert(node);
            connected_sets.insert(node, conn_set.clone());

            if let Some(neighs) = graph.get(&node) {
                for n in neighs {
                    if !seen[*n as usize] {
                        queue.push_back(*n);
                        seen[*n as usize] = true;
                    }
                }
            }
        }
    }

    fn construct_grids(c: i32, connections: Vec<Vec<i32>>) -> HashMap<i32, Rc<RefCell<BTreeSet<i32>>>> {
        let graph = Self::construct_graph(connections);

        let mut seen = vec![false; c as usize + 1];
        let mut grids = HashMap::new();

        for node in graph.keys() {
            if !seen[*node as usize] {
                Self::bfs(*node, &graph, &mut seen, &mut grids);
            }
        }

        grids
    }

    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut grids = Self::construct_grids(c, connections);
        let mut results = Vec::with_capacity(queries.len());
        let mut offline = HashSet::new();

        for query in queries {
            let query_type = query[0];
            let node = query[1];

            if query_type == 1 {
                if !offline.contains(&node) {
                    results.push(node);
                } else {
                    if let Some(grid) = grids.get_mut(&node) {
                        if let Some(connected_node) = grid.borrow().iter().next() {
                            results.push(*connected_node);
                        } else {
                            results.push(-1);
                        }
                    } else {
                        results.push(-1);
                    }
                }
            } else {
                if let Some(grid) = grids.get_mut(&node) {
                    grid.borrow_mut().remove(&node);
                }

                offline.insert(node);
            }
        }

        results
    }
}
