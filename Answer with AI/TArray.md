# TArray in Unreal Engine 5

In UE5, `TArray` is a dynamically sized array of typed elements. It’s a container class that stores one or more elements of the same type. `TArray` is a value type, meaning it should be treated similarly to built-in types like `int32` or `float`. It’s not designed to be extended, and creating or destroying `TArray` instances with new and `delete` is not recommended.

## Key Features

- Dynamically resizable: elements can be added or removed at runtime.
- Typed: stores elements of a specific type, such as `int32`, `FString`, or `TSharedPtr`.
- Default allocator: uses a heap-based allocator, which provides memory as needed when new elements are added.
- `Add` and `Emplace` functions: create new elements at the end of the array. Add copies or moves an instance of the element type, while `Emplace` constructs a new instance using the provided arguments.

## Example Usage

```c++
// Create an empty TArray designed to hold a sequence of integers.
TArray<int32> IntArray;

// Populate the array using Add and Emplace functions.
IntArray.Add(1);
IntArray.Emplace(2);
IntArray.Emplace(3); // IntArray == [1, 2, 3]
```

## Important Considerations

- `TArray` elements are value types and are owned by the array. This means that when an element is added or removed, the array takes care of memory management.
- `TArray` is not a pointer to a dynamically allocated array. It’s a standalone container that manages its own memory.

By using `TArray` in UE5, you can efficiently and safely store and manipulate collections of elements, taking advantage of the engine’s built-in memory management and type safety features.

