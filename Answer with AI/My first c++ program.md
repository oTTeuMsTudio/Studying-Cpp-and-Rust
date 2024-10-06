# First C++ Program Explained
To write your first C++ program, you’ll need a few things:

### 1. Text Editor or IDE: 
Choose a text editor like Notepad++ or Sublime Text, or an Integrated Development Environment (IDE) like Code::Blocks, Eclipse, or Visual Studio.

### 2. Compiler: 
Install a C++ compiler like GCC (MINGW) or Visual C++ (VC++).

### 3. Basic Understanding: 
Familiarize yourself with C++ syntax and basic concepts.

## Step-by-Step Guide

1. Open your chosen text editor or IDE and create a new file.
2. Save the file with a .cpp extension, for example, myfirstprogram.cpp.
3. Write the following code:

```c++
#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
```

## Code Explanation

- #include <iostream>: Includes the iostream header file, which provides input/output functions like std::cout.
- int main(): Declares the main function, which is the entry point of your program. It returns an integer value (0 in this case).
- std::cout << "Hello, World!" << std::endl;: Prints the string “Hello, World!” to the console, followed by a newline character.
- return 0;: Returns an integer value of 0 to indicate successful program execution.

## Compiling and Running

1. Compile your code using your chosen compiler. For GCC, open a terminal and navigate to the directory containing your myfirstprogram.cpp file, then run:

g++ myfirstprogram.cpp -o myfirstprogram

This will create an executable file named myfirstprogram.

2. Run the executable:

./myfirstprogram

You should see the output:

Hello, World!

Congratulations! You’ve just written and executed your first C++ program.

### Tips and Resources

- For a more detailed explanation of the code, refer to online resources like W3Schools, Tutorials Point, or Code::Blocks documentation.
- Practice writing simple programs to solidify your understanding of C++ basics.
- Experiment with different compilers and IDEs to find what works best for you.

Remember, this is just the beginning of your C++ journey. Happy coding!

