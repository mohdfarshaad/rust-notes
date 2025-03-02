fn main() {
    let name:String  = String::from("Hello");
    println!("String lenght is {}", get_string_length(name));
}

fn get_string_length(str:String) -> usize{
    str.chars().count()  // implicit return  
}
