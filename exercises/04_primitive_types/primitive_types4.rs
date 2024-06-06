// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // Crée une tranche contenant les éléments 2, 3, 4

    // Compare la tranche nice_slice avec une tranche littérale [2, 3, 4]
    assert_eq!(nice_slice, &[2, 3, 4]);
}

fn main() {
    slice_out_of_array();
}
