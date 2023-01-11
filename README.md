![Rust](https://github.com/cecton/with-thread-local/actions/workflows/rust.yml/badge.svg)
[![Latest Version](https://img.shields.io/crates/v/with-thread-local.svg)](https://crates.io/crates/with-thread-local)
![Rust 1.46+](https://img.shields.io/badge/rust-1.46%2B-orange.svg)
![License](https://img.shields.io/crates/l/with-thread-local)
[![Docs.rs](https://docs.rs/with-thread-local/badge.svg)](https://docs.rs/with-thread-local)
[![LOC](https://tokei.rs/b1/github/cecton/with-thread-local)](https://github.com/cecton/with-thread-local)
[![Dependency Status](https://deps.rs/repo/github/cecton/with-thread-local/status.svg)](https://deps.rs/repo/github/cecton/with-thread-local)

with-thread-local
=================

<!-- cargo-rdme start -->

A micro crate that simplifies a bit the use of the std macro `thread_local!`.

```rust
extern crate regex;

use with_thread_local::with_thread_local;
use regex::Regex;

let user_input = "cat";

let (is_a_pet, needs_a_walk) = with_thread_local! {
    static REGEX_PET: Regex = Regex::new(r"cat|dog").unwrap();
    static REGEX_WALK: Regex = Regex::new(r"dog").unwrap();

    {
        let is_a_pet = REGEX_PET.is_match(user_input);
        let needs_a_walk = REGEX_WALK.is_match(user_input);

        (is_a_pet, needs_a_walk)
    }
};

assert!(is_a_pet && !needs_a_walk);
```

You can also use its variant `move` to move variables inside the block. Though I admit I could
not write a good example:

```rust
extern crate regex;

use with_thread_local::with_thread_local;
use regex::Regex;

let user_input = vec!["cat", "love", "dog"];

let output = with_thread_local! {
    static REGEX_PET: Regex = Regex::new(r"cat|dog").unwrap();

    move {
        user_input
            .into_iter()
            .filter(|s| REGEX_PET.is_match(s))
            .collect::<Vec<_>>()
    }
};

assert_eq!(output, ["cat", "dog"]);
```

<!-- cargo-rdme end -->
