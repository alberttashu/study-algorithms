mod leet_code_linked_list;

#[allow(dead_code)]
struct Queue<T> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.len() > 0 {
            return Some(self.items.remove(0));
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
struct Graph {
    nodes: Vec<Vec<usize>>,
}

impl Graph {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[allow(dead_code)]
struct DFSInfo {
    distance: u32,
    ancestor: Option<usize>,
}

#[allow(dead_code)]
fn dfs(graph: Graph, source: usize) -> Vec<DFSInfo> {
    let mut dfs_info = Vec::with_capacity(graph.len());
    for _ in graph.nodes.iter() {
        dfs_info.push(DFSInfo {
            distance: 0,
            ancestor: None,
        });
    }

    let mut queue = Queue::new();
    queue.enqueue(source);

    while let Some(current_idx) = queue.dequeue() {
        let neighbours = &graph.nodes[current_idx];
        for next_idx in neighbours {
            match dfs_info[*next_idx].ancestor {
                None if *next_idx != source => {
                    dfs_info[*next_idx].ancestor = Some(current_idx);
                    dfs_info[*next_idx].distance = dfs_info[current_idx].distance + 1;
                    queue.enqueue(*next_idx);
                }
                _ => continue,
            }
        }
    }

    dfs_info
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leet_code() {
        let head = leet_code_linked_list::create_list(10);
        let mut curr = head;
        println!("TEST RUN -- ");
        loop {
            match curr {
                Some(n) => {
                    println!("{:?}", *n);
                    curr = (*n).next;
                }
                None => break,
            }
        }
    }

    #[test]
    fn it_works() {
        let graph = Graph {
            nodes: vec![
                vec![1],
                vec![0, 4, 5],
                vec![3, 4, 5],
                vec![2, 6],
                vec![1, 2],
                vec![1, 2, 6],
                vec![3, 5],
                vec![],
            ],
        };

        let dfsInfo = dfs(graph, 3);

        for (idx, info) in dfsInfo.iter().enumerate() {
            // println("vertex " + i + ": distance = " + bfsInfo[i].distance + ", predecessor = " + bfsInfo[i].predecessor);
            println!(
                "vertex {}: distance = {} ancestor = {}",
                idx,
                info.distance,
                match info.ancestor {
                    Some(x) => x.to_string(),
                    None => "NULL".to_string(),
                }
            )
        }
    }
}
