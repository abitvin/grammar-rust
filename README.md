Grammer
=======

About
-----
The Grammer API is a parser combinator written in Rust which offers infinite look ahead scanning and were the rules are build using a regexp-like language.
You parse a UTF-8 string into a generic T. Where T is a custom defined data structure for example an AST or a primitive f64. 

You can use Grammer for:
* Creating a programming language syntax and parse an AST out of it.
* Making different parsers for different Unicode text based file formats.
* Writing a calculator with correct operator precedence with a few lines of code.
* A text comparer, like a regexp alternative.
* Much more...

For more examples of grammer you can look in the TypeScript version also available in my GitHub account.

License
-------
This project is licensed under the MIT license.