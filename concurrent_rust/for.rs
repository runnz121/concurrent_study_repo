fn is_even(v: u32) -> bool {
    if v % 2 == 0 {
        true
    } else {
        false
    }
}

fn even_odd() {
    for n in 0..10 {
        println!("{} is {}", n,
            if is_even(n) { "even" } else {"odd"});
    }
}

fn even_odd_loop() {
    let mut n = 0;
    loop {
        println!("{} is {}", n,
            if is_even(n) { "even"} else { "odd" });
        n += 1;
        if n >= 15 {
            break;
        }
    }
}

fn main() {
    // even_odd();
    even_odd_loop();
}