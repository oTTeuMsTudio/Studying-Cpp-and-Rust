# Object-Oriented Programming

> ### This section was written in conjunction with ChatGPT.

Object-Oriented Programming (OOP) is a programming paradigm that organizes code around objects, which are instances of classes. It focuses on the concept of objects, their properties (attributes), and behaviors (methods), allowing for modular, reusable, and structured code design.

Encapsulation
Encapsulation is the practice of bundling data and the methods that operate on that data within a single unit, which is typically a class. It promotes data hiding and information hiding, ensuring that the internal state and implementation details of an object are not directly accessible from outside the object. Encapsulation helps achieve data integrity, security, and abstraction by controlling access through well-defined interfaces.

Data Hiding
Data hiding is a principle closely related to encapsulation. It involves concealing the internal implementation details of an object and exposing only the necessary information through well-defined interfaces. By hiding implementation details, the object's data is protected and can only be accessed or modified through controlled methods. This enhances data security, code maintainability, and reduces the risk of unintended modifications or access to critical data.

Inheritance

Inheritance is a mechanism in OOP that allows new classes (derived classes or subclasses) to inherit properties and behaviors from existing classes (base classes or superclasses). Inheritance promotes code reuse, as derived classes can inherit and extend the functionality of their base classes. The derived classes can add new attributes and behaviors or override existing ones to customize the behavior of inherited members. Inheritance facilitates code organization, modularity, and the creation of hierarchical relationships between classes.

Polymorphism

Polymorphism is a fundamental concept in object-oriented programming (OOP) that allows objects of different classes to be treated as objects of a common base class. It enables you to write code that can work with objects of multiple types in a uniform manner.

Polymorphism is often illustrated through inheritance, where you have a base class and multiple derived classes that inherit from it. The derived classes can override or extend the behavior of the base class's methods, while still adhering to the common interface.

> ### This section was written in conjunction with Leo, Brave`s build-in AI.

# Object-Oriented Programming
Object-Oriented Programming (OOP) is a programming paradigm that revolves around the concept of “objects” and their interactions. In C++, OOP is achieved through the use of classes, objects, inheritance, polymorphism, and encapsulation.

Key Concepts:

Classes: A class is a blueprint or a template that defines the properties and behavior of an object. It’s a user-defined data type that encapsulates data and functions that operate on that data.
Objects: An object is an instance of a class, with its own set of attributes (data) and methods (functions). Objects have their own memory space and can be manipulated independently.
Inheritance: Inheritance allows a derived class to inherit the properties and behavior of a base class. This enables code reuse and facilitates a hierarchical organization of classes.
Polymorphism: Polymorphism is the ability of an object to take on multiple forms. In C++, this is achieved through function overloading (multiple functions with the same name but different parameters) and function overriding (a derived class provides a different implementation of a function already defined in its base class).
Encapsulation: Encapsulation is the concept of hiding an object’s internal state and behavior from the outside world, while exposing only a controlled interface through which other objects can interact with it.
C++ OOP Examples:

Simple Class: Define a Car class with attributes brand, model, and mileage, and methods startEngine() and accelerate().
class Car {
public:
    string brand;
    string model;
    int mileage;

    void startEngine() { cout << "Vroom!" << endl; }
    void accelerate() { mileage++; }
};

Inheritance: Create a ElectricCar class that inherits from Car and adds a batteryLevel attribute and a charge() method.
class ElectricCar : public Car {
public:
    int batteryLevel;

    void charge() { batteryLevel += 10; }
};

Polymorphism: Define a Vehicle class with a virtual drive() method, and have Car and ElectricCar classes override this method.
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

Best Practices:

Follow the Single Responsibility Principle (SRP): Each class should have a single, well-defined responsibility.
Use encapsulation to hide internal implementation details.
Favor composition over inheritance: Instead of inheriting from a base class, consider composing objects from smaller, independent components.
Use polymorphism to write flexible and reusable code.
Resources:

“Object-Oriented Programming in C++” by Robert Lafore (book)
GeeksforGeeks - Object-Oriented Programming in C++
C++ documentation - Classes and Objects
Remember to practice and experiment with different OOP concepts in C++ to solidify your understanding.

