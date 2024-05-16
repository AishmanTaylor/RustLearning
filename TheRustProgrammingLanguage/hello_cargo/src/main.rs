fn main() {
    println!("Hello, world!");
}

/**
 * Cargo basics:
 *  - Cargo is Rusts build system and packagae manager
 *  - Really useful as it handles building your libraries, downloading dependencies, and building out those dependencies
 *  
 * Creating a project with Cargo:
 *  - Type into the terminal: cargo new [project name]
 *  - This will create a directory containing two files: Cargo.toml and src/main.rs
 * 
 * Building and Running Cargo Projects:
 *  - Type into the terminal: cargo build
 *   - If your code is error free, it should build/compile
 * - Type into the terminal: cargo run
 *  - This will run the project
 * - Type into the terminal: cargo check
 *  - Checks if the code compiles but doesn't execute it
 * 
 * With simple projects, Cargo doesn't provide a lot of value over rustc. Once a program grows to need multiple files/dependencies, it is worth it
 */