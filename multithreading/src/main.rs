fn parallel_add_vec(a: &[i32], b: &[i32]) -> Vec<i32> {
    assert_eq!(a.len(), b.len(), "Vectors must be of the same length");
    let mut result = vec![0; a.len()];
    let num_threads = num_cpus::get_physical();
    let chunk_size = a.len() / num_threads;
    let chunks_a = a.chunks(chunk_size);
    let chunks_b = b.chunks(chunk_size);
    let chunks_c = result.chunks_mut(chunk_size);
    std::thread::scope(|s| {
        for (chunk_a, chunk_b, chunk_c) in itertools::izip!(chunks_a, chunks_b, chunks_c) {
            s.spawn(move || {
               for i in 0..chunk_a.len() {
                 chunk_c[i] = chunk_a[i] + chunk_b[i];
               }
            });
        }
    });
    return result;
}

fn main() {
    let N= 100_000_000; // Define the size of the vectors
    let a = vec![1; N];
    let b = vec![2; N];
    
    println!("Starting parallel vector addition of {} elements...", a.len());
    
    let start = std::time::Instant::now();
    let result = parallel_add_vec(&a, &b);
    let duration = start.elapsed();
    
    println!("Parallel execution time: {:?}", duration);
    println!("First few results: {:?}", &result[0..5]);
    println!("Last few results: {:?}", &result[result.len()-5..]);
    
    // For comparison, let's also time a sequential version
    let start_seq = std::time::Instant::now();
    let mut result: Vec<i32> = vec![0; a.len()];
    for (a_value,b_value,c_value) in itertools::izip!(&a, &b, result.iter_mut()) {
        *c_value = a_value + b_value;
    }
    let duration_seq = start_seq.elapsed();
    
    println!("Sequential execution time: {:?}", duration_seq);

    println!("First few results: {:?}", &result[0..5]);
    println!("Last few results: {:?}", &result[result.len()-5..]);
    println!("Speedup: {:.2}x", duration_seq.as_nanos() as f64 / duration.as_nanos() as f64);
}
