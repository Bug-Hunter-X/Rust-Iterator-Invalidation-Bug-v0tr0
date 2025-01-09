fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Create a copy of the vector to avoid iterator invalidation
    let vec_copy = vec.clone();

    for element in vec_copy {
        println!("Element: {}", element);
    }

    // Modify the original vector
    vec.push(4);

    println!("Modified vector: {:?}", vec);
} 