fn main() {
    let mut number = 0;

    let foo = &number;

    // E0505: cannot assign to `number` because it is borrowed
    number = 42;

    println!("{foo}");
}
