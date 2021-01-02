fn main() {
    println!("{}", add(1,2));

}

fn add(a: isize, b: isize)->isize {

    // for early returns: use return statement.
    if b == 0 {
        return a;
    } else if a == 0 {
        return b;
    }

    // return should be at last of the function.
    // without "semicolon"
    a + b
}
