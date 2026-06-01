use minesweeper::solve_board;

fn main() {
    println!("{:?}", solve_board(&[]));
    println!("{:?}", solve_board(&[""]));
    println!("{:?}", solve_board(&["***"]));
    println!("{:#?}", solve_board(&["   ", " * ", "   ",]));
    println!("{:#?}", solve_board(&["*  ", "   ", "  *",]));
}