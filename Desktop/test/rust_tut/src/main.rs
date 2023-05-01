// // use std::collections::HashMap;


// // fn main() {
// //     let mut heroes = HashMap::new();

// //     heroes.insert("Superman", "Clark Kent");
// //     heroes.insert("Batman", "Bruce Wayne");
// //     heroes.insert("Flash", "Barry Allen");

    
// //     for(k,v) in heroes.iter() {
// //         println!("k ->{} \n v -> {}",k, v);
// //     }

// //     if heroes.contains_key(&"Batman"){
// //         let the_batman = heroes.get(&"Batman");

// //         match the_batman {
// //             Some(x) => println!("batman is a hero {}", x),
// //             None => println!("Batman is not a hero"),
// //         }
// //     }
// // }


// fn main() {
//     const PI: f32 = 3.14;


//     trait Shape {
//         fn new(length: f32, width: f32) -> Self;
//         fn area(&self) -> f32;
//     }


//     struct Rectangle {
//         length: f32, width: f32
//     };
//     struct Circle {
//         length: f32, width: f32
//     };



//     //TRAIT nieco ako INTERFACE
//     impl Shape for Rectangle {
//         fn new(length: f32, width: f32) -> Rectangle {
//             return Rectangle{length,width}
//         }
//         fn area(&self) -> f32 {
//             return self.length * self.width
//         }
//     }

//     impl Shape for Circle {
//         fn new(length: f32, width: f32) -> Circle {
//             return Circle{length,width}
//         }
//         fn area(&self) -> f32 {
//             return (self.length /2.0).powf(2.0) * PI
//         }
//     }


//     let rectangle: Rectangle= Shape::new(10.0, 10.0);
//     let circle: Circle = Shape::new(10.0,10.0);

//     println!("Rectangel area: {}", rectangle.area());

// }

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    let top: &str = "Cesnak"; 
    order_food(&top);
}
