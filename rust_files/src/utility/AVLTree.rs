
#[derive(Debug, PartialEq)]
pub struct AVLTree<T> {
    data: T,
    left_child: Option<Box<AVLTree<T>>>,
    right_child: Option<Box<AVLTree<T>>>,
    height: isize

}

impl <T> AVLTree<T>
where T: PartialOrd
{

    pub fn make_empty_tree(data: T) -> AVLTree<T> {
        return AVLTree {
            data,
            left_child: None,
            right_child: None,
            height: 0
        };
    }

    pub fn get_height(self) -> isize {
        return self.height;
    }

    pub fn insert_bst(self, new_node_value: T) -> AVLTree<T> {
        if new_node_value > self.data {
            match self.right_child {
                None => return AVLTree {
                    data: self.data,
                    left_child: self.left_child,
                    right_child: Some(Box::new(
                        AVLTree::make_empty_tree(new_node_value))),
                    height: self.height + 1
                },
                Some(right_child) => return AVLTree {
                        data: self.data,
                        left_child: self.left_child,
                        right_child: Some(Box::new(right_child.insert_bst(new_node_value))),
                        height: self.height + 1
                }
            }
        }
        else {
            match self.left_child {
                None => return AVLTree {
                    data: self.data,
                    left_child: Some(Box::new(
                        AVLTree::make_empty_tree(new_node_value))),
                    right_child: self.right_child,
                    height: self.height + 1
                },
                Some(left_child) => {
                    return AVLTree {
                        data: self.data,
                        left_child: Some(Box::new(left_child.insert_bst(new_node_value))),
                        right_child: self.right_child,
                        height: self.height + 1
                    }
                }
            }
        }
    }

    fn is_avl_unbalanced(&self) -> bool {
        return false
    }

    fn is_leaf(&self) -> bool {
        return false;
    }

    fn left_rotate(&self, data:T) -> AVLTree<T> {
        return AVLTree::make_empty_tree(data);
    }

}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_make_empty_tree() {
        let value = 3;
        assert_eq!(AVLTree::make_empty_tree(value), AVLTree {
            data: value,
            left_child: None,
            right_child: None,
            height: 0
        });
    }

    #[test]
    fn test_get_height() {
        let empty_tree = AVLTree::make_empty_tree(34);
        assert_eq!(empty_tree.get_height(), 0);
    }

    #[test]
    fn test_insert() {
        let base_tree = AVLTree::make_empty_tree(10);
        let insert_to_right = base_tree.insert_bst(15);
        assert_eq!(insert_to_right, AVLTree {
            data: 10,
            left_child: None,
            right_child: Some(Box::new(AVLTree::make_empty_tree(15))),
            height: 1
        });
        let insert_left_then_right = insert_to_right.insert_bst(13);
        assert_eq!(insert_left_then_right, AVLTree {
            data: 10,
            left_child: None,
            right_child: Some(Box::new(AVLTree {
                data: 15,
                left_child: Some(Box::new(AVLTree::make_empty_tree(13))),
                right_child: None,
                height: 1
            })),
            height: 2
        });
    }

    #[test]
    fn test_is_avl_unbalanced() {
        let base_tree = AVLTree::make_empty_tree(10);
        let insert_to_right = base_tree.insert_bst(15);
        let insert_left_then_right = insert_to_right.insert_bst(13);

        assert!(insert_left_then_right.is_avl_unbalanced());
    }

    #[test]
    fn test_is_leaf() {
        let base_tree = AVLTree::make_empty_tree(10);
        assert!(base_tree.is_leaf());
        let insert_to_right = base_tree.insert_bst(15);
        assert!(!insert_to_right.is_leaf());
        
    }
    #[test]
    fn test_rotate_left() {
        let base_tree = AVLTree::make_empty_tree(10);
        let tree = base_tree.insert_bst(15);
        let tree = tree.insert_bst(13);
        let tree = tree.insert_bst(16);
        let tree = tree.insert_bst(7);
        let tree = tree.insert_bst(8);
        let tree = tree.insert_bst(3);

        let rotated_left_tree = tree.left_rotate(10);
        let expected_rotate_left = AVLTree::make_empty_tree(15);
        let expected_rotate_left = expected_rotate_left.insert_bst(10);
        let expected_rotate_left = expected_rotate_left.insert_bst(16);
        let expected_rotate_left = expected_rotate_left.insert_bst(13);
        let expected_rotate_left = expected_rotate_left.insert_bst(7);
        let expected_rotate_left = expected_rotate_left.insert_bst(3);
        let expected_rotate_left = expected_rotate_left.insert_bst(8);
        assert_eq!(rotated_left_tree, expected_rotate_left);



    }

}