//! A micro crate that simplifies a bit the use of the std macro `thread_local!`.
//!
//! ```
//! extern crate regex;
//!
//! use with_thread_local::with_thread_local;
//! use regex::Regex;
//!
//! let user_input = "cat";
//!
//! let (is_a_pet, needs_a_walk) = with_thread_local! {
//!     static REGEX_PET: Regex = Regex::new(r"cat|dog").unwrap();
//!     static REGEX_WALK: Regex = Regex::new(r"dog").unwrap();
//!
//!     {
//!         let is_a_pet = REGEX_PET.is_match(user_input);
//!         let needs_a_walk = REGEX_WALK.is_match(user_input);
//!
//!         (is_a_pet, needs_a_walk)
//!     }
//! };
//!
//! assert!(is_a_pet && !needs_a_walk);
//! ```
//!
//! You can also use its variant `move` to move variables inside the block. Though I admit I could
//! not write a good example:
//!
//! ```
//! extern crate regex;
//!
//! use with_thread_local::with_thread_local;
//! use regex::Regex;
//!
//! let user_input = vec!["cat", "love", "dog"];
//!
//! let output = with_thread_local! {
//!     static REGEX_PET: Regex = Regex::new(r"cat|dog").unwrap();
//!
//!     move {
//!         user_input
//!             .into_iter()
//!             .filter(|s| REGEX_PET.is_match(s))
//!             .collect::<Vec<_>>()
//!     }
//! };
//!
//! assert_eq!(output, ["cat", "dog"]);
//! ```

#[macro_export]
macro_rules! with_thread_local {
    (
        $(
        static $name:ident : $ty:ty = $init:expr;
        )+

        $block:block
    ) => {{
        thread_local! {
            #[allow(unused_parens)]
            static _THIS_LOCAL: ($($ty),+) = ($($init),+);
        }

        _THIS_LOCAL.with(|#[allow(non_snake_case, unused_parens)] ($($name),+)| $block)
    }};
    (
        $(
        static $name:ident : $ty:ty = $init:expr;
        )+

        move $block:block
    ) => {{
        thread_local! {
            #[allow(unused_parens)]
            static _THIS_LOCAL: ($($ty),+) = ($($init),+);
        }

        _THIS_LOCAL.with(move |#[allow(non_snake_case, unused_parens)] ($($name),+)| $block)
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn use_one() {
        let res = with_thread_local! {
            static ONE: usize = 42;

            {
                *ONE
            }
        };
        assert_eq!(res, 42);
    }

    #[test]
    fn use_two() {
        let res = with_thread_local! {
            static ONE: usize = 42;
            static TWO: usize = 2;

            {
                *ONE + *TWO
            }
        };
        assert_eq!(res, 44);
    }
}
