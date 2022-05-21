fn hello(v: u32) {
    println!("hello world: v = {}", v);
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn my_func1() {
    let n = add(10, 5);
    hello(n);
}

fn main() {
    my_func1();
}