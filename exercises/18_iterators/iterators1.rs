// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[test]
#[cfg(test)]
mod tests {
    #[test]
    fn test_favorite_fruits() {
        let my_fav_fruits = vec![
            "banana".to_string(),
            "custard apple".to_string(),
            "avocado".to_string(),
            "peach".to_string(),
            "raspberry".to_string(),
        ];

        let mut my_iterable_fav_fruits = my_fav_fruits.into_iter(); // TODO: Step 1

        assert_eq!(my_iterable_fav_fruits.next(), Some("banana".to_string()));
        assert_eq!(my_iterable_fav_fruits.next(), Some("custard apple".to_string())); // TODO: Step 2
        assert_eq!(my_iterable_fav_fruits.next(), Some("avocado".to_string()));
        assert_eq!(my_iterable_fav_fruits.next(), Some("peach".to_string())); // TODO: Step 3
        assert_eq!(my_iterable_fav_fruits.next(), Some("raspberry".to_string()));
        assert_eq!(my_iterable_fav_fruits.next(), None); // TODO: Step 4
    }
}
