fn app_n(f: fn(u64) -> u64, mut n: u64, mut x: u64) -> u64 {
    loop {
        if n == 0 {
            return x;
        }
        x = f(x);
        n -= 1;
    }
}

fn mul2(x: u64) -> u64 {
    x * 2
}

fn my_func3() {
    println!("app_n(mul2, 4, 3) = {}", app_n(mul2, 4, 3));
}

fn main() {
    my_func3();
}