fn main() {
    println!("Hello World!");
}

/**
 * Anatomy of a Rust program:
 *  - This file contains one function, main
 *  - main functions are special within Rust programs, main is always ran first
 *  - main() defines a function with no paramters and returns nothing
 *   - If there were paramters for main to work, they would be defined in the ()
 *  - Functions are wrapped with {} as Rust requires curly braces around all function bodies
 *   - Good practice to open curly braces on the same line functions are declared with closing on their own line
 *   - running rustfmt in the command line on the file is a good way to automatically refactor code to standard style
 *  - Each layer of indentation within a Rust program is done with four spaces, not tabs
 *  - println! is a Rust macro
 *   - If it had called a function, it would be written as println
 *   - ! calls a macro, without calls a function. Macros do not always follow function rules
 *  - "Hello World!" is a string which is passed into the macro, println!
 *  - Semi colons end lines in Rust programs
 * 
 * Compiling and Running:
 *  - Before running a Rust program you have to compile it
 *   - To do so, type rustc [file name].rs into the terminal
 *  - To run file type ./[file name]
 */