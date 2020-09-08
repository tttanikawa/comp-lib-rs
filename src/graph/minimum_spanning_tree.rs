use crate::data_structure::union_find::UnionFind;

pub struct MinimumSpanningTree<T> {
    n: usize,
    edges: Vec<(usize, usize, T)>, // (i, j, cost)
    selected: Vec<(usize, usize, T)>,
    total_cost: T,
}

impl<T> MinimumSpanningTree<T>
where
    T: Copy
        + num::Zero
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::AddAssign
        + std::cmp::Ord,
{
    /// Create new instance
    pub fn new(n: usize, edges: &[(usize, usize, T)]) -> Self {
        MinimumSpanningTree {
            n,
            edges: edges.to_owned(),
            selected: vec![],
            total_cost: num::Zero::zero(),
        }
    }

    /// Kruskal's algorithm
    /// O(E log E}
    pub fn kruskal(&mut self) {
        let mut uf = UnionFind::new(self.n);
        self.selected = vec![];
        self.total_cost = num::Zero::zero();

        self.edges.sort_by_cached_key(|x| x.2);
        for &(x, y, cost) in self.edges.iter() {
            if !uf.is_same(x, y) {
                uf.unite(x, y);
                self.selected.push((x, y, cost));
                self.total_cost += cost;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kruskal_test() {
        let n = 4;
        let edges = vec![(0, 1, 1), (1, 2, 2), (2, 3, 1), (0, 1, 3), (1, 3, 5)];

        let mut mst = MinimumSpanningTree::new(n, &edges);
        mst.kruskal();

        assert_eq!(mst.selected, vec![(0, 1, 1), (2, 3, 1), (1, 2, 2),]);
        assert_eq!(mst.total_cost, 4);
    }

    /// https://atcoder.jp/contests/abc065/tasks/arc076_b
    #[test]
    fn solve_abc065_d() {
        let n = 6;
        let xy = vec![(8, 3), (4, 9), (12, 19), (18, 1), (13, 5), (7, 6)];

        let mut xyi = xy
            .into_iter()
            .enumerate()
            .map(|(i, (x, y))| (x, y, i))
            .collect::<Vec<(i64, i64, usize)>>();

        let mut edges = vec![];
        xyi.sort();
        for i in 0..(n - 1) {
            let dist = std::cmp::min(
                (xyi[i].0 - xyi[i + 1].0).abs(),
                (xyi[i].1 - xyi[i + 1].1).abs(),
            );
            edges.push((xyi[i].2, xyi[i + 1].2, dist));
        }
        xyi.sort_by_cached_key(|&(_, y, _)| y);
        for i in 0..(n - 1) {
            let dist = std::cmp::min(
                (xyi[i].0 - xyi[i + 1].0).abs(),
                (xyi[i].1 - xyi[i + 1].1).abs(),
            );
            edges.push((xyi[i].2, xyi[i + 1].2, dist));
        }

        let mut mst = MinimumSpanningTree::new(n, &edges);
        mst.kruskal();
        assert_eq!(mst.total_cost, 8);
    }
}
