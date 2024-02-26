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
}
