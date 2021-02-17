const DEGREE_SYMBOL: &str = "º";

fn to_centigrade(degree: f32) -> f32 {
    return ((degree - 32.0) / 9.0) * 5.0;
}

fn to_fahrenheit(degree: f32) -> f32 {
    return ((degree / 5.0) * 9.0) + 32.0;
}

fn degree_string(degree: f32) -> String {
    return format!("{:2.2}{}", degree, DEGREE_SYMBOL);
}

// function returns statement rather than explicit return
fn is_greater_than(arg1: i32, arg2: i32) -> bool {
    arg1 > arg2
}

fn main() {
    println!("Hello, world!");

    let var = 10;

    if var < 10 {
        println!("var is {}", var);
    }

    // range iterator with reverse
    for x in (0..10).rev() {
        println!("x={}", x);
    }

    // declare array and then iterate through it
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("element is {}", *element);
    }

    for fahrenheit  in [-40.0, 0.0, 98.4, 212.0].iter() {
        println!("{}F is {}C", degree_string(*fahrenheit), degree_string(to_centigrade(*fahrenheit)));
    }

    let centigrade: f32 = 100.0;
    println!("{}C is {}F", degree_string(centigrade), degree_string(to_fahrenheit(centigrade)));

    for x in 0..10 {
        if is_greater_than(x, 5) {
            println!("{} > 5", x);
        }
    }
}
