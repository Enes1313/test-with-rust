#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(c_variadic)]

include!("../bindings/util_example.rs");

/*

32 bit : apt-get install gcc-multilib
cargo install grcov
rustup component add llvm-tools-preview

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

grcov . --binary-path ./target/debug/deps/ -s .. -t html --branch --ignore-not-existing -o target/coverage/html

*/

/*

use mockall::*;
use mockall::{automock, mock, predicate::*};
use mockall_double::double;

mod outer {
    use mockall::automock;

    #[automock]
    pub mod ffi {
        include!("../bindings/log.rs");
    }
}

#[double]
use outer::ffi;

*/

#[cfg(test)]
mod util_sum {
    use super::*;

    #[test]
    fn sum__success() {
        /*
        let mock = func_context();
        mock.expect().once().return_const(());
*/      
        let mut result: ::core::ffi::c_int = 0;

        unsafe {
            result = util_sum(2, 7, 5);
        }

        assert_eq!(result, 12);
    }

    // another tests for util_sum
}

#[cfg(test)]
mod util_mult {
    use super::*;

    #[test]
    fn multiplication__success() {
        /*
        let mock = func_context();
        mock.expect().once().return_const(());
*/      
        let mut b : bool = false;
        let mut out: ::core::ffi::c_int = 0;

        unsafe {
             b = util_mult(5, 7, &mut out);
        }

        assert_eq!(b, true);
        assert_eq!(out, 35);
    }

    // another tests for util_mult
}
