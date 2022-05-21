fn mul(x: &mut u64, y: &u64) {
    *x *= *x * *y;
}

fn my_func2() {
    let mut n = 10;
    let m = 20;
    println!("n = {}, m = {}", n, m);
    mul(&mut n, &m);
    println!("n = {}, m = {}", n, m);
}

fn main() {
    my_func2();
}