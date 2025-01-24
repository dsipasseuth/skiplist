use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;
use rand::Rng;

use rand::rngs::ThreadRng;

pub struct SkipList<T: PartialOrd> {
    pub(crate) lanes: Vec<NonNull<SkipNode<T>>>,
    pub(crate) size: usize,
    pub(crate) rng: ThreadRng
}

pub(crate) struct SkipNode<T: PartialOrd> {
    pub(crate) next: Vec<NonNull<SkipNode<T>>>,
    pub(crate) value: T
}

impl<T: PartialOrd> SkipNode<T> {
    fn new(value: T, lane: usize) -> Self {
        SkipNode {
            value,
            next: Vec::with_capacity(lane + 1) // lane 0 is all elements.
        }
    }
}

pub(crate) struct LaneWalker<'a, T: PartialOrd> {
    // current node, can be None
    current_node: Option<NonNull<SkipNode<T>>>,
    // next iteration lanes
    lanes: &'a Vec<NonNull<SkipNode<T>>>,
}

impl<'a, T: PartialOrd> LaneWalker<'a, T> {

    pub(crate) fn new(starting_nodes: &'a Vec<NonNull<SkipNode<T>>>) -> Self {
        Self {
            current_node: None,
            lanes: starting_nodes,
        }
    }

    fn next(&mut self, lane: usize) {
        self.current_node = self.peek_next(lane);
        if self.current_node.is_none() {
            panic!("Cannot move to next item!")
        }
    }

    fn peek_next(&self, lane: usize) -> Option<NonNull<SkipNode<T>>> {
        if let Some(node) = self.current_node {
            unsafe { node.as_ref().next.get(lane).map_or(None, |&n| Some(n)) }
        } else {
            self.lanes.get(lane).map_or(None, |&n| Some(n))
        }
    }
}

impl<T: PartialOrd + Debug> Debug for SkipList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_list();
        let mut lane_walker = LaneWalker::new(&self.lanes);
        while let Some (node) = lane_walker.peek_next(0) {
            unsafe { debug.entry(&node.as_ref().value) };
            lane_walker.next(0);
        };
        debug.finish()
    }
}

impl<'a, T: PartialOrd> SkipList<T> {

    fn with(lane_count: usize, rng: ThreadRng) -> Self {
        Self {
            lanes: Vec::with_capacity(lane_count),
            size: 0,
            rng
        }
    }

    pub fn new() -> Self {
        Self::with(16, rand::rng())
    }

    fn search_path(&self, value: &T) -> Vec<Option<NonNull<SkipNode<T>>>> {
        let current_lane_count = self.lanes.len();
        let mut updates = Vec::with_capacity(current_lane_count);
        updates.resize(current_lane_count, None);

        let mut node_walker = LaneWalker::new(&self.lanes);
        let mut current_lane = current_lane_count;
        while current_lane > 0 {
            let lane_index = current_lane - 1;
            if node_walker.peek_next(lane_index).is_some_and(| node| unsafe { &node.as_ref().value <= value } == true) {
                node_walker.next(lane_index);
                continue;
            }
            updates[lane_index] = node_walker.current_node;
            current_lane -= 1;
        }
        updates
    }

    pub fn insert(&mut self, value: T) -> &mut Self {
        let updates = self.search_path(&value);

        let max_lane = if updates.get(0).is_some_and(
                |v| v.as_ref().is_some_and(
                    |n| unsafe { n.as_ref().value == value} == true)) {
            0 // duplicate value
        } else {
            self.rng.random_range(0 ..self.lanes.capacity())
        };

        let mut shared_new_node = NonNull::new(Box::into_raw(Box::new(SkipNode::new(value, max_lane)))).unwrap();

        for lane_index in 0..=max_lane {
            if let Some(mut previous_node) = updates.get(lane_index).map_or(None, |&item| item) {
                let previous_node_borrow = unsafe { previous_node.as_mut() };
                if let Some(&previous_next_node) = previous_node_borrow.next.get(lane_index) {
                    unsafe { shared_new_node.as_mut().next.push(previous_next_node); }
                    previous_node_borrow.next[lane_index] = shared_new_node;
                } else {
                    previous_node_borrow.next.push(shared_new_node);
                }
            } else {
                // there's nothing, so add it to lane
                if let Some(&start_node)  = self.lanes.get(lane_index) {
                    unsafe { shared_new_node.as_mut().next.push(start_node) };
                    self.lanes[lane_index] = shared_new_node;
                } else {
                    self.lanes.resize(lane_index+1, shared_new_node);
                }
            }
        }

        self.size += 1;
        self
    }

    pub fn len(&self) -> usize {
        self.size
    }
}
