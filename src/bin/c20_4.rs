/***
二元树是一种树型数据结构，其中每个节点都有两个子节点（左侧和右侧）。我们将创建一个树状结构，其中每个节点存储一个值。对于给定的节点 N，N 的左侧子树中的所有节点都包含较小的值，而 N 的右侧子树中的所有节点都将包含较大的值。

实现以下类型，以便通过指定的测试。

额外提示：对按顺序返回值的二元树实现迭代器。
 */
use std::ops::Deref;

/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Subtree<T> {
    pub fn insert(&mut self, val: T) {
        match &mut self.0 {
            None => {
                self.0 = Some(Box::new(Node {
                    value: val,
                    left: Subtree(None),
                    right: Subtree(None),
                }))
            }
            Some(node) => {
                if val == node.value {
                } else if val < node.value {
                    node.left.insert(val)
                } else {
                    node.right.insert(val)
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(node) => return node.left.len() + node.right.len() + 1,
        }
    }

    pub fn has(&self, val: &T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => {
                if *val == node.value {
                    true
                } else if *val < node.value {
                    node.left.has(val)
                } else {
                    node.right.has(val)
                }
            }
        }
    }
}

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: Subtree(None),
        }
    }

    pub fn insert(&mut self, val: T) {
        self.root.insert(val)
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn has(&self, val: &T) -> bool {
        self.root.has(val)
    }
}

// Implement `new`, `insert`, `len`, and `has`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}

fn main() {}
