pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut sums = Vec::with_capacity(arr.len() + 1);
    sums.push(0);
    let mut current_sum = 0;

    for &num in arr {
        current_sum = current_sum + num;
        sums.push(current_sum);
    }
    sums.reverse();
    sums
}

/*
pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut sums = Vec::with_capacity(arr.len() + 1);
    
    // The sum of the empty array is always 0
    sums.push(0);
    
    let mut current_sum = 0;
    
    // Accumulate the sums from left to right
    for &num in arr {
        current_sum += num;
        sums.push(current_sum);
    }
    
    // Reverse the vector to match the expected descending pattern
    sums.reverse();
    
    sums
}

*/