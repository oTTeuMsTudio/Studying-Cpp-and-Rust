# Unreal's C++ Standard Library Use

Unreal Engine generally avoids using the C++ Standard Library (STL) containers and classes. This is due to several reasons:

### - Engine portability: 
Unreal Engine aims to be highly portable across various platforms, including Windows, Mac, and Linux. The STL, being a compiler-dependent implementation, can lead to compatibility issues and potential bugs when used across different platforms.

### - Efficient and optimized containers: 
Unreal Engine has invested significant effort in developing its own container classes, such as TArrays and TMaps, which are optimized for performance and tailored to the engine’s specific needs. These containers provide better memory management, caching, and other features that are crucial for the engine’s functionality.

### - Consistency and predictability: 
By avoiding the STL, Unreal Engine ensures consistency and predictability in its behavior across different platforms and compilers. This helps developers write more reliable and maintainable code.

While it is technically possible to use external libraries that rely on the STL, Unreal Engine discourages this approach. Instead, developers are encouraged to:

- Use Unreal’s native container classes and types
- Create wrapper classes to convert external library types to Unreal Engine types
- Implement custom solutions that adhere to Unreal Engine’s guidelines and conventions
By avoiding the C++ Standard Library, Unreal Engine maintains its focus on performance, portability, and consistency, ultimately benefiting
