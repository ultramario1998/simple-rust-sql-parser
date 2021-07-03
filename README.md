# Overview

This is a simple program that takes in text, removes everything except for math operators and numbers, and splits the string at the operators.

I wanted to try something fairly simple that could transition into something marginally more useful. Parsing text is always something that needs to happen with any language, so I decided I wanted to learn more about that too.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

I used the regex crate to parse text against regex, and std::io to get user input.

# Useful Websites

* [The Rust Programming Language book](https://doc.rust-lang.org/book/title-page.html)
* [Crate regex documentation](https://docs.rs/regex/1.5.4/regex/index.htmle)

# Future Work

* I want to be able to evaluate the passed string and spit out a result at the end.