use std::collections::VecDeque;

use super::array_2d::Array2D;

pub struct AdjList<T: Copy + Ord> {
    edges: Vec<Vec<Edge<T>>>,
}

struct Edge<T> {
    data: T,
    node: usize,
}

pub struct AdjMatrix<T: Copy + Default + PartialEq> {
    data: Array2D<T>,
}

struct AugmentedVertex {
    seen: bool,
    dist: Option<usize>,
    pred: Option<usize>,
}

impl<T: Copy + Ord> AdjList<T> {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn insert(&mut self, src: usize, dst: usize, data: T) {
        while src.max(dst) >= self.edges.len() {
            self.edges.push(Vec::new());
        }

        self.edges[src].push(Edge { data, node: dst });
        self.edges[dst].push(Edge { data, node: src });
    }

    pub fn list_edges(&self) -> Vec<(usize, usize, T)> {
        let mut list = vec![];

        for (src, edges) in self.edges.iter().enumerate() {
            for edge in edges {
                list.push((src, edge.node, edge.data));
            }
        }

        list.sort();
        list.dedup();

        list
    }

    pub fn breadth_first_search<F>(&self, src: usize, mut cb: F) -> Vec<Option<usize>>
    where
        F: FnMut(usize),
    {
        let mut verts = self.construct_augmented_verts();
        let mut queue = VecDeque::new();

        verts[src].dist = Some(0);

        queue.push_back(src);

        while let Some(vert_idx) = queue.pop_front() {
            cb(vert_idx);

            for edge in &self.edges[vert_idx] {
                if verts[edge.node].seen {
                    continue;
                }

                verts[edge.node].seen = true;

                if let Some(d) = verts[vert_idx].dist {
                    verts[edge.node].dist = Some(d + 1);
                }

                verts[edge.node].pred = Some(vert_idx);

                queue.push_back(edge.node);
            }

            verts[vert_idx].seen = true;
        }

        // predecessor subgraph
        verts.iter().map(|v| v.pred).collect()
    }

    fn construct_augmented_verts(&self) -> Vec<AugmentedVertex> {
        let mut verts = Vec::with_capacity(self.edges.len());

        for _ in 0..self.edges.len() {
            verts.push(AugmentedVertex {
                seen: false,
                dist: None,
                pred: None,
            });
        }

        verts
    }

    pub fn shortest_path(&self, src: usize, mut dst: usize) -> Option<Vec<usize>> {
        let predecessor_subgraph = self.breadth_first_search(src, |_| {});
        let mut nodes = vec![];

        while let Some(pred) = predecessor_subgraph[dst] {
            nodes.push(dst);
            dst = pred;
        }

        if dst == src {
            nodes.push(src);
        } else {
            return None;
        }

        nodes.reverse();

        Some(nodes)
    }

    pub fn bfs_simple(&self, src: usize, mut cb: impl FnMut(usize)) {
        let mut visited = vec![false; self.edges.len()];
        let mut queue = VecDeque::new();

        visited[src] = true;
        queue.push_back(src);

        while let Some(curr) = queue.pop_front() {
            cb(curr);

            for edge in &self.edges[curr] {
                if !visited[edge.node] {
                    visited[edge.node] = true;
                    queue.push_back(edge.node);
                }
            }
        }
    }

    pub fn depth_first_search(&self, src: usize, mut cb: impl FnMut(usize)) {
        let mut visited = vec![false; self.edges.len()];
        let mut stack = Vec::new();

        visited[src] = true;
        stack.push(src);

        while let Some(curr) = stack.pop() {
            cb(curr);

            for edge in &self.edges[curr] {
                if !visited[edge.node] {
                    visited[edge.node] = true;
                    stack.push(edge.node);
                }
            }
        }
    }
}

impl<T: Copy + Ord> Default for AdjList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Default + PartialEq> AdjMatrix<T> {
    pub fn new(num_vertices: usize) -> Self {
        Self {
            data: Array2D::new(T::default(), num_vertices, num_vertices),
        }
    }

    pub fn set(&mut self, src: usize, dst: usize, val: T) {
        self.data[src][dst] = val;
        self.data[dst][src] = val;
    }

    pub fn list_edges(&self) -> Vec<(usize, usize, T)> {
        let mut list = vec![];
        let empty = T::default();

        for y in 0..self.data.height() {
            for x in y..self.data.width() {
                let data = self.data[y][x];

                if data != empty {
                    list.push((y, x, data));
                }
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adj_list_simple() {
        let mut l = AdjList::<()>::new();

        l.insert(0, 1, ());
        l.insert(0, 4, ());
        l.insert(1, 4, ());
        l.insert(1, 3, ());
        l.insert(1, 2, ());
        l.insert(3, 2, ());
        l.insert(3, 4, ());

        let e = [
            (0, 1, ()),
            (0, 4, ()),
            (1, 0, ()),
            (1, 2, ()),
            (1, 3, ()),
            (1, 4, ()),
            (2, 1, ()),
            (2, 3, ()),
            (3, 1, ()),
            (3, 2, ()),
            (3, 4, ()),
            (4, 0, ()),
            (4, 1, ()),
            (4, 3, ()),
        ];

        assert_eq!(&e, l.list_edges().as_slice());
    }

    #[test]
    fn adj_mat_simple() {
        let mut m = AdjMatrix::<u8>::new(5);

        m.set(0, 1, 1);
        m.set(0, 4, 1);
        m.set(1, 4, 1);
        m.set(1, 3, 1);
        m.set(1, 2, 1);
        m.set(3, 2, 1);
        m.set(3, 4, 1);

        let e = [
            (0, 1, 1),
            (0, 4, 1),
            (1, 2, 1),
            (1, 3, 1),
            (1, 4, 1),
            (2, 3, 1),
            (3, 4, 1),
        ];

        assert_eq!(&e, m.list_edges().as_slice());
    }

    #[test]
    fn bfs() {
        let mut l = AdjList::<()>::new();

        l.insert(0, 1, ());
        l.insert(0, 4, ());
        l.insert(1, 4, ());
        l.insert(1, 3, ());
        l.insert(1, 2, ());
        l.insert(3, 2, ());
        l.insert(3, 4, ());

        let mut collection = vec![];

        l.breadth_first_search(2, |i| {
            collection.push(i);
        });

        let expected = [2, 1, 3, 0, 4];

        assert_eq!(&expected, collection.as_slice());
    }

    #[test]
    fn bfs_2() {
        /*
         *  1--2  4--6
         *  |  |/ | /|
         *  0  3--5--7
         *
         */
        let mut l = AdjList::<()>::new();

        l.insert(0, 1, ());
        l.insert(1, 2, ());
        l.insert(2, 3, ());
        l.insert(3, 4, ());
        l.insert(3, 5, ());
        l.insert(4, 5, ());
        l.insert(4, 6, ());
        l.insert(5, 6, ());
        l.insert(5, 7, ());
        l.insert(6, 7, ());

        let mut collection = vec![];

        l.breadth_first_search(2, |i| {
            collection.push(i);
        });

        let expected = [2, 1, 3, 0, 4, 5, 6, 7];

        assert_eq!(&expected, collection.as_slice());
    }

    #[test]
    fn bfs_3() {
        let mut l = AdjList::<()>::new();

        l.insert(0, 1, ());
        l.insert(1, 2, ());
        l.insert(2, 3, ());
        l.insert(3, 4, ());
        l.insert(3, 5, ());
        l.insert(4, 5, ());
        l.insert(4, 6, ());
        l.insert(5, 6, ());
        l.insert(5, 7, ());
        l.insert(6, 7, ());

        let mut collection = vec![];

        l.bfs_simple(2, |i| {
            collection.push(i);
        });

        let expected = [2, 1, 3, 0, 4, 5, 6, 7];

        assert_eq!(&expected, collection.as_slice());
    }

    #[test]
    fn shortest_path() {
        let mut l = AdjList::<()>::new();

        l.insert(0, 1, ());
        l.insert(1, 2, ());
        l.insert(2, 3, ());
        l.insert(3, 4, ());
        l.insert(3, 5, ());
        l.insert(4, 5, ());
        l.insert(4, 6, ());
        l.insert(5, 6, ());
        l.insert(5, 7, ());
        l.insert(6, 7, ());

        let path = l.shortest_path(1, 7);

        assert_eq!(Some([1, 2, 3, 5, 7].as_slice()), path.as_deref());
    }

    #[test]
    fn dfs() {
        let mut l = AdjList::<()>::new();

        l.insert(0, 1, ());
        l.insert(1, 2, ());
        l.insert(2, 3, ());
        l.insert(3, 4, ());
        l.insert(3, 5, ());
        l.insert(4, 5, ());
        l.insert(4, 6, ());
        l.insert(5, 6, ());
        l.insert(5, 7, ());
        l.insert(6, 7, ());

        let mut collection = vec![];

        l.depth_first_search(0, |i| {
            collection.push(i);
        });

        let expected = [0, 1, 2, 3, 5, 7, 6, 4];

        assert_eq!(&expected, collection.as_slice());
    }
}
