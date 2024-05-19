macro_rules! capitalize {
    ($a: expr) => {
        let mut value: Vec<char> = $a.chars().collect();
        value[0] = value[0].to_uppercase().nth(0).unwrap();
        $a = value.into_iter().collect();
    };
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i8,
    y: i8,
}

fn print_point(point: Point) {
    println!("X: {} Y: {}", point.x, point.y);
}

pub(crate) fn macros() {
    let mut value = String::from("woohoo!");
    capitalize!(value);
    print!("{}", value);

    let point = Point { x: 1, y: 2 };
    print_point(point);
    println!("{:#?}", point);
}
