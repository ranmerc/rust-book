fn main() {
    let n = 10;

    let mut a = 0;
    let mut b = 1;

    print!("{a} {b}");

    for _ in 0..n {
        let next = a + b;
        print!(" {next}");

        a = b;
        b = next;
    }

    println!("")
}
