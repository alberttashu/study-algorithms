use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
    pub rand: Option<Box<Node<T>>>,
}

pub fn create_list(count: usize) -> Option<Box<Node<usize>>> {
    let mut nodes: Vec<Node<usize>> = (0..count)
        .map(|idx| Node {
            value: idx,
            next: None,
            rand: None,
        })
        .collect();

    let mut rand_order: Vec<usize> = (0..count).collect();
    rand_order.shuffle(&mut thread_rng());

    for (seq_idx, rand_idx) in rand_order.iter().enumerate().rev() {
        let next_node = if seq_idx < (count - 1) {
            Some(Box::new(nodes[seq_idx + 1].clone()))
        } else {
            None
        };

        nodes[seq_idx].next = next_node;
        nodes[seq_idx].rand = Some(Box::new(nodes[*rand_idx].clone()));
    }

    Some(Box::new(nodes[0].clone()))
}
