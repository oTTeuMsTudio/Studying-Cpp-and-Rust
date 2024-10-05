# Unreal Engine C++ Class Hierarchy

In Unreal Engine, C++ classes are organized into a hierarchical structure using inheritance. This allows for code reuse, modularity, and easier maintenance. Here’s an overview of the class hierarchy and inheritance concepts:

## Class Hierarchy

### 1. Parent Classes (Base Classes): These classes define the common attributes and behaviors that will be inherited by their child classes.
### 2. Child Classes (Derived Classes): These classes inherit the properties and methods from their parent classes and can also add new ones or override existing ones.

## Inheritance

Unreal Engine supports single inheritance, where a child class inherits from a single parent class. Multiple inheritance is not directly supported, but it can be achieved using interfaces or composition (more on this later).

## Key Concepts

### 1. UCLASS: A macro used to declare a class that will be registered with the Unreal Engine’s class system.
### 2. UINTERFACE: A macro used to declare an interface, which is a contract that a class must implement.
### 3. ** GENERATED_BODY()**: A macro that generates the necessary boilerplate code for a class.
### 4. Super: A reference to the parent class, used to access its members and override its behavior.

## Example

Suppose we have a base class AMyShapeActor that defines common attributes and behaviors for shape actors. We can create a child class ACubeActor that inherits from AMyShapeActor and adds its own specific attributes and behaviors.

// AMyShapeActor.h
UCLASS()
class MYPROJECT_API AMyShapeActor : public AActor
{
    GENERATED_BODY()

    // Common attributes and behaviors
    float length;
    float width;
    float height;

    // ...
};

// ACubeActor.h
UCLASS()
class MYPROJECT_API ACubeActor : public AMyShapeActor
{
    GENERATED_BODY()

    // Specific attributes and behaviors for cube actors
    int numFaces;

    // Override parent class methods
    void Tick(float DeltaTime) override;
};

## Interfaces and Composition

While Unreal Engine does not support multiple inheritance, you can achieve similar behavior using interfaces or composition.

### 1. Interfaces: Define an interface using UINTERFACE, which specifies a contract that a class must implement. Child classes can then implement this interface, effectively achieving multiple inheritance.
### 2. Composition: Instead of inheriting from multiple classes, create a child class that contains instances of other classes as members. This approach allows for loose coupling and flexibility.

## Best Practices

### 1. Use inheritance judiciously, as it can lead to tight coupling and fragility.
### 2. Favor composition over inheritance when possible.
### 3. Use interfaces to define contracts and decouple classes.
### 4. Keep class hierarchies shallow and focused on a specific domain.

By understanding and applying these concepts, you can effectively design and implement a robust and maintainable C++ class hierarchy in Unreal Engine.
