struct TreeNode<'a, T> {
    elem: T,
    children: Vec<&'a TreeNode<'a, T>>,
}

impl<'a, T> TreeNode<'a, T> {
    fn new(elem: T) -> TreeNode<'a, T> {
        TreeNode {
            elem: elem,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: &'a TreeNode<'a, T>) {
        self.children.push(&child);
    }

    fn depth_first_iter(&self) -> DepthFirstIter<T> {
        let sub_iters: Vec<DepthFirstIter<T>> = self.children.iter().map(|child| {
            child.depth_first_iter()
        }).collect();
        DepthFirstIter {
            head: Some(self),
            active_iter: None,
            sub_iters: sub_iters,
        }
    }
}

struct DepthFirstIter<'a, T> {
    head: Option<&'a TreeNode<'a, T>>,
    active_iter: Option<Box<DepthFirstIter<'a, T>>>,
    sub_iters: Vec<DepthFirstIter<'a, T>>,
}

impl<'a, T> Iterator for DepthFirstIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.active_iter.is_none() {
            let maybe_next_child = self.sub_iters.pop();
            if maybe_next_child.is_some() {
                self.active_iter = Some(Box::new(maybe_next_child.unwrap()));
            }
        }

        let mut next: Option<Self::Item> = None;
        match self.active_iter.as_mut() {
            None => {
                next = match self.head {
                    None => None,
                    Some(node) => Some(&node.elem),
                };
                self.head = None;
            },
            Some(active_iter) => {
                next = active_iter.next();
                if active_iter.head.is_none() {
                    self.active_iter = None;
                }
            }
        };
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let tree = TreeNode::new(5);
        
        assert_eq!(tree.elem, 5);
        assert_eq!(tree.children.len(), 0);
    }

    #[test]
    fn tree_with_children() {
        let mut tree = TreeNode::new(5);
        let left = TreeNode::new(6);
        let right = TreeNode::new(7);

        tree.add_child(&left);
        tree.add_child(&right);

        assert_eq!(tree.children.len(), 2);
        let values: Vec<i32> = tree.children.iter().map(|node| {
            node.elem
        }).collect();
        assert_eq!(values, vec![6, 7])
    }

    #[test]
    fn add_subtrees() {
        let mut head = TreeNode::new(String::from("head"));

        let mut left_subtree = TreeNode::new(String::from("head/left"));
        let left_leaf_1 = TreeNode::new(String::from("head/left/leaf1"));
        let left_leaf_2 = TreeNode::new(String::from("head/left/leaf2"));
        left_subtree.add_child(&left_leaf_1);
        left_subtree.add_child(&left_leaf_2);

        let mut right_subtree = TreeNode::new(String::from("head/left"));
        let right_leaf_1 = TreeNode::new(String::from("head/right/leaf1"));
        let right_leaf_2 = TreeNode::new(String::from("head/right/leaf2"));
        right_subtree.add_child(&right_leaf_1);
        right_subtree.add_child(&right_leaf_2);

        head.add_child(&left_subtree);
        head.add_child(&right_subtree);

        assert_eq!(head.children.len(), 2);

        let left = head.children.pop().unwrap();
        assert_eq!(left.elem, "head/left");
        assert_eq!(left.children.len(), 2);

        let right = head.children.pop().unwrap();
        assert_eq!(right.elem, "head/left");
        assert_eq!(right.children.len(), 2);
    }
    #[test]
    fn df_iter_empty_tree() {
        let tree = TreeNode::new(String::from("just me"));
        let mut iter = tree.depth_first_iter();
        assert_eq!(iter.next(), Some(&String::from("just me")));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn df_two_levels() {
        let mut head = TreeNode::new(String::from("head"));

        let mut left_subtree = TreeNode::new(String::from("head/left"));
        let left_leaf_1 = TreeNode::new(String::from("head/left/leaf1"));
        let left_leaf_2 = TreeNode::new(String::from("head/left/leaf2"));
        let left_leaf_3 = TreeNode::new(String::from("head/left/leaf3"));
        left_subtree.add_child(&left_leaf_1);
        left_subtree.add_child(&left_leaf_2);
        left_subtree.add_child(&left_leaf_3);

        let mut right_subtree = TreeNode::new(String::from("head/right"));
        let right_leaf_1 = TreeNode::new(String::from("head/right/leaf1"));
        let right_leaf_2 = TreeNode::new(String::from("head/right/leaf2"));
        right_subtree.add_child(&right_leaf_1);
        right_subtree.add_child(&right_leaf_2);

        head.add_child(&left_subtree);
        head.add_child(&right_subtree);

        let mut iter = head.depth_first_iter();
        assert_eq!(iter.next(), Some(&String::from("head/right/leaf2")));
        assert_eq!(iter.next(), Some(&String::from("head/right/leaf1")));
        assert_eq!(iter.next(), Some(&String::from("head/right")));
        assert_eq!(iter.next(), Some(&String::from("head/left/leaf3")));
        assert_eq!(iter.next(), Some(&String::from("head/left/leaf2")));
        assert_eq!(iter.next(), Some(&String::from("head/left/leaf1")));
        assert_eq!(iter.next(), Some(&String::from("head/left")));
        assert_eq!(iter.next(), Some(&String::from("head")));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn df_deep_levels() {
        let mut head = TreeNode::new(String::from("head"));
        let mut level_1 = TreeNode::new(String::from("head/1"));
        let mut level_2 = TreeNode::new(String::from("head/1/2"));
        let mut level_3 = TreeNode::new(String::from("head/1/2/3"));
        let mut level_4 = TreeNode::new(String::from("head/1/2/3/4"));
        let mut level_5 = TreeNode::new(String::from("head/1/2/3/4/5"));
        let mut level_6 = TreeNode::new(String::from("head/1/2/3/4/5/6"));
        let level_6a = TreeNode::new(String::from("head/1/2/3/4/5/6/A"));
        let level_6b = TreeNode::new(String::from("head/1/2/3/4/5/6/B"));

        level_6.add_child(&level_6a);
        level_6.add_child(&level_6b);

        level_5.add_child(&level_6);
        level_4.add_child(&level_5);
        level_3.add_child(&level_4);
        level_2.add_child(&level_3);
        level_1.add_child(&level_2);
        head.add_child(&level_1);

        let mut df = head.depth_first_iter();
        assert_eq!(df.next(), Some(&String::from("head/1/2/3/4/5/6/B")));
        assert_eq!(df.next(), Some(&String::from("head/1/2/3/4/5/6/A")));
        assert_eq!(df.next(), Some(&String::from("head/1/2/3/4/5/6")));
        assert_eq!(df.next(), Some(&String::from("head/1/2/3/4/5")));
        assert_eq!(df.next(), Some(&String::from("head/1/2/3/4")));
        assert_eq!(df.next(), Some(&String::from("head/1/2/3")));
        assert_eq!(df.next(), Some(&String::from("head/1/2")));
        assert_eq!(df.next(), Some(&String::from("head/1")));
        assert_eq!(df.next(), Some(&String::from("head")));
        assert_eq!(df.next(), None);
        assert_eq!(df.next(), None);
    }
}
