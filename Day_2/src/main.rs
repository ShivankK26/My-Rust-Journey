fn main() {
    let _my_first_bool = true;
    let _my_second_bool = false;

    // Integers
    let _days_of_week: i8 = 7;
    let _number_of_users: i64 = 128000;
    let _number_of_tokens: u64 = 1000;
    let _just_a_number = 0;

    // Floating Point Number
    let _pi: f32 = 3.14;

    // Characters
    let _my_char: char = 'a';

    // Strings
    let message = "Hey, Shivank";
    let _my_string = String::from("Hey, Shivank");

    // Array
    let _days_in_week: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let _first_element = _days_in_week[0];
    let _last_element = _days_in_week[_days_in_week.len() - 1];

    // Slice
    let slice = &_days_in_week[1..3]; // We get Tuesday and Wednesday as output

    // Tuples
    let person = ("Alice", 30);

    let name = person.0;
    let age = person.1;

    // Unit Type
    let _unit_type = ();

    // Variables
    let mut num = 5;
    num = 6;
}
