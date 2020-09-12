//! Union-Find Tree
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    total: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: (0..n).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            total: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parents[x] {
            x
        } else {
            self.parents[x] = self.find(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let (large, small) = if self.sizes[parent_x] < self.sizes[parent_y] {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parents[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.total -= 1;
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        uf.unite(1, 2);
        uf.unite(3, 4);

        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(3), uf.find(4));
        assert_ne!(uf.find(0), uf.find(4));
        assert_eq!(uf.is_same(1, 0), true);
        assert_eq!(uf.is_same(4, 1), false);

        assert_eq!(uf.sizes, vec![3, 0, 0, 2, 0]);
        assert_eq!(uf.total, 2);
    }
}
