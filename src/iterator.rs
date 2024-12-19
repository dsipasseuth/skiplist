use std::ptr::{NonNull};
use crate::structures::{SkipList, SkipNode};

pub struct SkipListIterator<'a, T: PartialOrd> {
    current_node: Option<&'a NonNull<SkipNode<T>>>
}

impl<'a, T: PartialOrd> SkipListIterator<'a, T> {
    fn new(skip_list: &'a SkipList<T>) -> Self {
        SkipListIterator {
            current_node: skip_list.lanes.get(0)
        }
    }
}

impl<'a, T: PartialOrd> Iterator for SkipListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_node.map(|n| unsafe { n.as_ref() } );
        self.current_node = self.current_node
            .map_or(None,
                    |node| unsafe {
                        node.as_ref().next.get(0)
                    }
            );
        result.map(|node| &node.value)
    }
}

impl<'a, T: PartialOrd> IntoIterator for &'a SkipList<T> {
    type Item = &'a T;
    type IntoIter = SkipListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SkipListIterator::new(self)
    }
}
