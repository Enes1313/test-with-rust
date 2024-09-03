#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(c_variadic)]

include!("../bindings/app_example.rs");
mod outer1 {
include!("../mocks/mock_util_example.rs");
}

mod outer2 {
include!("../mocks/mock_lib_example.rs");
}

/*

32 bit : apt-get install gcc-multilib
sudo pip3 install gcovr
cargo install grcov
rustup component add llvm-tools-preview

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

grcov . --binary-path ./target/debug/deps/ -s .. -t html --branch --ignore-not-existing -o target/coverage/html

*/


use mockall::*;
use mockall::{automock, mock, predicate::*};
use mockall_double::double;
/* 
mod outer {
    use mockall::automock;

    #[automock]
    pub mod ffi {
        include!("../bindings/log.rs");
    }
}*/

#[double]
use outer2::ffi;

#[cfg(test)]
mod app_init {
    use super::*;

    #[test]
    fn init__success() {
        
        let mock = ffi::lib_init_context();
        mock.expect().once().return_const(true);

        unsafe {
            app_init();
        }
    }

    // another tests for util_sum
}
