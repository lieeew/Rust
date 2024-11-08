use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // const
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // use Shadowing cause  compile-time error:
    // let mut space = "       ";
    // space = space.len();

    // it works fine
    let space = "      ";
    let space = space.len();

    // Date type
    let number: i8 = -128;
    let number: u8 = 0;  // count be negative number
    println!("The value of number is: {number} and the space: {space}");

    let x: u8 = 255;
    let wrapped = x.wrapping_add(1);  // 结果是 0，因为 255 + 1 在 u8 范围内溢出
    println!("wrapped_add result: {}", wrapped);  // 输出: wrapped_add result: 0

    let x: u8 = 255;
    let checked = x.checked_add(1); // 结果是 None，因为发生了溢出
    println!("checked_add result: {:?}", checked);  // 输出: checked_add result: None

    let x: u8 = 255;
    let (result, overflowed) = x.overflowing_add(1);  // 结果是 (0, true)
    println!("overflowing_add result: {}, overflowed: {}", result, overflowed);  // 输出: overflowing_add result: 0, overflowed: true

    let x: u8 = 255;
    let saturated = x.saturating_add(1);  // 结果是 255，因为发生溢出时钉在了 u8 的最大值
    println!("saturating_add result: {}", saturated);  // 输出: saturating_add result: 255

    // Float
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let number = 98_222;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // The Boolean Type
    let t = true;
    let f: bool = false;

    // Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    another_function();

    let i = five();
    println!("The value of i is: {i}");
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    return x + 1;
}

fn another_function2() {
    let mut str = String::new();

    let value = io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
}

fn for_collector() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn for_element() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

}

// 测试模块
#[cfg(test)]
mod tests {
    // 使用 super::* 导入外部函数
    // use super::*;

    use crate::{for_collector, for_element, multiple_loops, plus_two};

    #[test]
    fn function_test() {
        multiple_loops();
        for_collector();
        for_element();
    }

    #[test]
    fn function_test_1() {
        // another_function(111);
        // print_labeled_measurement(50,'A');
        // expressions_evaluate();
        // let i = plus_two(12);
        // println!("plus two : {}", i);
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");

        loops();
    }

    fn loops() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }
    fn expressions_evaluate() {
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }
    fn another_function(x: i32) {
        println!("The value of x is: {x}");
    }

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value} {unit_label}");
    }

    #[test]
    fn float() {
        // Float
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32

    }

    #[test]
    fn compound_type() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of x is: {x}  y is: {y}");

        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;
        println!("The value of five_hundred is: {five_hundred} six_point_four is {six_point_four} one is {one}");

        let a = [1, 2, 3, 4, 5];

        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];

        let a: [i32; 5] = [3; 5];

        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
    }

    #[test]
    fn test_operators() {
        // addition
        let sum = 5 + 10;
        println!("Sum is {}", sum);

        // subtraction
        let difference = 95.5 - 4.3;
        println!("Difference is {}", difference);

        // multiplication
        let product = 4 * 30;
        println!("Product is {}", product);

        // division
        let quotient = 56.7 / 32.2;
        println!("quotient is {}", quotient);

        let truncated = -5 / 3; // Results in -1
        println!("Truncated is {}", truncated);

        // remainder
        let remainder = 43 % 5;
        println!("Remainder is {}", remainder);
    }
}