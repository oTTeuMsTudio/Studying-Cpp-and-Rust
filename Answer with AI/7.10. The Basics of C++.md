# Lets talk about the Basics of C++ 

## We will cover:

### 1. Variables and constants
### 2. Naming conventions
### 3. Mathematical expressions
### 4. Writing to and reading from the console
### 5. Working with the standard library
### 6. Comments

# Variables - Small Exercise
Write code to swap the value of two variables (this is a common interview question).

Declare 2 variables a and b.

```c++
#include <iostream>
using namespace std;

int main(){
  int a = 1;
  int b = 2;
  
  cout << a;

  return 0;
}
```
## Solution
We declare a third variable and call it temp, then on the end the b is 1.

```c++
#include <iostream>
using namespace std;

int main(){
  int a = 1;
  int b = 2;
  int temp = a;
  a = b;
  b = temp;
  std::cout << a;

  return 0;
}
```
# Constants - 
Where we don`t want the value of a variable to change we use constants.

```c++
#include <iostream>

int main(){
  const double pi = 3.14;
  pi = 0;

  return 0:
}
```

# Naming Conventions

Different teams prefer different conventions.

```c++
#include <iostream>

int main(){
  int file_size; // Snake Case
  int FileSize; // Pascal Case
  int fileSize; // Camel Case
  int iFileSize; // Hungarian Notation


  return 0:
}
```

# Mathematical Expressions
Now that we learned, how to declare variables and constants, let`s write expressions for performing calculations. This is where the fun begins.

## Floating point number
To get a floating point number, we have to convert one of these numbers to a double.
```c++
#include <iostream>

int main(){
  int x = 10;
  int y = 3;
  double z = x / y;
  std::cout << z;

  return 0;
}
```
## Modulus
Returns the remainder of a division.

```c++
#include <iostream>

int main(){
  int x = 10;
  int y = 3;
  int z = x % y;
  std::cout << z;

  return 0;
}
```
# Increment, Decrement
```c++
#include <iostream>

int main(){
  int x = 10;
  int y = x++; // x = 11, y = 10
  int z = ++x; // x = 11, z = 11
  x++
  ++x
  x--
  --x
  std::cout << x;
  return 0;
}
```
# 

















