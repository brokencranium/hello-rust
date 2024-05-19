pub(crate) fn ownership() -> () {
let one:String = String::from("one");
print(&one);// borrowed reference 
println!(" {} ", one);
let two:String = one + " two ";
println!("{}", two);
}

fn print(input_string:&String) {
    println!(" {} ", input_string);
    test();
}

fn test() { 
    let one:String = String::from("one");
    let two:String = one + " two ";
    println!("{}", two);

}