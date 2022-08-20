fn main() {
    let x = 4;
    let y = [1, 2, 3, 4];
    let z = &y[0..3];
    for i in y.iter() {
        println!("{}", i)
    }
    println!("x is {}", x);
}
