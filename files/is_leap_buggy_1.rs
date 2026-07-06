fn is_leap_buggy_1(year: i32) -> bool {
    year % 4 == 0 && year % 100 != 0
}

fn main() {
    let year = 2023;
    println!("{}", is_leap_buggy_1(year));
}
