/*

pub fn search(array: &[i32], key: i32) -> Option<usize> { 
    for (index, &value) in array.iter().enumerate().rev() {
        if value == key {
            return Some(index);
        }
    }
    None
}

    */

pub fn search(array: &[i32], key: i32) -> Option<usize> {

    array.iter().rposition(|&x| x == key)
}