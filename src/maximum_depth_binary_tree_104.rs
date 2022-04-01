// Definition for a binary tree node.
// Reference Counting:
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-04-rc.html
// Interior mutability
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-05-interior-mutability.html

use std::rc::Rc; // reference counted
use std::cell::RefCell;  // a mutable memory location with dynaimcally checked borrow rules

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,  // every option is Some and contains a value or None
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
    }
  }
}

struct Solution {

}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(a) => {
                let left = Solution::max_depth(a.borrow().left.clone());
                let right = Solution::max_depth(a.borrow().right.clone());
                return std::cmp::max(left, right) + 1;
            },
            None => return 0,
        }

    }
}

#[cfg(test)]
mod tests {
    use std::cell::{RefCell, Cell, UnsafeCell};
    use crate::maximum_depth_binary_tree_104::TreeNode;
    use crate::maximum_depth_binary_tree_104::Solution;
    use std::rc::Rc;
    use std::ops::Deref;
    use std::borrow::BorrowMut;

    #[test]
    fn test_1() {
        let mut tree_node = TreeNode{val:4, left: None, right: None};
        tree_node.left = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        tree_node.right = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let mut root = Rc::new(RefCell::new(tree_node));

        let expected_result = 2;
        let actual_result = Solution::max_depth(Option::from(root));
        assert_eq!(actual_result, expected_result);
    }
}
