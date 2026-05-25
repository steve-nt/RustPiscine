use scytale_decoder::*;

fn main() {
    println!(
        "\"sec yCtoadle\" size=2 -> {:?}",
        scytale_decoder("sec yCtoadle".to_owned(), 2)
    );

    println!(
        "\"steoca dylCe\" size=4 -> {:?}",
        scytale_decoder("steoca dylCe".to_owned(), 4)
    );
}
/* */
And its output
$ cargo run
"sec yCtoadle" size=2 -> Some("scytale Code")
"steoca dylCe" size=4 -> Some("scytale Code")
$
*/