//! This bin folder can be used to try the examples out located in the examples directory.
//!
//! All you need to do is:
//!
//! - Download the crossterm source code.
//! - Add this in the Cargo.toml file:
//!   ``` [[bin]]
//!        name = "example_bin"
//!        path = "./examples/bin.rs"
//!   ```
//!   
//! - Run program with: `cargo run`
extern crate crossterm;

use crossterm::Context;


// mod terminal;
// mod color;
// mod cursor;
// mod crossterm_type;
use crossterm::raw::IntoRawMode;
use std::{thread, time};
use std::io::Read;

fn main()
{


//    let mut rv = String::new();
//    {
//        let alternate = ::crossterm::screen::AlternateScreen::from(context.clone());
//        alternate.into_raw_mode(context.clone());

//        thread::spawn(|| {
//            let context = Context::new();
//            let input = ::crossterm::input::input(&context);
//            let result = input.read_async().unwrap();
//            println!("input: {:?}",result);
//        });

        let context = Context::new();
        let input = ::crossterm::input::input(&context);
        let mut stdin = input.read_until_async(b'\r' as u8).bytes();

        for i in 0..100
        {
            let a = stdin.next();

            println!("input: {:?} exptected: {:?}", a,b'\r');

            if let Some(Ok(b'q')) = a {
                break;
            }

            thread::sleep(time::Duration::from_millis(50));
//            println!("Some data {:?}", b)
        }



//        ::std::io::stdin().read_line(&mut rv);
//        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
//        rv.truncate(len);


//    }

}


