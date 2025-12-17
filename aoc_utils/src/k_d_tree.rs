/** k-D tree for spatial queries
 *
 * For 3D integer points. (Can be adapted for f32/f64 with minor changes.)
 * Should work for most AoC problems involving spatial queries.
 * 
 * https://en.wikipedia.org/wiki/K-d_tree
 * https://www.baeldung.com/cs/k-d-trees
 * https://stackoverflow.com/questions/65003877/understanding-leafsize-in-scipy-spatial-kdtree
 *
*/
use glam::IVec3;

#[derive(Clone, Copy,Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    #[inline]
    pub fn next(self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
}


/// Axis-aligned bounding box for integer 3D points
/// empty AABB has min > max
#[derive(Clone, Copy, Debug)]
pub struct IAabb {
    pub min: IVec3,
    pub max: IVec3,
}

impl IAabb {
    #[inline]
    pub fn contains(&self, p: IVec3) -> bool {
        let min = self.min;
        let max = self.max;
        p.x >= min.x && p.x <= max.x &&
        p.y >= min.y && p.y <= max.y &&
        p.z >= min.z && p.z <= max.z
    }

    #[inline]
    pub fn expand_to_fit(&mut self, p: IVec3) {
        self.min = self.min.min(p);
        self.max = self.max.max(p);
    }

    #[inline]
    pub fn overlaps(&self, other: &IAabb) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            min: IVec3::new(i32::MAX, i32::MAX, i32::MAX),
            max: IVec3::new(i32::MIN, i32::MIN, i32::MIN),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }
}

#[derive(Clone)]
struct Node {
    aabb: IAabb,
    axis: Axis,            // X, Y, or Z
    left: Option<usize>,   // child node indices
    right: Option<usize>,
    start: usize,          // leaf range into indices
    end: usize,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Node {:?} {} - {}", &self.axis, &self.aabb.min, &self.aabb.max))
        // f.debug_struct("Node").field("aabb", &self.aabb).field("axis", &self.axis).field("left", &self.left).field("right", &self.right).field("start", &self.start).field("end", &self.end).finish()
    }
}


/// k-D tree for 3D integer points
#[derive(Clone)]
pub struct IKdTree3d {
    nodes: Vec<Node>,
    /// Original points, stable storage
    points: Vec<IVec3>,
    /// Permutation over points used by leaves
    indices: Vec<usize>,
    /// `leaf_size` ~ 16–64 is a good start. 
    /// Larger leaf_size = shallower tree, faster build, slower queries.
    /// Smaller leaf_size = deeper tree, slower build, slower traversal, faster queries.
    leaf_size: usize,
}

impl std::fmt::Debug for IKdTree3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "IKdTree3d [{}] {{ nodes: {:?}, points: {:?}, indices: {:?} }}", 
            self.leaf_size,self.nodes, self.points, self.indices )

        // f.debug_struct("IKdTree3d").field("nodes", &self.nodes).field("points", &self.points).field("indices", &self.indices).field("leaf_size", &self.leaf_size).finish()
    }
}



impl IKdTree3d {
    /// Build from integer points in 3D. 
    /// `leaf_size` ~ 16–64 is a good start. 
    /// Larger leaf_size = shallower tree, faster build, slower queries.
    /// Smaller leaf_size = deeper tree, slower build, slower traversal, faster queries.
    /// example:
    /// ``` rust
    /// let points = vec![IVec3::new(1,2,3), IVec3::new(4,5,6), IVec3::new(7,8,9)];
    /// let tree = IKdTree3d::new(points, 16);
    /// ```
    pub fn new(points: Vec<IVec3>, leaf_size: usize) -> Self {
        let n = points.len();
        let mut indices: Vec<usize> = (0..n).collect();
        let mut nodes = Vec::with_capacity(n.saturating_mul(2).max(1));

        // Global AABB
        let mut global = IAabb::empty();
        for &p in &points {
            global.expand_to_fit(p);
        }
        // println!("KD-Tree global AABB: {:?}", global);

        // Each stack entry: [start, end) over indices, split axis, aabb, parent index
        #[derive(Debug)]
        struct StackEntry(usize, usize, Axis, IAabb, Option<usize>);

        let mut stack: Vec<StackEntry> = vec![StackEntry(0, n, Axis::X, global, None)];

        while let Some(StackEntry(start, end, axis, aabb, parent)) = stack.pop() {
            let node_index = nodes.len();
            nodes.push(Node {
                aabb,
                axis,
                left: None,
                right: None,
                start,
                end,
            });

            // Link to parent
            if let Some(p) = parent {
                if nodes[p].left.is_none() {
                    nodes[p].left = Some(node_index);
                } else {
                    nodes[p].right = Some(node_index);
                }
            }

            let count = end - start;
            if count <= leaf_size {
                continue;
            }

            // Median split by the selected axis
            // some recommend using a random subselection for median, but select_nth_unstable is fast enough in our cases
            let mid = start + count / 2;

            let axis_key = |p: IVec3| -> i32 {
                match axis {
                    Axis::X => p.x,
                    Axis::Y => p.y,
                    Axis::Z => p.z,
                }
            };

            // reorder indices in place so that median is at mid. Elements before mid are <= median, after are >= median.
            indices[start..end].select_nth_unstable_by(mid - start, |&a, &b| {
                axis_key(points[a]).cmp(&axis_key(points[b]))
            });

            // Split value (median key)
            let split_val = axis_key(points[indices[mid]]);

            // Child AABBs (tightening only along split axis for speed)
            let (mut left_aabb, mut right_aabb) = (aabb, aabb);
            match axis {
                Axis::X => { left_aabb.max.x = split_val; right_aabb.min.x = split_val; }
                Axis::Y => { left_aabb.max.y = split_val; right_aabb.min.y = split_val; }
                Axis::Z => { left_aabb.max.z = split_val; right_aabb.min.z = split_val; }
            }

            let next_axis = axis.next();
            // Push right first, then left to visit left first during build (optional)
            stack.push(StackEntry(mid, end,   next_axis, right_aabb, Some(node_index)));
            stack.push(StackEntry(start, mid, next_axis, left_aabb,  Some(node_index)));
        }

        dbg!(Self { nodes, points, indices, leaf_size })
    }

    /// AABB range query
    /// Returns indices into original points.
    /// example:
    /// ``` rust    
    /// let results = tree.range_query(&IAabb { min: IVec3::new(0,0,0), max: IVec3::new(10,10,10) });
    /// ```
    pub fn range_query(&self, query: &IAabb) -> Vec<usize> {
        let mut out = Vec::new();
        if self.nodes.is_empty() || query.is_empty() {
            return out;
        }
        let mut stack = vec![0usize];
        while let Some(i) = stack.pop() {
            let n = &self.nodes[i];
            if !n.aabb.overlaps(query) {
                continue;
            }
            if n.end - n.start <= self.leaf_size {
                for &idx in &self.indices[n.start..n.end] {
                    if query.contains(self.points[idx]) {
                        out.push(idx);
                    }
                }
            } else {
                if let Some(l) = n.left { stack.push(l); }
                if let Some(r) = n.right { stack.push(r); }
            }
        }
        out
    }

    /// Nearest neighbor to query point q. Returns index into original points.
    /// Returns None if tree is empty.
    /// example:
    /// ``` rust
    /// let nearest = tree.nearest(IVec3::new(5,5,5));
    /// ```
    pub fn nearest(&self, q: IVec3) -> Option<usize> {
        // println!("Nearest neighbor query for point: {:?} IsEmpty: {:?}", q, self.nodes);
        if self.nodes.is_empty() {
            return None;
        }

        #[inline]
        fn sqr_dist_i64(a: IVec3, b: IVec3) -> i64 {
            let dx = (a.x as i64) - (b.x as i64);
            let dy = (a.y as i64) - (b.y as i64);
            let dz = (a.z as i64) - (b.z as i64);
            dx*dx + dy*dy + dz*dz
        }

        #[inline]
        fn aabb_sqr_dist_i64(a: &IAabb, q: IVec3) -> i64 {
            // Squared distance from a point to an AABB (0 if inside)
            let clamp_axis = |qk: i32, min: i32, max: i32| -> i64 {
                if qk < min {
                    let d = (min as i64) - (qk as i64);
                    d * d
                } else if qk > max {
                    let d = (qk as i64) - (max as i64);
                    d * d
                } else {
                    0
                }
            };

            // println!("clamp_axis q: {:?}, aabb min: {:?}, max: {:?}", q, a.min, a.max);
            if a.is_empty() {
                return i64::MAX;
            }
            clamp_axis(q.x, a.min.x, a.max.x)
            + clamp_axis(q.y, a.min.y, a.max.y)
            + clamp_axis(q.z, a.min.z, a.max.z)
        }

        let mut best_d2: i64 = i64::MAX;
        let mut best_idx: Option<usize> = None;

        // Small explicit stack
        let mut stack = vec![0usize];
        while let Some(i) = stack.pop() {
            let n = &self.nodes[i];

            println!("Looking for nearest from {} visiting node {}: {:?}", q, i, n);
            // Cheap pruning by AABB distance
            if n.aabb.is_empty() || aabb_sqr_dist_i64(&n.aabb, q) >= best_d2 {
                continue;
            }

            // inside leaf, just search all points
            if n.end - n.start <= self.leaf_size {
                println!("  Leaf node, scanning points {} to {}: {:?} - {:?}", n.start, n.end, self.points[self.indices[n.start]], self.points[self.indices[n.end - 1]]);
                for &idx in &self.indices[n.start..n.end] {
                    let d2 = sqr_dist_i64(self.points[idx], q);
                    println!("  Leaf point {:?} idx {} d2 {} best_d2 {}", self.points[idx], idx, d2, best_d2);
                    if d2 < best_d2 {
                        best_d2 = d2;
                        best_idx = Some(idx);
                    }
                }
            } else {
                // look across
                // Visit near child first to tighten best_d2 early
                // We approximate split position from child bounds
                let axis = n.axis;
                let near_first = match axis {
                    Axis::X => {
                        // Compare q.x against split plane between children
                        let lmax = n.left.map(|li| self.nodes[li].aabb.max.x);
                        let rmin = n.right.map(|ri| self.nodes[ri].aabb.min.x);
                        match (lmax, rmin) {
                            (Some(lx), Some(rx)) => q.x <= (lx + rx) / 2,
                            _ => true,
                        }
                    }
                    Axis::Y => {
                        let lmax = n.left.map(|li| self.nodes[li].aabb.max.y);
                        let rmin = n.right.map(|ri| self.nodes[ri].aabb.min.y);
                        match (lmax, rmin) {
                            (Some(ly), Some(ry)) => q.y <= (ly + ry) / 2,
                            _ => true,
                        }
                    }
                    _ => {
                        let lmax = n.left.map(|li| self.nodes[li].aabb.max.z);
                        let rmin = n.right.map(|ri| self.nodes[ri].aabb.min.z);
                        match (lmax, rmin) {
                            (Some(lz), Some(rz)) => q.z <= (lz + rz) / 2,
                            _ => true,
                        }
                    }
                };

                let (near, far) = if near_first {
                    (n.left, n.right)
                } else {
                    (n.right, n.left)
                };

                if let Some(ni) = near { stack.push(ni); }
                if let Some(fi) = far {
                    // Additional pruning: only push far if its bbox could beat best
                    let d2 = aabb_sqr_dist_i64(&self.nodes[fi].aabb, q);
                    if d2 <= best_d2 { stack.push(fi); }
                }
            }
        }

        best_idx
    }

    /// Visit all points inside the given AABB
    /// example:    
    /// ``` rust
    /// tree.visit_volume(&IAabb { min: IVec3::new(0,0,0), max: IVec3::new(10,10,10) }, |point| {
    ///     // process point here
    ///     info!("Visiting point: {:?}", point);
    /// });
    pub fn visit_volume<F>(&self, query: &IAabb, mut f: F)
    where
        F: FnMut(&IVec3),
    {
        for idx in &self.range_query(query) {
            let point = &self.points[*idx];
            // what is the best way to add a payload to a point? Separate generic storage inside IKdTree3d? 
            f(point);
        }
    }

    /// Access original point by returned index
    pub fn point(&self, idx: usize) -> IVec3 {
        self.points[idx]
    }

    pub fn len(&self) -> usize { self.points.len() }
    pub fn is_empty(&self) -> bool { self.points.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_next() {
        assert_eq!(Axis::X.next(), Axis::Y);
        assert_eq!(Axis::Y.next(), Axis::Z);
        assert_eq!(Axis::Z.next(), Axis::X);
    }

    #[test]
    fn test_iaabb_contains() {
        let aabb = IAabb {
            min: IVec3::new(0, 0, 0),
            max: IVec3::new(10, 10, 10),
        };
        assert!(aabb.contains(IVec3::new(5, 5, 5)));
        assert!(aabb.contains(IVec3::new(0, 0, 0)));
        assert!(aabb.contains(IVec3::new(10, 10, 10)));
        assert!(!aabb.contains(IVec3::new(11, 5, 5)));
        assert!(!aabb.contains(IVec3::new(-1, 5, 5)));
    }

    #[test]
    fn test_iaabb_expand_to_fit() {
        let mut aabb = IAabb::empty();
        aabb.expand_to_fit(IVec3::new(5, 5, 5));
        assert_eq!(aabb.min, IVec3::new(5, 5, 5));
        assert_eq!(aabb.max, IVec3::new(5, 5, 5));
        
        aabb.expand_to_fit(IVec3::new(10, 3, 7));
        assert_eq!(aabb.min, IVec3::new(5, 3, 5));
        assert_eq!(aabb.max, IVec3::new(10, 5, 7));
    }

    #[test]
    fn test_iaabb_overlaps() {
        let aabb1 = IAabb {
            min: IVec3::new(0, 0, 0),
            max: IVec3::new(10, 10, 10),
        };
        let aabb2 = IAabb {
            min: IVec3::new(5, 5, 5),
            max: IVec3::new(15, 15, 15),
        };
        let aabb3 = IAabb {
            min: IVec3::new(20, 20, 20),
            max: IVec3::new(30, 30, 30),
        };
        
        assert!(aabb1.overlaps(&aabb2));
        assert!(aabb2.overlaps(&aabb1));
        assert!(!aabb1.overlaps(&aabb3));
        assert!(!aabb3.overlaps(&aabb1));
    }

    #[test]
    fn test_kdtree_empty() {
        let tree = IKdTree3d::new(vec![], 16);
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.nearest(IVec3::ZERO), None);
        assert_eq!(tree.range_query(&IAabb::empty()).len(), 0);
    }

    #[test]
    fn test_kdtree_single_point() {
        let points = vec![IVec3::new(5, 5, 5)];
        let tree = IKdTree3d::new(points, 16);
        
        assert_eq!(tree.len(), 1);
        assert!(!tree.is_empty());
        assert_eq!(tree.nearest(IVec3::new(4, 4, 4)), Some(0));
        assert_eq!(tree.point(0), IVec3::new(5, 5, 5));
    }

    #[test]
    fn test_kdtree_nearest_basic() {
        let points = vec![
            IVec3::new(0, 0, 0),
            IVec3::new(10, 10, 10),
            IVec3::new(5, 5, 5),
            IVec3::new(-5, -5, -5),
        ];
        let tree = IKdTree3d::new(points.clone(), 2);
        
        let nearest = tree.nearest(IVec3::new(6, 6, 6));
        assert_eq!(nearest, Some(2)); // Closest to (5,5,5)
        
        let nearest = tree.nearest(IVec3::new(-10, -10, -10));
        assert_eq!(nearest, Some(3)); // Closest to (-5,-5,-5)
        
        let nearest = tree.nearest(IVec3::new(11, 11, 11));
        assert_eq!(nearest, Some(1)); // Closest to (10,10,10)
    }

    #[test]
    fn test_kdtree_range_query_basic() {
        let points = vec![
            IVec3::new(0, 0, 0),
            IVec3::new(5, 5, 5),
            IVec3::new(10, 10, 10),
            IVec3::new(15, 15, 15),
            IVec3::new(20, 20, 20),
        ];
        let tree = IKdTree3d::new(points, 2);
        
        let query = IAabb {
            min: IVec3::new(4, 4, 4),
            max: IVec3::new(11, 11, 11),
        };
        let results = tree.range_query(&query);
        
        assert_eq!(results.len(), 2);
        assert!(results.contains(&1)); // (5,5,5)
        assert!(results.contains(&2)); // (10,10,10)
    }

    #[test]
    fn test_kdtree_range_query_no_results() {
        let points = vec![
            IVec3::new(0, 0, 0),
            IVec3::new(10, 10, 10),
        ];
        let tree = IKdTree3d::new(points, 2);
        
        let query = IAabb {
            min: IVec3::new(20, 20, 20),
            max: IVec3::new(30, 30, 30),
        };
        let results = tree.range_query(&query);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_kdtree_range_query_all_points() {
        let points = vec![
            IVec3::new(0, 0, 0),
            IVec3::new(5, 5, 5),
            IVec3::new(10, 10, 10),
        ];
        let tree = IKdTree3d::new(points, 2);
        
        let query = IAabb {
            min: IVec3::new(-10, -10, -10),
            max: IVec3::new(20, 20, 20),
        };
        let results = tree.range_query(&query);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_kdtree_different_leaf_sizes() {
        let points: Vec<IVec3> = (0..100)
            .map(|i| IVec3::new(i, i, i))
            .collect();
        
        for leaf_size in [1, 4, 16, 32, 64] {
            let tree = IKdTree3d::new(points.clone(), leaf_size);
            assert_eq!(tree.len(), 100);
            
            let nearest = tree.nearest(IVec3::new(50, 50, 50));
            assert_eq!(nearest, Some(50));
        }
    }

    #[test]
    fn test_kdtree_negative_coordinates() {
        let points = vec![
            IVec3::new(-10, -10, -10),
            IVec3::new(-5, -5, -5),
            IVec3::new(0, 0, 0),
            IVec3::new(5, 5, 5),
        ];
        let tree = IKdTree3d::new(points, 2);
        
        let nearest = tree.nearest(IVec3::new(-8, -8, -8));
        assert_eq!(nearest, Some(0));
        
        let query = IAabb {
            min: IVec3::new(-12, -12, -12),
            max: IVec3::new(-4, -4, -4),
        };
        let results = tree.range_query(&query);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_kdtree_duplicate_points() {
        let points = vec![
            IVec3::new(5, 5, 5),
            IVec3::new(5, 5, 5),
            IVec3::new(5, 5, 5),
        ];
        let tree = IKdTree3d::new(points, 2);
        
        assert_eq!(tree.len(), 3);
        let nearest = tree.nearest(IVec3::new(5, 5, 5));
        assert!(nearest.is_some());
    }

    #[test]
    fn test_kdtree_point_access() {
        let points = vec![
            IVec3::new(1, 2, 3),
            IVec3::new(4, 5, 6),
            IVec3::new(7, 8, 9),
        ];
        let tree = IKdTree3d::new(points.clone(), 2);
        
        for i in 0..points.len() {
            assert_eq!(tree.point(i), points[i]);
        }
    }

    #[test]
    fn test_kdtree_large_coordinates() {
        let points = vec![
            IVec3::new(i32::MIN / 2, i32::MIN / 2, i32::MIN / 2),
            IVec3::new(0, 0, 0),
            IVec3::new(i32::MAX / 2, i32::MAX / 2, i32::MAX / 2),
        ];
        let tree = IKdTree3d::new(points, 2);
        
        let nearest = tree.nearest(IVec3::ZERO);
        assert_eq!(nearest, Some(1));
    }

    #[test]
    fn test_kdtree_sparse_distribution() {
        let points = vec![
            IVec3::new(0, 0, 0),
            IVec3::new(1000, 0, 0),
            IVec3::new(0, 1000, 0),
            IVec3::new(0, 0, 1000),
        ];
        let tree = IKdTree3d::new(points, 2);
        
        let nearest = tree.nearest(IVec3::new(100, 0, 0));
        assert_eq!(nearest, Some(0));
        
        let nearest = tree.nearest(IVec3::new(900, 0, 0));
        assert_eq!(nearest, Some(1));
    }
}
