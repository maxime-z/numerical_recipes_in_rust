fn main() {
    // println!("Hello, world!");
    double_in_hexadecimal(6.5);
    println!("end");
}

// print double in hexadecimal
union Udoub {
    d: f64,
    c: [char; 8],
}

fn double_in_hexadecimal(x: f64) {
    let x = Udoub { d: x };
    // let c = 'z';

    unsafe {
        for number in x.c {
            print!("{:x}", number as i32)
        }
        println!();
    }
}
