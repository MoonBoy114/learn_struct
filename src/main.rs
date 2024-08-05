use std::{hash::RandomState, os::windows};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 12,
        height:15,
    };
    let rect3 = Rectangle {
        width: 60,
        height:65,
    };
    let rect4 = Rectangle::square(3);

    println!("Может ли rect1 содержать в себе rect2?: {}",rect1.can_hold(&rect2));
    println!("Может ли rect1 содержать в себе rect3?: {}", rect1.can_hold(&rect3));


    // println!("{} квадратных пикселей", area(&rect1));
}
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }




// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active:bool,
// }
// fn build_User(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         sign_in_count: 1,
//         active: true,
//     }
// }
// struct Color(i32, i32,i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let mut user1 = User {
//         email: String::from("some_email@mail.com"),
//         username: String::from("Bolly"),
//         sign_in_count: 1,
//         active: true,
//     };
//     user1.email = String::from("bolly@mail.ru");
//     let mut user2 = User {
//         email: String::from("something@mail.ru"),
//         username: String::from("Hally"),
//         // sign_in_count: user1.sign_in_count,
//         // active: user1.active,
//         ..user1
//     };
//     let black = Color(0,0,0);
//     let point = Point(32,12,10);
// }