use crate::graph::union_find::UnionFind;

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
    pub fn new(n: usize, edges: &Vec<(usize, usize, T)>) -> Self {
        MinimumSpanningTree {
            n,
            edges: edges.clone(),
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
}
