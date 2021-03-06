use std::cmp::max;
use std::fmt::Debug;


#[derive(Debug, PartialEq)]
pub struct AVLTree<T> {
    data: T,
    left_child: Option<Box<AVLTree<T>>>,
    right_child: Option<Box<AVLTree<T>>>,
    height: isize

}


pub fn make_tree_from_list<T: Clone + Debug + PartialOrd + Copy>(data_set: &[T]) -> Option<AVLTree<T>>{
    if data_set.is_empty() {
        return None
    }

    let data = data_set[0].clone();

    return Some(make_tree_from_list_helper(&data_set[1..],
                                      AVLTree::make_empty_tree(data)));
}

pub fn make_tree_from_list_helper<T: Clone + Debug + PartialOrd + Copy>(data_set: &[T], accum: AVLTree<T>) -> AVLTree<T> {
    if data_set.is_empty() {
        return accum
    }
    else {
        let accum = accum.insert(data_set[0].clone());
        return make_tree_from_list_helper(&data_set[1..],
                                          accum);
    }
}

impl <T> AVLTree<T>
where T: PartialOrd + Copy + Debug
{

    pub fn make_empty_tree(data: T) -> AVLTree<T> {
        return AVLTree {
            data,
            left_child: None,
            right_child: None,
            height: 0
        };
    }



    pub fn get_height(&self) -> isize {
        return 1 + max(get_height_of_avl_tree(&self.left_child),
                       get_height_of_avl_tree(&self.right_child));
    }

    pub fn insert(self, data: T) -> AVLTree<T> {
        return self.insert_avl(data);
    }

    pub fn insert_bst(self, new_node_value: T) -> AVLTree<T> {
        if new_node_value > self.data {
            match self.right_child {
                None => {
                 let new_tree = AVLTree {
                     data: self.data,
                     left_child: self.left_child,
                     right_child: Some(Box::new(
                         AVLTree::make_empty_tree(new_node_value))),
                     height: self.height + 1
                 };
                    return new_tree.update_height();
                },
                Some(right_child) => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: self.left_child,
                        right_child: Some(Box::new(right_child.insert_bst(new_node_value))),
                        height: self.height + 1
                    };
                    return new_tree.update_height();
                }
            }
        }
        else {
            match self.left_child {
                None => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: Some(Box::new(
                            AVLTree::make_empty_tree(new_node_value))),
                        right_child: self.right_child,
                        height: self.height + 1
                    };
                    return new_tree.update_height();
                },
                Some(left_child) => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: Some(Box::new(left_child.insert_bst(new_node_value))),
                        right_child: self.right_child,
                        height: self.height + 1
                    };
                    return new_tree.update_height();
                }
            }
        }
    }


    fn is_avl_unbalanced(&self) -> bool {
        return self.avl_heaviness().abs() >= 2
    }
    // Positive when right heavy
    fn avl_heaviness(&self) -> isize {
        let left_height = get_height_of_avl_tree(&self.left_child);
        let right_height = get_height_of_avl_tree(&self.right_child);

        return right_height - left_height;
    }

    fn insert_avl(self, new_node_value: T) -> AVLTree<T> {
        let new_tree_generic_bst = self.insert_bst(new_node_value).update_height();

        let data = new_tree_generic_bst.data;
        // Now we fix any AVL mismatches
        if new_tree_generic_bst.is_avl_unbalanced() {
            return new_tree_generic_bst.fix_avl();
        }
        else {
           return match (new_tree_generic_bst.left_child, new_tree_generic_bst.right_child){
               (None, None) => {
                   return AVLTree::make_empty_tree(data)
               },
               (Some(left), None) => {
                    let updated_tree = AVLTree {
                        data,
                        left_child: Some(Box::new(left.fix_avl())),
                        right_child: None,
                        height: 0
                    };
                    updated_tree.update_height()
               },
               (None, Some(right)) => {
                    let updated_tree = AVLTree {
                        data,
                        left_child: None,
                        right_child: Some(Box::new(right.fix_avl())),
                        height: 0
                    };
                   updated_tree.update_height()
               },
               (Some(left), Some(right)) => {
                   let updated_tree = AVLTree {
                       data,
                       left_child: Some(Box::new(left.fix_avl())),
                       right_child: Some(Box::new(right.fix_avl())),
                       height: 0
                   };
                   updated_tree.update_height()
               }
            }
        }


    }

    fn fix_avl(self) -> AVLTree<T> {
        //Assume right is heavier
        let avl_heaviness = (&self).avl_heaviness();
        let data = (&self).data;

        // If this tree is right heavy or balanced
        if avl_heaviness == 0 {
            return self;
        }
        else if avl_heaviness > 0 {
            let right_rotation = self.right_rotate(data);
            return right_rotation;
        }
        else {
            // Must exist based on avl_heaviness
            let right_tree_data = (&self).right_child.as_ref().unwrap().data;
            let right_rotate_right_child = self.right_rotate(right_tree_data);
            let left_rotate_original_data = right_rotate_right_child.left_rotate(data);
            return left_rotate_original_data;

        }
    }



    fn update_height(self) -> AVLTree<T> {
        let new_height = self.get_height();
        return AVLTree {
            data: self.data,
            left_child: self.left_child,
            right_child: self.right_child,
            height: new_height
        }
    }

    fn update_height_left_child(self) -> AVLTree<T> {
        match self.left_child {
            None => return self,
            Some(left_child) => {
                let updated_left_child = left_child.update_height();
                let updated_node = AVLTree {
                    data: self.data,
                    left_child: Some(Box::new(updated_left_child)),
                    right_child: self.right_child,
                    height: self.height
                };
                return updated_node.update_height();
            }
        }
    }

    fn update_height_right_child(self) -> AVLTree<T> {
        match self.right_child {
            None => return self,
            Some(right_child) => {
                let updated_right_child = right_child.update_height();
                let updated_node = AVLTree {
                    data: self.data,
                    left_child: self.left_child,
                    right_child: Some(Box::new(updated_right_child)),
                    height: self.height
                };
                return updated_node.update_height();
            }
        }
    }

    fn update_height_children(self) -> AVLTree<T> {
        let updated_left_child = self.update_height_left_child();
        return updated_left_child.update_height_right_child();
    }



    fn right_rotate(self, data:T) -> AVLTree<T> {
        if data > self.data {
            match self.right_child {
                None => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: self.left_child,
                        right_child: Some(Box::new(
                            AVLTree::make_empty_tree(data))),
                        height: self.height
                    };
                    return new_tree.update_height();
                },
                Some(right_child) => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: self.left_child,
                        right_child: Some(Box::new(right_child.right_rotate(data))),
                        height: self.height
                    };
                    return new_tree.update_height();
                }
            }
        }
        else if data < self.data {
            match self.left_child {
                None => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: Some(Box::new(
                            AVLTree::make_empty_tree(data))),
                        right_child: self.right_child,
                        height: self.height
                    };
                    return new_tree.update_height();
                },
                Some(left_child) => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: Some(Box::new(left_child.right_rotate(data))),
                        right_child: self.right_child,
                        height: self.height
                    };
                    return new_tree.update_height();
                }
            }
        }
        else {
            // We are at the node to left rotate
            match self.right_child {
                None => {
                    // We cannot replace the root with the right child if it does not exist
                    return self
                }
                Some(right_child) => {
                    let new_root_data = right_child.data;
                    let new_right_tree = right_child.right_child;
                    // Increment new_right_tree's height by one
                    let new_right_tree = match new_right_tree {
                        None => None,
                        Some(new_right_tree) =>{
                            Some(Box::new(new_right_tree.update_height_children().update_height()))
                        }

                    };
                    let new_left_child = AVLTree {
                        data: self.data,
                        left_child: self.left_child,
                        right_child: right_child.left_child,
                        height: self.height // Updated by update_height_children
                    }.update_height_children().update_height();

                    let new_tree = AVLTree {
                        data: new_root_data,
                        left_child: Some(Box::new(new_left_child)),
                        right_child: new_right_tree,
                        height: self.height
                    };
                    return new_tree.update_height();
                }
            }
        }
    }

    fn left_rotate(self, data:T) -> AVLTree<T> {
        if data > self.data {
            match self.right_child {
                None => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: self.left_child,
                        right_child: Some(Box::new(
                            AVLTree::make_empty_tree(data))),
                        height: self.height
                    };
                    return new_tree.update_height();
                },
                Some(right_child) => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: self.left_child,
                        right_child: Some(Box::new(right_child.left_rotate(data))),
                        height: self.height
                    };
                    return new_tree.update_height();
                }
            }
        }
        else if data < self.data {
            match self.left_child {
                None => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: Some(Box::new(
                            AVLTree::make_empty_tree(data))),
                        right_child: self.right_child,
                        height: self.height
                    };
                    return new_tree.update_height();
                },
                Some(left_child) => {
                    let new_tree = AVLTree {
                        data: self.data,
                        left_child: Some(Box::new(left_child.left_rotate(data))),
                        right_child: self.right_child,
                        height: self.height
                    };
                    return new_tree.update_height();
                }
            }
        }
        else {
            // We are at the node to right rotate
            match self.left_child {
                None => {
                    // We cannot replace the root with the right child if it does not exist
                    return self
                }
                Some(left_child) => {
                    let new_root_data = left_child.data;
                    let new_left_tree = left_child.left_child;
                    // Increment new_right_tree's height by one
                    let new_left_tree = match new_left_tree {
                        None => None,
                        Some(new_left_tree) =>{
                            Some(Box::new(new_left_tree.update_height_children().update_height()))
                        }

                    };
                    let new_right_child = AVLTree {
                        data: self.data,
                        left_child: left_child.right_child,
                        right_child: self.right_child,
                        height: self.height // Updated by update_height_children
                    }.update_height_children().update_height();

                    let new_tree = AVLTree {
                        data: new_root_data,
                        left_child: new_left_tree,
                        right_child: Some(Box::new(new_right_child)),
                        height: self.height
                    };
                    return new_tree.update_height();
                }
            }
        }
    }

}


fn get_height_of_avl_tree<T>(maybe_tree: &Option<Box<AVLTree<T>>>) -> isize {
    match maybe_tree {
        None => -1,
        Some(avl_tree) => avl_tree.height
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

        assert!(insert_left_then_right.update_height().is_avl_unbalanced());
    }

    #[test]
    fn test_rotate_right() {
        let base_tree = AVLTree::make_empty_tree(10);
        let tree = base_tree.insert_bst(15);
        let tree = tree.insert_bst(13);
        let tree = tree.insert_bst(16);
        let tree = tree.insert_bst(7);
        let tree = tree.insert_bst(8);
        let tree = tree.insert_bst(3);

        let rotated_right_tree = tree.right_rotate(10);
        let expected_rotate_right = AVLTree::make_empty_tree(15);
        let expected_rotate_right = expected_rotate_right.insert_bst(10).update_height();
        let expected_rotate_right = expected_rotate_right.insert_bst(16).update_height();
        let expected_rotate_right = expected_rotate_right.insert_bst(13).update_height();
        let expected_rotate_right = expected_rotate_right.insert_bst(7).update_height();
        let expected_rotate_right = expected_rotate_right.insert_bst(3).update_height();
        let expected_rotate_right = expected_rotate_right.insert_bst(8).update_height();
        assert_eq!(rotated_right_tree, expected_rotate_right);

        let tree = AVLTree::make_empty_tree(15);
        let tree = tree.insert_bst(17);
        let tree = tree.insert_bst(20);

        assert_eq!(tree.right_rotate(15), AVLTree {
            data: 17,
            left_child: Some(Box::new(AVLTree::make_empty_tree(15))),
            right_child: Some(Box::new(AVLTree::make_empty_tree(20))),
            height: 1
        });

    }

    #[test]
    fn test_rotate_left() {
        let tree = AVLTree::make_empty_tree(15);
        let tree = tree.insert_bst(10).update_height();
        let tree = tree.insert_bst(16).update_height();
        let tree = tree.insert_bst(13).update_height();
        let tree = tree.insert_bst(7).update_height();
        let tree = tree.insert_bst(3).update_height();
        let tree = tree.insert_bst(8).update_height();

        let rotated_left_tree = tree.left_rotate(15);

        let  expected_rotate_left = AVLTree::make_empty_tree(10);
        let  expected_rotate_left =  expected_rotate_left.insert_bst(15).update_height();
        let  expected_rotate_left =  expected_rotate_left.insert_bst(13).update_height();
        let  expected_rotate_left =  expected_rotate_left.insert_bst(16).update_height();
        let  expected_rotate_left =  expected_rotate_left.insert_bst(7).update_height();
        let  expected_rotate_left =  expected_rotate_left.insert_bst(8).update_height();
        let  expected_rotate_left =  expected_rotate_left.insert_bst(3).update_height();

        assert_eq!(rotated_left_tree, expected_rotate_left);


    }

    #[test]
    fn test_get_height_of_avl_tree() {
        let base_tree = AVLTree::make_empty_tree(10);
        assert_eq!(get_height_of_avl_tree(&Some(Box::new(AVLTree::make_empty_tree(10)))), 0);
        let tree = base_tree.insert_bst(15);
        assert_eq!(get_height_of_avl_tree::<i32>(&None), -1);
        assert_eq!(get_height_of_avl_tree(&Some(Box::new(tree))), 1);
    }

    #[test]
    fn test_insert_avl() {

        let tree = AVLTree::make_empty_tree(15);
        let tree = tree.insert_avl(17);
        let tree = tree.insert_avl(20);

        assert_eq!(tree, AVLTree {
            data: 17,
            left_child: Some(Box::new(AVLTree {
                data: 15,
                left_child: None,
                right_child: None,
                height: 0
            })),
            right_child: Some(Box::new(AVLTree {
                data: 20,
                left_child: None,
                right_child: None,
                height:0
            })),
            height: 1
        });

        let tree = AVLTree::make_empty_tree(15);
        let tree = tree.insert_avl(17);
        let tree = tree.insert_avl(20);
        let tree = tree.insert_avl(21);
        let tree = tree.insert_avl(23);
        let tree = tree.insert_avl(25);

        assert_eq!(tree, AVLTree {
            data: 21,
            left_child: Some(Box::new(AVLTree {
                data: 17,
                left_child: Some(Box::new(AVLTree::make_empty_tree(15))),
                right_child: Some(Box::new(AVLTree::make_empty_tree(20))),
                height: 1
            })),
            right_child: Some(Box::new(AVLTree {
                data: 23,
                left_child: None,
                right_child: Some(Box::new(AVLTree::make_empty_tree(25))),
                height: 1
            })),
            height: 2
        })

    }

}