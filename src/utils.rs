use std::collections::VecDeque;

pub fn convert_queue_to_vec(queue: VecDeque<String>) -> Vec<String> {
    let mut res = Vec::new();

    for i in 0..queue.len() {
        res.push(queue[i].clone())
    }
    res
}