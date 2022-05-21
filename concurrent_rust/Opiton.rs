/**
option 타입은 실패하는 함수가 있을 때 성공하면 some 안에 값을 포함시키고
실패하면 None을 반환하는데 이용됨
**/

fn pred(v: u32) -> Option<u32> {
    if v== 0 {
        None
    } else {
        Some(v-1)
    }
}

fn print_pred(v: u32) {
    match pred(v) {
        Some(w)=> {
            println! ("pred({}) = {}", v, w);
        }
        None => {
            println!("pred({}) is undefined", v);
        }
    }
}


fn main() {
    print_pred(40);
}