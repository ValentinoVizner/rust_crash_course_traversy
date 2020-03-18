// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first
    // variable will no longer hold the that value.
    // You'll need to use a reference (&) to point to the resource.

    // Vector
    let vec1 = vec![1, 2, 3];
    // If we use as in array, we get this error:
    /*
    ---- move occurs because `vec1` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    16 |     let vec2 = vec1;
    |                ---- value moved here
    17 |
    18 |     println!("Values: {:?}", (vec1, vec2));
    |                               ^^^^ value used here after move
    */
    //let vec2 = vec1;

    // Correct is to use & to point to the vec1
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));
}
