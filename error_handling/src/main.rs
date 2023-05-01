// // use std::fs::File;
// // use std::io::Write;
// // use std::io::ErrorKind;
// // fn main() {
// //     let path = "lines.txt";
// //     let outputt = File::create(path);

// //     let mut output = match outputt {
// //         Ok(file) => file,
// //         Err(error) => {
// //             panic!("Problem creating file: {:?}", error);
// //         }
// //     };

// //     write!(output, "Just some text").expect("Something went wrong");

// //     let output2 = File::create("second.txt");
// //     let mut output3 = match output2 {
// //         Ok(file) => file,
// //         Err(er) => match er.kind() {
// //             ErrorKind::NotFound => match File::create("rand.txt") {
// //                 Ok(file) => file,
// //                 Err(e) => panic!("Canct create the file {:?}", e),
// //             }
// //             _err => panic!("Problem with the file")
// //         },
        
// //     };
// //     write!(output3, "MegaTEXT").expect("Something went wrong");
// // }

// fn main() {
//     fn use_func<T>(a: i32, b: i32, func: T) -> i32 
//     where T: Fn(i32, i32) -> i32 {
//         func(a, b)
//     }

//     let sum = |a,b| a+b;
//     let prod = |a,b| a*b;

//     println!("5 + 4 is = {}", use_func(5,4,sum));
//     println!("5 * 4 is = {}", use_func(5,4,prod));


// }
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc,Mutex};

fn main() {
    pub struct Bank {
        balance: f32
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>,amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current balance: {} Withdrawal a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew: {}", bank_ref.balance);
        }
    }

    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank{balance: 20.00}));
    let handles = (0..10).map(|_|{
        let bank_ref = bank.clone();
        thread::spawn(||{
            customer(&bank_ref)
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);

    
}