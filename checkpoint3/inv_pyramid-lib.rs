pub fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    let mut result = Vec::new();
    if i == 0 {
        return result;
    }
    for n in 1..=i {
        let spaces = " ".repeat(n);
        let chars = v.repeat(n);
        result.push(format!("{}{}", spaces, chars));
    }
    for n in (1..i).rev(){
        let spaces = " ".repeat(n);
        let chars = v.repeat(n);
        result.push(format!("{}{}", spaces, chars));
    }
    result
}


/*
pub fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    let mut result = Vec::new();

    if i == 0 {
        return result;
    }

    // Ascending part (1 to i)
    for n in 1..=i {
        let spaces = " ".repeat(n);
        let chars = v.repeat(n);
        result.push(format!("{}{}", spaces, chars));
    }

    // Descending part (i - 1 down to 1)
    for n in (1..i).rev() {
        let spaces = " ".repeat(n);
        let chars = v.repeat(n);
        result.push(format!("{}{}", spaces, chars));
    }

    result
}
*/