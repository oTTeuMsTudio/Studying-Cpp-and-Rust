# Getting ready

Before we can do anything useful, we need to set up the Rust development environment. This chapter describes the steps required to install the Rust build toolchain and to install one of these development environments:
- JetBrains RustRover (https://www.jetbrains.com/rust/)
- Visual Studio Code with the RustAnalyzer extension. (https://code.visualstudio.com)

## Installing Rust

There is little value in rehashing the excellent Rust Getting Started Guide: https://www.rust-lang.org/learn/get-started. Please follow the steps on this page to install Rust on your development machine. Return here when you can successfully run: cargo --version.

Good luck!

Adding some tooling
Once Rust is installed, we'll be adding the Clippy lint tool. You'll be seeing a lot of Clippy later. Open a terminal window and perform these steps:

rustup component add clippy
Also add the Rust formatting tool: rustfmt:

cargo install rustfmt
Setting up the development environment
Pick one of the following development environments. Both are excellent choices. The JetBrains RustRover is a full-blown IDE, while Visual Studio Code is a lightweight editor with a lot of extensions. We'll be using RustRover in this book.

JetBrains RustRover
In this book, we'll be using the newly developed RustRover IDE from JetBrains.

Step 1. Visit https://www.jetbrains.com/rust/ and download RustRover for your operating system.

Step 2. Launch RustRover. If there are any projects open, close them until you see the "Welcome to RustRover" screen.

Step 3. Open the Settings page (Cmd+,).

Click on "Languages and Frameworks" in the left-hand menu.
Click on "Rust".
Verify that a Rust toolchain is detected and that the "expand macros" option is checked.
Then expand the "Rust" entry in the left-hand menu and select "Clippy" for the External tool in External Linters.
It is recommended that you enable "Run external linter on the fly". This will slow down your IDE a bit, but it will show you Clippy warnings in real-time. You can always disable this later if it slows down your machine too much.
Then select "Rustfmt" and select "Use Rustfmt instead of the builtin formatter".
Then click on "Configure actions on save..."
Enable "Reformat code" and click "OK".
Your IDE is now ready for use.

Visual Studio Code with Rust Analyzer
Visual Studio Code is a popular editor for Rust development. It comes with a lot of extensions that make Rust development a breeze. The must-have extension for Rust development is the Rust Analyzer. It is a language server that provides code completion, refactoring, and other IDE features.

Step 1. Visit https://code.visualstudio.com and download VSCode for your operating system.

Step 2. Once installed, launch VSCode and open the Command Palette (Cmd+Shift+P) and type 'shell command' to find the Shell Command: Install 'code' command in PATH command.

Step 3. Then open the Extensions tab (Cmd+Shift+X) and install these extensions:

rust-analyzer
crates
GitLens
Better TOML
Cargo.toml Snippets
Error Lens
and if you want: Intellij Keybindings
Step 4. It is highly recommended to also change these settings. Open the Command Palette (Cmd+Shift+P) and type ' settings' and click on 'Preferences: Open Settings (UI)'

Editor: Format on Save: true
Rust-analyzer › Check On Save: Command: clippy in favor of check
Your IDE is now ready for use.

Test the development environment
On the RustRover welcome screen, click on "New Project" and create a new project using the Binary (application) template. This will create a new Cargo project with a main.rs file.

Explore the project and open the src > main.rs file by double-clicking on it. It should look like this:

fn main() {
    println!("Hello, world!");
}
Now run the program by clicking the play-button ▶️ in front of fn main(). Check the output and confirm you see Hello, world!

Exercise
Our first exercise is to change the program to print your name instead of "world". Press the play-button again and confirm that the output has changed.

This concludes the "Getting Ready" steps. See you in class!

