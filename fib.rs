fn main() {
    println!("{}", fib(5));
}


fn fib(num:u32)-> u32{

    let mut first : u32 = 0; // mutable(non-constant) variable - default immutable(constant)
    let mut second : u32= 1;

    if num == 0 {
        return  first;
    }

    if num == 1 {
        return  second;
    }

    for _ in 0..(num - 1){ // using _ because i is unused
        let temp: u32 = second;
        second = second + first;
        first = temp 
    }

    return  second;

}
