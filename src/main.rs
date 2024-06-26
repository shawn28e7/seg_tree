//! The `seg_tree` module provides an implementation of a segment tree for efficient range queries and updates.
//!
//! # Example
//!
//! ```
//! use seg_tree::SegTree;
//!
//! fn main() {
//!     let mut seg_tree = SegTree::new(0, 10);
//!     println!("Build success");
//!
//!     for i in 0..10 {
//!         seg_tree.revise(i, i as i32);
//!     }
//!     println!("Revise success");
//!
//!     for i in 1..=10 {
//!         println!("Sum from 0 to {}: {}", i - 1, seg_tree.ask(0, i));
//!     }
//!     println!("Ask success");
//! }
//! ```
pub mod seg_tree
{
    use std::cell::RefCell;
    use std::rc::Rc;
    pub struct SegTree
    {
        val: i32,
        range: (usize, usize),
        mid: usize,
        l_node: Option<Rc<RefCell<SegTree>>>,
        r_node: Option<Rc<RefCell<SegTree>>>,
    }

    impl SegTree
    {
        /// Creates a new segment tree with the specified range `[l, r)`.
        ///
        /// # Panics
        ///
        /// Panics if `l >= r`, as this would create an invalid range.
        ///
        /// # Examples
        ///
        /// ```
        /// let seg_tree = SegTree::new(0, 10);
        /// ```
        pub fn new(l: usize, r: usize) -> SegTree
        {
            if l >= r
            {
                panic!("Invalid range: left bound must be less than right bound");
            }
            let m: usize = l + (r - l) / 2;
            SegTree {
                val: 0,
                l_node: Some(Self::build(l, m)),
                r_node: Some(Self::build(m, r)),
                range: (l, r),
                mid: l + (r - l) / 2,
            }
        }

        fn build(l_bound: usize, r_bound: usize) -> Rc<RefCell<SegTree>>
        {
            if r_bound - l_bound == 1
            {
                return Rc::new(RefCell::new(SegTree {
                    val: 0,
                    l_node: None,
                    r_node: None,
                    range: (l_bound, r_bound),
                    mid: l_bound,
                }));
            }
            let m = l_bound + (r_bound - l_bound) / 2;
            Rc::new(RefCell::new(SegTree {
                val: 0,
                l_node: Some(Self::build(l_bound, m)),
                r_node: Some(Self::build(m, r_bound)),
                range: (l_bound, r_bound),
                mid: l_bound + (r_bound - l_bound) / 2,
            }))
        }
        /// Updates the value at a specific index in the segment tree.
        ///
        /// # Arguments
        ///
        /// * `tar` - The index to update.
        /// * `k` - The new value.
        ///
        /// # Panics
        ///
        /// Panics if the target index is out of range.
        ///
        /// # Examples
        ///
        /// ```
        /// let mut seg_tree = SegTree::new(0, 10);
        /// seg_tree.revise(2, 10);
        /// ```
        pub fn revise(&mut self, target_pos: usize, value: i32)
        {
            if target_pos < self.range.0 || target_pos >= self.range.1
            {
                panic!("Target index out of range");
            }
            if (target_pos, target_pos + 1) == self.range
            {
                self.val = value;
                return;
            }
            if target_pos < self.mid
            {
                if let Some(ref left) = self.l_node
                {
                    left.borrow_mut().revise(target_pos, value);
                }
            }
            else
            {
                if let Some(ref right) = self.r_node
                {
                    right.borrow_mut().revise(target_pos, value);
                }
            }
            self.val = SegTree::comb(
                self.l_node.as_ref().map_or(0, |left| left.borrow().val),
                self.r_node.as_ref().map_or(0, |right| right.borrow().val),
            );
        }
        /// Queries the sum of values in the specified range `[l, r)`.
        ///
        /// # Arguments
        ///
        /// * `l` - The left bound of the query range.
        /// * `r` - The right bound of the query range.
        ///
        /// # Panics
        ///
        /// Panics if the query range is invalid.
        ///
        /// # Examples
        ///
        /// ```
        /// let seg_tree = SegTree::new(0, 10);
        /// let sum = seg_tree.ask(0, 5);
        /// ```
        pub fn ask(&self, l: usize, r: usize) -> i32
        {
            if l >= r || l < self.range.0 || r > self.range.1
            {
                panic!("Invalid query range");
            }
            if (l, r) == self.range
            {
                self.val
            }
            else if r <= self.mid
            {
                self.l_node
                    .as_ref()
                    .map_or(0, |left| left.borrow().ask(l, r))
            }
            else if l >= self.mid
            {
                self.r_node
                    .as_ref()
                    .map_or(0, |right| right.borrow().ask(l, r))
            }
            else
            {
                let left_val = self
                    .l_node
                    .as_ref()
                    .map_or(0, |left| left.borrow().ask(l, self.mid));
                let right_val = self
                    .r_node
                    .as_ref()
                    .map_or(0, |right| right.borrow().ask(self.mid, r));
                left_val + right_val
            }
        }

        // for testing
        pub fn get_val(&self) -> i32
        {
            self.val
        }
        pub fn get_range(&self) -> (usize, usize)
        {
            self.range
        }
        // combine two values
        fn comb(a: i32, b: i32) -> i32
        {
            a + b
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::seg_tree::SegTree;

    #[test]
    fn test_build()
    {
        let seg_tree = SegTree::new(0, 10);
        assert_eq!(seg_tree.get_range(), (0, 10));
        assert_eq!(seg_tree.get_val(), 0);
    }

    #[test]
    #[should_panic(expected = "Invalid range: left bound must be less than right bound")]
    fn test_invalid_build()
    {
        SegTree::new(10, 0);
    }

    #[test]
    fn test_revise()
    {
        let mut seg_tree = SegTree::new(0, 10);
        seg_tree.revise(2, 10);
        assert_eq!(seg_tree.ask(2, 3), 10);
    }

    #[test]
    #[should_panic(expected = "Target index out of range")]
    fn test_invalid_revise()
    {
        let mut seg_tree = SegTree::new(0, 10);
        seg_tree.revise(10, 10);
    }

    #[test]
    fn test_ask()
    {
        let mut seg_tree = SegTree::new(0, 10);
        for i in 0..10
        {
            seg_tree.revise(i, i as i32);
        }
        assert_eq!(seg_tree.ask(0, 10), 45); // Sum of 0 to 9
        assert_eq!(seg_tree.ask(0, 5), 10); // Sum of 0 to 4
        assert_eq!(seg_tree.ask(5, 10), 35); // Sum of 5 to 9
        assert_eq!(seg_tree.ask(3, 7), 18); // Sum of 3 to 6
    }

    #[test]
    #[should_panic(expected = "Invalid query range")]
    fn test_invalid_ask()
    {
        let seg_tree = SegTree::new(0, 10);
        seg_tree.ask(10, 0);
    }
}
fn main()
{
    let mut seg_tree: seg_tree::SegTree = seg_tree::SegTree::new(0, 10);
    println!("Build success");

    for i in 0..10
    {
        seg_tree.revise(i, i as i32);
    }
    println!("Revise success");

    for i in 1..=10
    {
        println!("Sum from {} to {}: {}", 0, i - 1, seg_tree.ask(0, i));
    }
    println!("Ask success");
}
