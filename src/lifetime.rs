/// Gets the highest of two numbers
/// Uses match statement than the ususal if condtion
fn get_highest<'a, 'b: 'a>(first: &'a i8, second: &'b i8) -> &'a i8 {
    match first >= second {
        true => return first,
        false => return second,
    }
}

pub(crate) fn lifetime() {
    let first: i8 = 10;
    let third;
    {
        let second: i8 = 20;
        third = get_highest(&first, &second);
        println!("Third {} ", third);
    }
}
