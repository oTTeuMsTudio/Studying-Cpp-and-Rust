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

























