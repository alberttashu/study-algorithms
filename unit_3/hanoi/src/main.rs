fn main() {
    let disks = 10;
    let peg_a = "A";
    let peg_b = "B";
    let peg_c = "C";
    println!("Tower of Hanoi with {disks} disks from {peg_a} to {peg_b}.");
    solve_hanoi(disks, peg_a, peg_b, peg_c);
}

fn solve_hanoi(disks: u32, from: &str, to: &str, spare: &str) {
    match disks {
        0 => (),
        1 => print_move(from, to),
        _ => {
            solve_hanoi(disks - 1, from, spare, to);
            print_move(from, to);
            solve_hanoi(disks - 1, spare, to, from);
        }
    }
}

fn print_move(from: &str, to: &str) {
    println!("  {}  =>  {}", from, to);
}
