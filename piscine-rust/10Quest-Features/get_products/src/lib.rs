pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    
    // Return an empty vector for arrays with 0 or 1 elements
    if n <= 1 {
        return vec![];
    }
    
    let mut result = vec![1usize; n];

    // Calculate prefix products
    let mut prefix = 1usize;
    for i in 0..n {
        result[i] = prefix;
        prefix *= arr[i];
    }

    // Calculate suffix products and multiply them with the prefixes
    let mut suffix = 1usize;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= arr[i];
    }

    result
}

/*// pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
//     (0..arr.len())
//         .map(|i| arr.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, &v)| v).product())
//         .collect()
// }

pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 0 {
        return vec![];
    }
    let mut result = vec![1usize; n];

    let mut prefix = 1usize;
    for i in 0..n {
        result[i] = prefix;
        prefix *= arr[i];
    }

    let mut suffix = 1usize;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= arr[i];
    }

    result
}
*/