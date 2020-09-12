use std::cmp;
use std::collections::BinaryHeap;

pub fn warshall_floyd(adj_matrix: &mut Vec<Vec<usize>>) {
    let n = adj_matrix.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                adj_matrix[i][j] = cmp::min(
                    adj_matrix[i][j],
                    adj_matrix[i][k].saturating_add(adj_matrix[k][j]),
                );
            }
        }
    }
}

/// adj_list: Vec<Vec<(to, cost)>>
pub fn dijkstra(adj_list: &[Vec<(usize, usize)>], start: usize, goal: usize) -> Option<usize> {
    let mut dist = vec![std::usize::MAX; adj_list.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push((cmp::Reverse(0), start));
    while let Some((cmp::Reverse(cost), position)) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }

        for &(to, c) in adj_list[position].iter() {
            let next_cost = cost + c;
            if next_cost < dist[to] {
                heap.push((std::cmp::Reverse(next_cost), to));
                dist[to] = next_cost;
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn warshall_floyd_test() {
        let n = 4;
        let abt = vec![
            (1, 2, 1),
            (2, 3, 1),
            (3, 4, 1),
            (4, 1, 1),
            (1, 3, 1),
            (4, 2, 1),
        ];

        let mut adj_matrix = vec![vec![std::usize::MAX; n]; n];
        for (i, row) in adj_matrix.iter_mut().enumerate() {
            row[i] = 0;
        }
        for (a, b, t) in abt.into_iter() {
            adj_matrix[a - 1][b - 1] = t;
            adj_matrix[b - 1][a - 1] = t;
        }

        warshall_floyd(&mut adj_matrix);
        let ans = adj_matrix
            .into_iter()
            .map(|v| v.into_iter().max().unwrap())
            .min()
            .unwrap();
        assert_eq!(ans, 1);
    }

    #[test]
    fn dijkstra_test() {
        let graph = vec![
            vec![(2, 10), (1, 1)],
            vec![(3, 2)],
            vec![(1, 1), (3, 3), (4, 1)],
            vec![(0, 7), (4, 2)],
            vec![],
        ];

        assert_eq!(dijkstra(&graph, 0, 1), Some(1));
        assert_eq!(dijkstra(&graph, 0, 3), Some(3));
        assert_eq!(dijkstra(&graph, 3, 0), Some(7));
        assert_eq!(dijkstra(&graph, 0, 4), Some(5));
        assert_eq!(dijkstra(&graph, 4, 0), None);
    }
}
