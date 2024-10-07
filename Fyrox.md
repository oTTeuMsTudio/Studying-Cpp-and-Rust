# Install Fyrox on Linux

To install Fyrox on Linux, follow these steps:

### 1. Install Rust: 
Fyrox is written in Rust, so you need to have Rust installed on your system. You can install Rust using the following command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install Rust and its associated tools.

### 2. Clone the Fyrox repository: 
Clone the Fyrox repository from GitHub using Git:

```
git clone https://github.com/FyroxEngine/Fyrox.git
```

This will download the Fyrox source code to a directory named Fyrox.

### 3. Build Fyrox: 
Navigate to the Fyrox directory and build Fyrox using Cargo:

```
cd Fyrox
cargo build --release
```

This will compile Fyrox and its dependencies.

### 4. Run Fyrox: 
After building, you can run Fyrox using the following command:

```
target/release/rusty-editor
```

Note that the executable file does not have an extension on Linux, as executables do not require one.

## Alternatively, use the standalone FyroxEd editor:

If you want to use the standalone FyroxEd editor, you can install it using Cargo:

```
cargo install fyroxed
```

Then, run the editor using:

```
fyroxed
```

### Note: 
The standalone FyroxEd editor only allows scene creation and editing, but not running games within the editor.

### Additional dependencies: 
Depending on your Linux distribution, you might need to install additional dependencies, such as OpenGL or GLX, to run Fyrox. Consult your distribution’s documentation for more information.

That’s it! With these steps, you should be able to install and run Fyrox on your Linux system.
