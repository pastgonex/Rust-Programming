/*
 * @Author: Binqi Ni
 * @Date: 2021-09-07 02:06:58
 * @LastEditTime: 2021-09-14 00:17:32
 * @LastEditors: Binqi Ni
 * @FilePath: /Rust-Programming/rust-learning/src/bin/03_function_as_parameter.rs
 */
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}
fn square(value: i32) -> i32 {
    value * value
}
fn cube(value: i32) -> i32 {
    value * value * value
}

fn main() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
}