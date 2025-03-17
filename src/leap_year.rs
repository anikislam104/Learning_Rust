pub fn main(){
    let mut is_leap_year = false;
    // take input from user
    println!("Enter a year: ");
    let mut year = String::new();

    std::io::stdin().read_line(&mut year).expect("Failed to read line");
    let year: i32 = year.trim().parse().expect("Please type a number!");

    if year % 400 == 0
    {
        is_leap_year = true;
    }
    else if year % 100 == 0
    {
        is_leap_year = false;
    }
    else if year % 4 == 0
    {
        is_leap_year = true;
    }
    else
    {
        is_leap_year = false;
    }

    if is_leap_year
    {
        println!("{} is a leap year", year)
    }
    else
    {
        println!("{} is not a leap year", year)
    }
}