
#[derive(Debug, PartialEq)]
pub struct AVLTree<T> {
    data: T,
    left_child: Option<Box<AVLTree<T>>>,
    right_child: Option<Box<AVLTree<T>>>,
    height: isize

}

impl <T> AVLTree<T>
where T: Sized
{

    pub fn make_empty_tree(data: T) -> AVLTree<T> {
        return AVLTree {
            data,
            left_child: None,
            right_child: None,
            height: 0
        };
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

}