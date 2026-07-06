fn is_leap_buggy_3(year: i32) -> bool {
    year % 4 == 0 && year % 400 == 0
}

fn main() {
    let year = 2023;
    println!("{}", is_leap_buggy_3(year));
}
