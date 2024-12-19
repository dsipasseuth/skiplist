mod structures;
mod iterator;
#[cfg(test)]
mod tests {

    use crate::structures::SkipList;

    #[test]
    fn insert_many_in_order_stay_in_order_test() {
        let mut skip_list = SkipList::new();

        skip_list.insert(1)
            .insert(2)
            .insert(3)
            .insert(4)
            .insert(5)
            .insert(6)
            .insert(7)
            .insert(8)
            .insert(9);
        assert_eq!(skip_list.len(), 9);
        skip_list.into_iter().zip([1, 2, 3, 4, 5, 6, 7, 8, 9].iter()).for_each(|(a, b)| {
            assert_eq!(a, b)
        })
    }

    #[test]
    fn insert_many_in_reverse_order_should_be_in_order_test() {
        let mut skip_list = SkipList::new();

        skip_list.insert(9)
            .insert(8)
            .insert(7)
            .insert(6)
            .insert(5)
            .insert(4)
            .insert(3)
            .insert(2)
            .insert(1);

        assert_eq!(skip_list.len(), 9);
        skip_list.into_iter().zip([1, 2, 3, 4, 5, 6, 7, 8, 9].iter()).for_each(|(a, b)| {
            assert_eq!(a, b)
        })
    }

    #[test]
    fn insert_many_duplicates_should_be_in_order_test() {
        let mut skip_list = SkipList::new();

        skip_list.insert(3)
            .insert(2)
            .insert(1)
            .insert(3)
            .insert(2)
            .insert(1)
            .insert(3)
            .insert(2)
            .insert(1);

        assert_eq!(skip_list.len(), 9);
        skip_list.into_iter().zip([1, 1, 1, 2, 2, 2, 3, 3, 3].iter()).for_each(|(a, b)| {
            assert_eq!(a, b)
        })
    }
}
