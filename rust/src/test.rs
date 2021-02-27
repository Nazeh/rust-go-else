pub fn run(id: &str ,expected:&str, result: &str) {
    assert!(
        result == expected,
        format!("{}\nExpected: {},\nGot: {}\n", id, expected, result)
    );

    println!("Sucess at {}", id)
}