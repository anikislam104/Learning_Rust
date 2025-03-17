mod leap_year;
mod print_first_n;
fn main() {
    // let num:i8 = -8;
    // println!("{}", num);
    // println!("{}", num + 1);
    // println!("{}", num - 1);
    // // print!(num);
    // let num2:u32 = 1000;
    // let float_num:f32 = 3.14;
    // println!("num2: {}, float_num: {}", num2, float_num);

    leap_year::check_leap_year();

    print_first_n::print_numbers(10);
}
