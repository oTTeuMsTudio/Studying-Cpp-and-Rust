# UE5 Class Constructor Usage

In Unreal Engine 5 (UE5), you can use class constructors to initialize objects and set their properties. When creating a class that inherits from UObject (such as AActor), you must implement the default constructor that takes an FObjectInitializer parameter.

## Regular Constructors

For simple classes without inheritance from UObject, you can define a regular constructor with or without parameters. This is useful when you need to initialize objects with custom values.

## Constructors with Parameters

When inheriting from UObject, you may need to define constructors with parameters to initialize specific properties. For example, in the HSpatialHash class, a constructor with a const float PCellSize parameter is used to initialize the CellSize property.

## Example

Here’s an example of a UE5 class constructor:
```C++
UCLASS()
class xxx_API UHSpatialHash final : public UObject
{
    GENERATED_BODY()

public:
    explicit UHSpatialHash(const float PCellSize) { CellSize = PCellSize; }
};
```

In this example, the UHSpatialHash class has a constructor that takes a const float parameter PCellSize and initializes the CellSize property.

Best Practices

When inheriting from UObject, implement the default constructor with an FObjectInitializer parameter.
For simple classes without inheritance, define regular constructors with or without parameters.
Use constructors with parameters to initialize specific properties when inheriting from UObject.
