macro_rules! capitalize {
    ($a: expr) => {
        let mut value: Vec<char> = $a.chars().collect();
        value[0] = value[0].to_uppercase().nth(0).unwrap();
        $a = value.into_iter().collect();
    };
}

pub(crate) fn macros() {
    let mut value = String::from("woohoo!");
    capitalize!(value);
    print!("{}", value);
}
