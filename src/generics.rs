fn add<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    return x + y;
}

pub(crate) fn generics() {
    println!("{}", add(3, 4));
}
