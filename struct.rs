

// Structs in Rust 
/*
    Structs are data structures used to store multiple variables. its similar to struct in c or c++. Structs not only used to just define a model type its more of a standlone structure similar to class in javascript 
*/


/*
    Lets create a user struct for storing user user data
*/
struct User{
    username:String,
    email:String,
    password:String,
    otp:u8,
    is_active:bool
}
   
/*
    Implementing Structs
*/

// Lets create a Rectancle structure

struct Rectancle{
    height:u32,
    width:u32,
}

// Now we can add the methods to this like a class in javascript have its own methods

impl Rectancle {
    fn area(&self) -> u32{
         self.width * self.height
    }

    /*
        💡 We can also add a argument for with using it 
           like _num : u32 where _ indicates best practice to show then unused
     */
    fn perimeter(&self, _num:u32 )->u32{
          2 * (self.height + self.width) 
    }

    fn debug() -> i32{
        return 1;
    }
}

fn main(){
    // Lets create an object of an user using the Strcut User
    let user = User{
    username:String::from("Farshad"),
    email:String::from("farshad@gmail.com"),
    password:String::from("1234"),
    otp:123,
    is_active:true,
   };

   println!("Username :{}" , user.username);
   println!("Email :{}" , user.email);
   println!("Password :{}" , user.password);
   println!("OTP : {}" , user.otp);
   println!("Active :{}" , user.is_active);

    let rect1 = Rectancle{
        height:20,
        width:30,
    };

    println!("Area of the rectangle : {}", rect1.area());
    println!("Perimeter of the rectangle : {}", rect1.perimeter(1));
    /*
    The debug function cant be called from object of rect which is rect1. It can be called using the Struct Rectangle itself like above

    ❌ println!("Debug : {}", rect1.debug())
*/
    println!("Debug : {}", Rectancle::debug()); 
}
