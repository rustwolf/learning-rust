pub fn run_infinite_loop() {
    let mut c = 0;
    loop {
        c = c + 1;
        if c > 10 {
            print!("{}", c);
            break;
        }
    }
}
