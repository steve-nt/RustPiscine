use luhn_algorithm::is_luhn_formula;

fn main() {
    println!("{}", is_luhn_formula(""));
    println!("{}", is_luhn_formula("1"));
    println!("{}", is_luhn_formula("79927398713"));
    println!("{}", is_luhn_formula("7992 7398 713"));
    println!("{}", is_luhn_formula("1234567890123456"));
}