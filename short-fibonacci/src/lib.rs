/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    return Vec::new();
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    return vec![0; count];
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut x = vec![1, 1];
    for i in 2..5 {
        let next_x = x[i - 1] + x[i - 2];
        x.push(next_x)
    }
    return x;
}
