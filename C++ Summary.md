# C++ Programming Summary/Overview

## Object-Oriented Programming

> ### This section was written in conjunction with ChatGPT.

Object-Oriented Programming (**OOP**) is a programming paradigm that organizes code around objects, which are instances of classes. It focuses on the concept of objects, their properties (attributes), and behaviors (methods), allowing for modular, reusable, and structured code design.

## Encapsulation
Encapsulation is the practice of bundling data and the methods that operate on that data within a single unit, which is typically a class. It promotes data hiding and information hiding, ensuring that the internal state and implementation details of an object are not directly accessible from outside the object. Encapsulation helps achieve data integrity, security, and abstraction by controlling access through well-defined interfaces.

## Data Hiding
Data hiding is a principle closely related to encapsulation. It involves concealing the internal implementation details of an object and exposing only the necessary information through well-defined interfaces. By hiding implementation details, the object's data is protected and can only be accessed or modified through controlled methods. This enhances data security, code maintainability, and reduces the risk of unintended modifications or access to critical data.

## Inheritance

Inheritance is a mechanism in OOP that allows new classes (derived classes or subclasses) to inherit properties and behaviors from existing classes (base classes or superclasses). Inheritance promotes code reuse, as derived classes can inherit and extend the functionality of their base classes. The derived classes can add new attributes and behaviors or override existing ones to customize the behavior of inherited members. Inheritance facilitates code organization, modularity, and the creation of hierarchical relationships between classes.

## Polymorphism

Polymorphism is a fundamental concept in object-oriented programming (OOP) that allows objects of different classes to be treated as objects of a common base class. It enables you to write code that can work with objects of multiple types in a uniform manner.

Polymorphism is often illustrated through inheritance, where you have a base class and multiple derived classes that inherit from it. The derived classes can override or extend the behavior of the base class's methods, while still adhering to the common interface.

> ### This section was written in conjunction with Leo, Brave`s build-in AI.

Object-Oriented Programming (**OOP**) is a programming paradigm that revolves around the concept of “objects” and their interactions. In C++, OOP is achieved through the use of classes, objects, inheritance, polymorphism, and encapsulation.

### Key Concepts:

1. **Classes**: A class is a blueprint or a template that defines the properties and behavior of an object. It’s a user-defined data type that encapsulates data and functions that operate on that data.
2. **Objects**: An object is an instance of a class, with its own set of attributes (data) and methods (functions). Objects have their own memory space and can be manipulated independently.
3. **Inheritance**: Inheritance allows a derived class to inherit the properties and behavior of a base class. This enables code reuse and facilitates a hierarchical organization of classes.
4. **Polymorphism**: Polymorphism is the ability of an object to take on multiple forms. In C++, this is achieved through function overloading (multiple functions with the same name but different parameters) and function overriding (a derived class provides a different implementation of a function already defined in its base class).
5. **Encapsulation**: Encapsulation is the concept of hiding an object’s internal state and behavior from the outside world, while exposing only a controlled interface through which other objects can interact with it.

### C++ OOP Features:

1. **Constructors**: Special member functions that initialize objects when they’re created.
2. **Destructors**: Special member functions that clean up resources when objects are destroyed.
3. **Access Specifiers**: public, private, and protected keywords control access to class members.
4. **Virtual Functions**: Allow for dynamic method dispatching and polymorphism.
5. **Operator Overloading**: Enables the use of operators (e.g., +, -, ==) with user-defined classes.

## C++ OOP Examples:

**Simple Class**: Define a `Car` class with attributes brand, model, and mileage, and methods `startEngine()` and `accelerate()`.
```c++
class Car {
public:
    string brand;
    string model;
    int mileage;

    void startEngine() { cout << "Vroom!" << endl; }
    void accelerate() { mileage++; }
};
```
**Inheritance**: Create a `ElectricCar` class that inherits from Car and adds a `batteryLevel` attribute and a `charge()` method.
```c++
class ElectricCar : public Car {
public:
    int batteryLevel;

    void charge() { batteryLevel += 10; }
};
```
**Polymorphism**: Define a Vehicle class with a virtual `drive()` method, and have Car and `ElectricCar` classes override this method.
```c++
class Vehicle {
public:
    virtual void drive() { cout << "Driving..." << endl; }
};

class Car : public Vehicle {
public:
    void drive() { cout << "Driving with gasoline..." << endl; }
};

class ElectricCar : public Vehicle {
public:
    void drive() { cout << "Driving with electricity..." << endl; }
};
```
### Best Practices:

1. Follow the Single Responsibility Principle (SRP): Each class should have a single, well-defined responsibility.
2. Use encapsulation to hide internal implementation details.
3. Favor composition over inheritance: Instead of inheriting from a base class, consider composing objects from smaller, independent components.
4. Use polymorphism to write flexible and reusable code.

Resources:

“Object-Oriented Programming in C++” by Robert Lafore (book)
GeeksforGeeks - Object-Oriented Programming in C++
C++ documentation - Classes and Objects
Remember to practice and experiment with different OOP concepts in C++ to solidify your understanding.

## Syntax and Structure

> ### This section was written in conjunction with ChatGPT.

Syntax refers to the set of rules that define the structure, format and grammar of a programming language. It dictates how statements and expressions should be written to form valid code.

C++ follows a structured syntax that includes elements such as keywords1, identifiers, operators and control structures. The syntax is designed to provide precise instructions to the compiler on how to interpret and execute the code.

### Weak vs Strong typing
Weak and strong typing refer to different approaches in how programming languages handle data types and type safety.

In C++, the language is considered strongly typed, as it requires explicit type conversions and does not perform implicit type coercion without the programmer's explicit instruction (except number data types). C++ enforces strong typing to ensure type safety and minimize potential errors.

Weak Typing (Python2 code):
```c++
a = 5 # Compiled! Because Python is a weak typing language.
```
Strong Typing (C++ code):
```c++
a = 5; // Error!
int a = 5; // Compiled!
```
### Semicolons in C++
In C++, a semicolon (;) is used to mark the end of a statement. It serves as a delimiter, indicating to the compiler that one statement has finished and another begins. The presence of semicolons allows the compiler to separate statements and interpret code correctly.

The requirement for semicolons in C++ is a design choice that provides explicit statement termination. This approach allows for more fine-grained control over program execution and eliminates ambiguity.

In contrast, languages like Python2 use indentation to define blocks of code, eliminating the need for explicit statement termination with semicolons.
```c++
int a = 5; // Compiled!
int b = 5 // Error! Semicolon missing.
```

### Curly Braces in C++
C++ uses curly braces ({}) as block delimiters to enclose multiple statements or define the body of control structures, functions, and classes. The use of curly braces provides a clear and explicit way to define the boundaries of code blocks (also know as a scope).

Curly braces help define the scope of variables and maintain code readability. They ensure that statements within the braces are treated as a single unit, making it easier to understand the flow and logic of the program.
```c++
class Car
{

};
namespace MyNamespace
{

}
void MyFunction()
{
    {
        // Scope inside a function
    }
}
```
### Comments in C++
Both single-line and multi-line comments are helpful for adding explanatory notes, documenting code, or temporarily disabling sections of code during debugging or development. They enhance code readability, facilitate collaboration, and provide valuable information to developers maintaining the codebase.

#### Single-line comments
Single-line comments start with two forward slashes // and continue until the end of the line.

They are typically used for brief comments or explanations on a single line.
```
// This is a single-line comment in C++
```

#### Multi-line comments
Multi-line comments, also known as block comments, start with a forward slash (/) followed by an asterisk (*) and end with an asterisk (*) followed by a forward slash (/).

Multi-line comments can span multiple lines and are used for more extensive comments or documentation.
```
/*
    This is a multi-line comment
    It can span multiple lines
*/
```

### Headers vs source files
In C++, header files and source files are two types of files used to organize and manage code in a C++ program.

#### Header Files (.h)
- Header files contain declarations of classes, functions, variables, and other elements that are used in the program.
- They provide interfaces to the functionality implemented in the corresponding source files.
- Header files are included in source files using #include directives to make the declarations visible to the compiler during the compilation process.

#### Source Files (.cpp)
- Source files contain the actual implementations of the functions and classes declared in the header files.
- They define how the functions and classes behave and provide the logic for the program's functionality.
- Source files are compiled to object files and then linked together to create the final executable.

#### Reason for Separate Header and Source Files
The separation of header and source files is a design choice that promotes modularity and improves build efficiency in C++. By keeping declarations in header files and implementations in source files, the compiler can easily check for correctness and compile only the necessary code, reducing build times and preventing redundant compilation.

#### History of Single File Extensions

In the early days of computing, languages like Fortran and COBOL used single file extensions because of the limitations of the operating systems and compilers at the time. Each file had to adhere to a specific format defined by the language and its compiler, and the extension represented that format.

Other languages, like C#3, Java4, and Python2, continued to use single file extensions because they adopted a more integrated approach to handling both declarations and implementations within a single file.

In modern programming, the choice of using single file extensions or separate header and source files depends on the language's design philosophy and the needs of the development community. Both approaches have their strengths and weaknesses, and different languages adopt the one that best aligns with their goals and use cases.

#### Includes
In C++, the include directive is used to bring external code (headers or libraries) into your source code. It allows you to access the declarations and definitions present in those files.

The include directive is typically written as:

#include "filename.h"   // Using double quotes for user-defined headers
#include <filename.h>   // Using angular brackets for standard library headers
Here's the difference between using double quotes and angular brackets:

1. **Double Quotes (`filename.h`**): When you use double quotes, the preprocessor searches for the header file in the current directory first. If it doesn't find the file there, it will look in the additional include directories specified in the project settings.

Example: `#include "MyHeader.h"`

2. **Angular Brackets (`<filename.h>`)**: When you use angular brackets, the preprocessor only searches for the header file in the standard library directories specified for the compiler.

Example: `#include <iostream>`

In general, you use double quotes for your own header files (which may be part of your project) and angular brackets for standard library headers (like iostream, vector, etc.) or headers from external libraries.

> ### This section was written in conjunction with Leo, Brave`s build-in AI.
C++ syntax refers to the rules that govern the structure of C++ programs, including:

1. **Indentation**: C++ uses indentation to denote block-level structure, such as if-else statements, loops, and functions.
2. **Keywords**: C++ has a set of reserved keywords, such as if, else, for, while, switch, case, etc., which have specific meanings.
3. **Identifiers**: C++ uses identifiers (names) for variables, functions, and labels, which must start with a letter or underscore and can be followed by letters, digits, and underscores.
4. **Operators**: C++ has various operators for arithmetic, comparison, logical, assignment, and other operations.
5. **Brackets**: C++ uses parentheses () for function calls, and square brackets [] for array indexing.
6. **Semicolons**: C++ uses semicolons ; to terminate statements.

## Structure in C++

C++ structure refers to the organization of code into logical units, including:

1. **Functions**: C++ functions are self-contained blocks of code that perform specific tasks, returning values or modifying external state.
2. **Classes**: C++ classes define user-defined data types, combining data members (variables) and member functions (methods).
3. **Structures**: C++ structures (typedef’d as struct) are similar to classes, but without inheritance and with default public access.
4. **Enums**: C++ enumerations (typedef’d as enum) define a set of named constants.
5. **Unions**: C++ unions define a type that can hold values of different data types, occupying the same memory space.
6. **Arrays**: C++ arrays store multiple values of the same data type, indexed by integers.
7. **Pointers**: C++ pointers store memory addresses, allowing indirect access to variables.

## Best Practices

1. **Consistent indentation**: Use a consistent number of spaces for indentation throughout the code.
2. **Proper naming**: Use meaningful and descriptive names for variables, functions, and classes.
3. **Code organization**: Structure code into logical modules, functions, and classes.
4. **Commenting**: Use comments to explain code intent, assumptions, and limitations.
5. **Error handling**: Implement robust error handling mechanisms to detect and recover from errors.

## Example Code

Here’s a simple C++ program demonstrating syntax and structure:
```c++
#include <iostream>

// Structure to represent a person
struct Person {
    std::string name;
    int age;
};

// Function to print person information
void printPerson(const Person& p) {
    std::cout << "Name: " << p.name << ", Age: " << p.age << std::endl;
}

int main() {
    // Create a Person structure instance
    Person john = {"John", 30};

    // Call the printPerson function
    printPerson(john);

    return 0;
}
```
This code demonstrates:

- Structure declaration (struct Person)
- Function definition (void printPerson)
- Variable declaration and initialization (Person john)
- Function call (printPerson(john))

Note that this is a simplified example, and real-world C++ programs typically involve more complex structures, functions, and error handling mechanisms.

# Standard Library
This section was written in conjunction with ChatGPT.
The standard library in C++ is a collection of pre-defined classes and functions that provide a wide range of functionality for common tasks. It is a part of the C++ Standard Template Library (STL) and is officially known as the C++ Standard Library. The library is designed to be platform-independent and provides a standardized set of features that are supported across different C++ compilers and environments.

The C++ Standard Library is organized into several header files, each of which contains declarations for specific classes and functions. Some of the key components of the standard library include containers (like vectors, lists, maps, etc.), algorithms (sorting, searching, etc.), iterators, input/output operations, strings, and more.

To use the standard library in C++, you include the appropriate header files in your code, and then you can directly use the classes and functions provided by the library. For example, to use the std::vector class, you include the <vector> header file and then create instances of the vector and use its methods.

The name "std" comes from the fact that all the classes, functions, and other elements of the standard library are part of the std namespace. The namespace std is used to avoid naming conflicts with other libraries and user-defined code. By using the std:: prefix before any element from the standard library, you explicitly specify that you are referring to the elements in the std namespace.

Here's a simple example of how to use the standard library in C++:
```c++
#include <iostream>
#include <vector>
#include <algorithm>

std::vector<int> numbers = {5, 2, 9, 1, 7};

// Use standard library algorithm to sort the vector
std::sort(numbers.begin(), numbers.end());

// Use standard library to print the sorted vector
for (int num : numbers)
{
    std::cout << num << " ";
}
```

> ### This section was written in conjunction with Leo, Brave`s build-in AI.

The **C++ Standard Library**, also known as the STL (Standard Template Library), is **a comprehensive collection of reusable components and algorithms for performing common tasks in C++ programming**. It provides a wide range of features, including:

1. **Containers**: Such as vectors, lists, sets, maps, and arrays, for storing and manipulating data.
2. **Algorithms**: For performing operations on containers, such as sorting, searching, and manipulating elements.
3. **Iterators**: For traversing and accessing elements in containers.
4. **Streams**: For input/output operations, including file I/O, console I/O, and string manipulation.
5. **Strings**: For working with strings, including concatenation, searching, and manipulation.
6. **Numerics**: For mathematical operations, such as arithmetic, trigonometry, and statistical functions.
7. **Utility**: For miscellaneous tasks, such as memory management, exception handling, and type traits.

### Implementations

The C++ Standard Library has several implementations, including:

1. **GNU C++ Standard Library** (libstdc++): Part of the GNU Compiler Collection (GCC), licensed under the GPL v3 with the GCC Runtime Library Exception.
2. **LLVM C++ Standard Library** (libc++): Part of the LLVM Compiler Infrastructure, licensed under the Apache License 2.0 with LLVM Exceptions.
3. **Microsoft C++ Standard Library** (MSVC STL): Part of the Microsoft Visual C++ compiler, licensed under the Apache License 2.0 with LLVM Exceptions.
4. **NVIDIA C++ Standard Library** (libcudacxx): Part of the NVIDIA CUDA Toolkit, licensed under the Apache License 2.0 with LLVM Exceptions.
5. **HPX C++ Standard Library for Parallelism and Concurrency** (HPX): Developed by the STELLAR Group, licensed under the Boost Software License 1.0.
6. **Electronic Arts Standard Template Library** (EASTL): Developed by Electronic Arts, licensed under the BSD 3-Clause License.

### Key Features and Evolution

The C++ Standard Library has undergone significant changes and improvements over the years. Some notable features and milestones include:

1. **C++98**: Introduced the STL, with a focus on generic programming and containers.
2. **C++11**: Added features like move semantics, rvalue references, and lambda expressions.
3. **C++14**: Introduced generic lambdas, return type deduction, and variable templates.
4. **C++17**: Added features like structured bindings, if-constexpr, and fold expressions.
5. **C++20**: Introduced concepts, modules, and coroutines.
6. **C++23**: Improved interoperability with C, deprecated some legacy headers, and added named modules.

### Conclusion

The C++ Standard Library is a fundamental part of the C++ programming language, providing a wide range of reusable components and algorithms for common tasks. Its implementations vary, but the core features and evolution have been shaped by the C++ standards committee and the contributions of the developer community.

# Data types

| This section was written in conjunction with ChatGPT. |
| --- |

 
| Native Types |
| --- |
| - `bool` - Represents a logical value, either true or false |
| - `char` - Represents a single character in the ASCII5 character set | 
| - `int` - Represents a integer (whole number) |
| - `float` - Represents a floating-point number, which is a real number with a fractional component |
| - `double` - Represents a double-precision floating-point number, which has twice the precision of a float |


Table from Microsoft Learn about Data Type Ranges.

| Type Name | Bytes	| Other Names | Range of Values |
| ------- | ---- | --------- | ------------- |
| `int` | 4 | `signed` | -2,147,483,648 to 2,147,483,647 |
| `unsigned int` | 4 | `unsigned` | 0 to 4,294,967,295 |
| `__int8` | 1 |	`char`	| -128 to 127
| `unsigned __int8` |	1 |	`unsigned char` |	0 to 255
| `__int16` | 2 | `short`, `short int`, `signed short int` | 	-32,768 to 32,767
| `unsigned __int16` |	2 |	`unsigned short`, `unsigned short int` |	0 to 65,535
| `__int32` |	4 |	`signed`, `signed int`, `int`	| -2,147,483,648 to 2,147,483,647
| `unsigned __int32` |	4 |	`unsigned`, `unsigned int` |	0 to 4,294,967,295
| `__int64` |	8	| `long long`, `signed long long`	| -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
| `unsigned __int64` |	8 |	`unsigned long long`	| 0 to 18,446,744,073,709,551,615
| `bool` |	1 |	none	| `false` or `true`
| `char` |	1	| none	| -128 to 127 by default; 0 to 255 when compiled by using /J
| `signed char` |	1 | 	none |	-128 to 127
| `unsigned char` |	1 |	none |	0 to 255
| `short`	| 2 |	`short int`, `signed short int` |	-32,768 to 32,767
| `unsigned short` |	2 |	`unsigned short int` |	0 to 65,535
| `long` |	4 |	`long int`, `signed long int` |	-2,147,483,648 to 2,147,483,647
| `unsigned long` |	4 |	`unsigned long int` |	0 to 4,294,967,295
| `long long`	| 8	| none (but equivalent `to __int64`)	| -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
| `unsigned long long` |	8 |	none (but equivalent to `unsigned __int64`)	| 0 to 18,446,744,073,709,551,615
| `enum` |	varies |	none	
| `float` |	4	| none	3.4E +/- 38 (7 digits)
| `double` |	8	| none |	1.7E +/- 308 (15 digits)
| `long double` |	same as `double` |	none	| Same as `double`
| `wchar_t` |	2	| `__wchar_t` | 	0 to 65,535

If its name begins with two underscores `(__)`, a data type is non-standard.

> [!Warning]
> C++ lacks explicitness about data types size, leading to potential variation. For instance, the int type can manifest as either 16-bits or 32-bits, depending on the context.

Here is a summary of the explicit data sizes:

|   |
| --- |
| - char, signed char and unsigned char are at least 8 bits |
| - signed short, unsigned short, signed int and unsigned int are at least 16 bits |
| - signed long and unsigned long are at least 32 bits |
| - signed long long and unsigned long long are at least 64 bits | 


### Char
```c++
char myChar = 'a';
```
### Booleans
```c++
bool isDead = true;
```
### Integers
```c++
int health = 10;
```
### Modifiers

C++ allows the char, int, and double data types to have modifiers preceding them. A modifier is used to alter the meaning of the base type so that it more precisely fits the needs of various situations.
|   |
| --- |
| List of modifiers 
| - signed 
| - unsigned 
| - long 
| - short


The modifiers signed, unsigned, long and short can be applied to integer base types. In addition, signed and unsigned can be applied to char, and long can be applied to double.

The modifiers signed and unsigned can also be used as prefix to long or short modifiers.

For example:

unsigned long int // Same as unsigned 32-bit integer (unsigned int)

> [!Note]
> The default behavior for all integer types is signed.

Here is a list of modifiers for **integer** data type:

| Declare	| Size (bits)	| Size (bytes)	| Min Value	| Max Value |
| --- | --- | --- | --- | --- |
| `unsigned char`	| 8 |	1 | 	0	| 255 |
| `unsigned short int` | 	16 | 	2	| 0 |	65,535
| `unsigned int` |	32 |	4 |	0 |	4,294,967,295
| `unsigned long long`	64	8	0	18,446,744,073,709,551,615
signed char	8	1	-128	127
signed short int	16	2	-32,768	32,767
signed int	32	4	-2,147,483,648	2,147,483,647
signed long long	64	8	-9,223,372,036,854,775,808	9,223,372,036,854,775,807
Floating points (floats and doubles)
float speedInMetersPerSecond = 5.5f; // C++ always uses 'f' or 'F' literal for defining a float variable.
double speedInMetersPerSecond = 5.5; // C++ never uses a literal for defining a double variable.

| This section was written in conjunction with Leo, Brave`s build-in AI. |
| --- |

C++ provides several built-in data types, which are categorized into two main groups: **Fundamental Types** and **Derived Types**.

## Fundamental Types

1. **Integer Types**
    - `char`: 1 byte, signed or unsigned (e.g., char c;)
    - `short`: 2 bytes, signed or unsigned (e.g., short s;)
    - `int`: 4 bytes, signed or unsigned (e.g., int i;)
    - `long`: 4 bytes or 8 bytes, signed or unsigned (e.g., long l;)
    - `long long`: 8 bytes, signed or unsigned (e.g., long long ll;)
2. **Floating-Point Types**
    - `float`: 4 bytes (e.g., float f;)
double: 8 bytes (e.g., double d;)
long double: 12 bytes (e.g., long double ld;)
Character Types
wchar_t: wide character (e.g., wchar_t wc;)
Boolean Type
bool: true or false (e.g., bool b;)
Void Type
void: no value (e.g., void* ptr;)

## Derived Types

1. **Enumerated Types** (`enum`)
    - A user-defined type with a set of named values (e.g., enum Color { RED, GREEN, BLUE };)
2. **Arrays**
    - A collection of values of the same type (e.g., int arr[5];)
3. **Pointers**
    - A variable that holds the memory address of another variable (e.g., int* ptr;)
4. **Structures** (`struct`)
    - A composite data type that combines multiple variables (e.g., struct Person { int age; string name; };)
4. **Unions** (`union`)
    - A composite data type that allows storing different types of data in the same memory location (e.g., union Color { int red; float green; };)
5. **Class Types** (`class`)
    - A user-defined type that encapsulates data and functions (e.g., class Person { int age; string name; public: void print(); };)

## Macro Constants

C++ provides several macro constants that define the minimum and maximum values for integer types, such as:

- CHAR_MIN and CHAR_MAX for char
- SHRT_MIN and SHRT_MAX for short
- INT_MIN and INT_MAX for int
- LONG_MIN and LONG_MAX for long
- LLONG_MIN and LLONG_MAX for long long
- UINT_MAX for unsigned integer types

These constants are defined in the <climits> header and are useful for ensuring portability and consistency across different platforms.

Additional Resources

cppreference.com - a comprehensive online reference for C++ syntax and standard library functions.
C++ Standard - the official C++ standard document.
I hope this helps! Let me know if you have any further questions.

## Typedefs

| This section was written in conjunction with ChatGPT. |
| --- |

In C++, the typedef keyword is used to create an **alias** or **alternative name** for existing data types. It provides a way to define a new name that can be used as a shorthand for the original type, improving code readability and maintainability.

Here's an example:
```c++
typedef int myInt; // Declare our alias for custom type

myInt x = 5;  // Equivalent to: int x = 5;
```
> [!Warning]
> UHT6 doesn't support typedefs. Meaning, you can't expose to Blueprint.

| This section was written in conjunction with Leo, Brave`s build-in AI. |
| --- |

**In C++, the `typedef` keyword is used to give a new name to an existing data type**. It allows you to create an alias for a type, making your code more readable and maintainable. This can be particularly useful when working with complex or lengthy type names, such as template instantiations or nested structs.

Here are some key points about `typedef` in C++:

1. **Alias creation**: `typedef` creates an alias for an existing type, allowing you to use a shorter and more meaningful name.
2. **Type equivalence**: The new alias is equivalent to the original type; it doesn’t define a new type, but rather provides an alternative name.
3. **Scope**: The scope of a `typedef` declaration is limited to the current translation unit (i.e., the current source file).
4. **Usage**: You can use the new alias as a type specifier, just like the original type.

Examples:

- `typedef int NaturalNumber`; creates an alias `NaturalNumber` for the `int` type.
- `typedef struct { int x; int y; } Point`; creates a struct alias `Point` for a struct with two `int` members.

Best practices:

- Use `typedef` sparingly, as excessive use can lead to confusion and decreased readability.
- Choose meaningful and descriptive names for your aliases.
- Avoid using `typedef` for types that are already well-known or have a clear, concise name (e.g., `int`, `char`, `std::string`).

Common use cases for `typedef` include:

- Simplifying complex type names
- Creating domain-specific types (e.g., `NaturalNumber` for a type-safe integer)
- Improving code readability and maintainability

Overall, `typedef` is a useful tool in C++ for making your code more expressive and easier to understand.

## Members

| This section was written in conjunction with ChatGPT. |
| --- |

Members are variables or functions that are part of a class or object. They define the properties and behaviors of the class.

There are two main types of members: `variables` and `functions`.

### Variables
Members that store data. They can be of different types such as numbers, strings, booleans, or custom data types. Variables hold values that can be accessed and manipulated within the class or object.

### Assignments
There are abbreviations for frequently done kinds of assignments. Here are a few:

| Abbreviation | 	Meaning | 	Note |
| --- | --- | --- |
| `n += k` | 	`n = n + k` |	
| `n -= k` | 	`n = n - k` |	
| `++n` | 	`n = n + 1` | 	Where the value of expression ++n is the value of n after the assignment
| `n++` | 	`n = n + 1` | 	But the value of expression n++ is the value of n before the assignment
| `--n` |	`n = n - 1` | 	Where the value of expression --n is the value of n after the assignment
| `n--`	| n = n - 1 |	But the value of expression n-- is the value of n before the assignment

### Functions
Functions are blocks of code that perform a specific task or set of tasks. They encapsulate a series of instructions and can be called and executed from various parts of a program. Functions can accept input parameters (arguments) and can also return a value as a result.

Functions can be defined outside of classes as standalone functions or can be defined within classes as member functions. Standalone functions are typically used for common tasks that are not specific to any particular class or object.

Here's an example:
```c++
/// @brief Calculates the factorial of a number using recursion.
/// @param n Number of times.
/// @result A number.
int Factorial(int n)
{
    if (n == 0 || n == 1)
        return 1;
    else
        return n * Factorial(n - 1);
}
```

| This section was written in conjunction with Leo, Brave`s build-in AI. |
| --- |

In C++, a class or struct consists of its members, which are divided into two categories: **data members and member functions**.

### Data Members
- Also known as **attributes** or **data variables**
- Are variables declared within a class or struct
- Have access specifiers (public, private, protected) to control visibility and accessibility
- Can be initialized using constructors or assignment operators
- Can be accessed using dot notation (e.g., `obj.memberVariable`)

### Member Functions
- Also known as methods or member procedures
- Are functions declared within a class or struct
- Can be overloaded to accept different parameters
- Can be called using dot notation (e.g., obj.memberFunction())
- Can access and modify data members
- Can be declared as:
    - Non-static: tied to an instance of the class, using this pointer
    - Static: shared by all instances of the class, without this pointer

### Types of Member Functions

1. **Constructors**: special member functions called during object creation
2. **Destructors**: special member functions called during object destruction
3. **Assignment operators**: special member functions for assigning values to objects
4. **Overloaded operators**: member functions that implement custom operator behavior (e.g., `+`, `-`, `==`)
5. **Normal member functions**: general-purpose functions that perform specific tasks

### Accessing Members

- Data members can be accessed using dot notation (e.g., `obj.memberVariable`)
- Member functions can be called using dot notation (e.g., `obj.memberFunction()`)

### Best Practices

- Use access specifiers (public, private, protected) to control visibility and accessibility of members
- Use constructors and assignment operators to initialize and assign values to data members
- Use member functions to encapsulate logic and hide implementation details
- Avoid using public data members whenever possible, instead use public member functions to provide controlled access

By following these guidelines, you can effectively use members in C++ to create robust, maintainable, and efficient classes and structs.

## Classes

| This section was written in conjunction with ChatGPT. |
| --- |

Classes are the building blocks of object-oriented programming (OOP). They are a blueprint for creating objects, which are instances of a class. A class defines the structure and behavior of objects by specifying the members it contains.

A class can have variables (members) to store data and functions (methods) to perform actions. The variables defined within a class are often referred to as attributes, while the functions are referred to as methods.

Objects created from a class can access and modify the class's members. They provide a way to create multiple instances that share the same structure and behavior defined by the class. Objects can be thought of as individual entities that represent real-world objects or abstract concepts.

Classes allow for code reusability, encapsulation (hiding internal details), and the ability to model complex systems by organizing related data and behavior together.

Here's an example:
```c++
class Person
{
public:
    Person(std::string name, int age)
        : name(name)
        , age(age)
    { }

    void DisplayInfo()
    {
        // ...
    }

private:
    std::string name;
    int age;
};
```

### Structs

In C++, a struct is a user-defined data type that allows you to group multiple variables of different data types under a single name. It is similar to a class, but with some key differences.

### Usage of `struct` in C++

Structs are used to create lightweight data structures to hold related data elements.
They are commonly used to represent simple data objects or records that do not require complex behavior or methods.

**Difference between** `class` **and** `struct` In C++, the main difference between a `class` and a `struct` is the default access level. In a `class`, the default access level for its members is private, while in a `struct`, the default access level is public. This means that members of a `struct` are accessible outside the struct without the need for access specifiers.

For example:
```c++
struct Vector3
{
    float x;
    float y;
    float z;

    Vector3(float _x = 0.0f, float _y = 0.0f, float _z = 0.0f)
        : x(_x)
        , y(_y)
        , z(_z)
    { }
};

Vector3 v1(1.0f, 2.0f, 3.0f);
Vector3 v2(4.0f, 5.0f, 6.0f);

float dx = v1.x - v2.x;
float dy = v1.y - v2.y;
float dz = v1.z - v2.z;

float dist = std::sqrt(dx * dx + dy * dy + dz * dz);
```

**Historical difference with C language** In C, there was no concept of classes, and `struct` was the primary way to define user-defined data types. In C++, the `struct` keyword was retained to maintain compatibility with C, but it gained additional features and behavior, such as the ability to have member functions and access specifiers.

In modern C++, the distinction between `class` and `struct` has become more a matter of convention and coding style rather than a strict rule. Many developers prefer to use `struct` for simple data containers with public data members and class for more complex objects with private data members and member functions. However, you can use either class or struct based on your design preferences.

| This section was written in conjunction with Leo, Brave`s build-in AI. |
| --- |

In C++, a class is a user-defined data type that encapsulates data and functions that operate on that data. A class is a blueprint or a template that defines the properties and behavior of an object. Classes are the foundation of object-oriented programming (OOP) in C++.

### Key Components of a Class

1. **Data Members**: These are the variables declared within the class, which represent the state of an object.
2. **Member Functions**: These are the functions declared within the class, which define the behavior of an object.
3. **Access Specifiers**: These determine the accessibility of data members and member functions, such as public, private, or protected.
4. **Constructors**: These are special member functions that initialize objects when they are created.
5. **Destructor**: This is a special member function that is called when an object is destroyed.

### Declaring a Class

A class is declared using the `class` keyword followed by the class name. The class body is enclosed in curly braces `{ }`, where you define the data members and member functions.

Example:
```c++
class Person {
    public:
        int age;
        string name;

        void printInfo() {
            cout << "Name: " << name << ", Age: " << age << endl;
        }
};
```

### Creating Objects

To create an object from a class, you use the class name followed by the object name. You can create multiple objects from the same class.

Example:
```c++
Person person1;
Person person2;

person1.age = 25;
person1.name = "John";

person2.age = 30;
person2.name = "Jane";
```

### Accessing Members

You can access data members and member functions using the dot operator (.).

Example:
```c++
person1.printInfo(); // Output: Name: John, Age: 25
```

### Inheritance

C++ supports inheritance, which allows a class to inherit properties and behavior from another class.

Example:
```c++
class Employee : public Person {
    public:
        int employeeID;
        void printEmployeeInfo() {
            printInfo(); // Calls the printInfo() function from the Person class
            cout << "Employee ID: " << employeeID << endl;
        }
};
```

### Polymorphism

C++ supports polymorphism, which allows objects of different classes to be treated as objects of a common superclass.

Example:
```c++
class Shape {
    public:
        virtual void draw() = 0; // Pure virtual function
};

class Circle : public Shape {
    public:
        void draw() {
            cout << "Drawing a circle." << endl;
        }
};

class Rectangle : public Shape {
    public:
        void draw() {
            cout << "Drawing a rectangle." << endl;
        }
};

// Create objects and call the draw() function
Shape* shape1 = new Circle();
shape1->draw(); // Output: Drawing a circle.

Shape* shape2 = new Rectangle();
shape2->draw(); // Output: Drawing a rectangle.
```
These are the basic concepts and features of classes in C++. With classes, you can create reusable and modular code, encapsulate data and behavior, and implement complex systems and algorithms.


## Accessibility

| This section was written in conjunction with ChatGPT. |
| --- |

| Keyword |	Access ability |	Description |
| --- | --- | --- |
| `public` | All | Members and functions are accessible from anywhere, including outside the class. |
| `protected` | 	Subclasses | 	Members and functions are accessible from within the class and any subclasses, but not from outside. |
| `private` | 	Class | 	Members and functions are only accessible from within the class itself.
| `mutable` |	Class | Specifies that a member variable can be modified even if the owning object is const.
| `friend` | Class | Allows a non-member function or class to access the private and protected members of a class.

```c++
class MyClass
{
public:
    int publicVar; // Public member variable

    // Public member function
    void publicFunction()
    {
        // ...
    }

protected:
    int protectedVar; // Protected member variable

    // Protected member function
    void protectedFunction()
    {
        // ...
    }

private:
    int privateVar; // Private member variable

    // Private member function
    void privateFunction()
    {
        // ...
    }

    mutable int mutableVar; // Mutable member variable

    friend void friendFunction(MyClass& obj); // Friend function declaration
};

void friendFunction(MyClass& obj)
{
    obj.privateVar = 42; // Friend function can access private member variable
}

MyClass obj;

// Accessing public members
obj.publicVar = 10; // Compiled!
obj.publicFunction(); // Compiled!

// Accessing private members via friend function
friendFunction(obj); // Compiled!

// Accessing private members directly (only possible within the class)
obj.privateVar = 20; // Error!
obj.privateFunction(); // Error!
```

| This section was written in conjunction with Leo, Brave`s build-in AI. |
| --- |

In C++, access modifiers (also known as access specifiers) are used to control the accessibility of class members (data and functions) from outside the class. There are three main access modifiers:

1. **Public**: Members declared as public can be accessed from anywhere outside the class.
2. **Private**: Members declared as private can only be accessed within the class itself, and not from outside the class.
3. **Protected**: Members declared as protected can be accessed within the class itself and by derived classes (i.e., classes that inherit from the current class), but not from outside the class.

**Example**:

```c++
class MyClass {
public:
    int publicMember; // accessible from anywhere
    void publicFunction() {} // accessible from anywhere

private:
    int privateMember; // accessible only within MyClass
    void privateFunction() {} // accessible only within MyClass

protected:
    int protectedMember; // accessible within MyClass and derived classes
    void protectedFunction() {} // accessible within MyClass and derived classes
};
```

**Friendship**:

In C++, a class can grant access to its private and protected members to another class or function using the `friend` keyword. This allows the friend class or function to access the private and protected members of the original class.

**Example**:

```c++
class MyClass {
private:
    int privateMember;

friend class MyFriendClass; // grant access to MyFriendClass

};

class MyFriendClass {
public:
    void accessPrivateMember(MyClass& obj) {
        obj.privateMember = 10; // access private member of MyClass
    }
};
```

**Best Practices**:

1. Use public members sparingly, as they can be accessed from anywhere.
2. Use private members extensively, as they are only accessible within the class.
3. Use protected members to provide a controlled interface to derived classes.
4. Use friendship judiciously, as it can compromise the encapsulation of a class.

**Additional Resources**:

cppreference.com - Comprehensive documentation on access modifiers and friendship in C++.
GeeksforGeeks - Article on access modifiers in C++ with examples.
I hope this helps! Let me know if you have any further questions.

## If-statements

| This section was written in conjunction with ChatGPT. |
| --- |

If-statement is a fundamental control structure that allows you to conditionally execute a block of code based on a specified condition. It provides a way to control the flow of execution in your program.

```c++
if (condition)
{
    // Code to be executed if the condition is true
}
else if (secondCondition)
{
    // Code to be executed if the secondCondition is true, but condition was false
}
else
{
    // Code to be executed if the condition and secondCondition is both false
}
```

| This section was written in conjunction with Leo, Brave`s build-in AI. |
| --- |

## C++ If Statement Syntax
In C++, an if statement is a control flow statement that allows you to execute a block of code only if a specified condition is true. The syntax is as follows:
```C++
if (condition) {
    // code to execute if condition is true
}
```
 
Here, condition is a boolean expression that evaluates to either true or false. If the condition is true, the code inside the curly braces is executed. If the condition is false, the code is skipped.

### Example
```C++
int x = 5;
if (x > 10) {
    std::cout << "x is greater than 10";
}  // output: nothing, since x is not greater than 10
```
### If-Else Statements

An if-else statement allows you to execute different blocks of code depending on whether the condition is true or false. The syntax is as follows:
```C++
if (condition) {
    // code to execute if condition is true
} else {
    // code to execute if condition is false
}
```
### Example
```C++
int x = 5;
if (x > 10) {
    std::cout << "x is greater than 10";
} else {
    std::cout << "x is less than or equal to 10";
}  // output: "x is less than or equal to 10"
```
### Nested If Statements

You can nest if statements to create more complex conditional logic. The syntax is as follows:
```C++
if (outer_condition) {
    if (inner_condition) {
        // code to execute if both conditions are true
    } else {
        // code to execute if outer_condition is true but inner_condition is false
    }
} else {
    // code to execute if outer_condition is false
}
```
### Example
```C++
int x = 5;
int y = 3;
if (x > 10) {
    if (y > 2) {
        std::cout << "x is greater than 10 and y is greater than 2";
    } else {
        std::cout << "x is greater than 10 but y is less than or equal to 2";
    }
} else {
    std::cout << "x is less than or equal to 10";
}  // output: "x is less than or equal to 10"
```

### constexpr If Statements

In C++11 and later, you can use constexpr if statements to evaluate a constant expression at compile-time. The syntax is as follows:
```C++
constexpr bool is_even(int x) {
    return (x % 2) == 0;
}

int main() {
    if constexpr (is_even(4)) {
        std::cout << "4 is even";
    } else {
        std::cout << "4 is odd";
    }
}  // output: "4 is even"
```
Note that constexpr if statements can only be used with constant expressions, and the branch is eliminated at compile-time if the condition is known to be true or false.

### Best Practices

Use meaningful variable names and comments to make your code readable.
Avoid deep nesting of if statements; instead, use functions or separate logic blocks.
Use constexpr if statements when possible to optimize your code.
Test your code thoroughly to ensure correct behavior.
I hope this helps! Let me know if you have any further questions.

## 🔣 Comparisons and Boolean Operators
This section was NOT written in conjunction with ChatGPT.
Here are some operations for creating conditions:

== - Equality check
!= - Inequality check
> - Check for greater
< - Check for less
>= - Check for greater or equal
<= - Check for less or equal
&& - Expression A && B is evaluated by first evaluating A. A has value 0, then A && B also has value 0, and B is not evaluated. Otherwise, B is evaluated; if B has value 0, then A && B has the same value 0, and otherwise has value 1. Also called AND operator.
|| - Expression A || B is evaluated by first evaluating A. If A has a nonzero value, then A || B has value 1, and B is not evaluated. Otherwise, A || B has value 1 if B is nonzero and value 0 if B is zero. Also called OR operator.
! - Expression !A is 0 if A is nonzero, and is 1 if A is 0. Also called NOT operator.
