# Rust Book 🦀

The [Rust Book 🦀](https://doc.rust-lang.org/book/title-page.html), aka The Book, is a 
fantastic resource to help learn more about the Rust programming language.
The Book and I have a bit of an interesting relationship. Rather than read it cover to cover in a weeks time,
I find a lot of utility in reading _chunks_ of the book, attempting to implement some code on my own,
continue self learning, and then later revisit the book. Working through the book's minigrep example was a moment
where I felt like it all started to come together.

## This Repo

This repo is mostly a 1:1 implementation of the
[minigrep](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) example. It's a fantastic little tutorial,
that helps to bridge the complexities of Rust's Type System, Cargo, and the rich features of the language
in a format that makes you quickly "see results". I've taken some liberties in the implementation
of `search`, opting to use some of Rust's iterator features and closures. An existing TODO for this
implementation of minigrep is to refactor the `search_case_insensitive` function to use some of the more
interesting functional features of the language. The current hurdle for me in refactoring this code
lies in the memory ownership.
