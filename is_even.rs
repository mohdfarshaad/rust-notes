fn main() {
    let ans : bool= is_even(21);
    println!("{}", ans)
}

fn is_even(num:u32)-> bool{
    if num % 2 == 0 {
        return true;
    }else {
        return false
    }

}
