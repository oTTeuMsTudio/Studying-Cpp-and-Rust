# Install UE 5.4.4 on Ubuntu 24.04

Here’s a step-by-step guide to install Unreal Engine 5.4.4 on Ubuntu 24.04:

## 1. Download Unreal Engine 5.4.4:
- Visit the official Unreal Engine website (www.unrealengine.com) and click on the “Download Unreal Engine” button.
- Select the “Linux” option and choose the “Ubuntu 22.04” package (as Unreal Engine 5.4.4 is compatible with Ubuntu 22.04 and later).
- Download the .zip file containing the Unreal Engine 5.4.4 installation package.

## 2. Extract the installation package:
- Extract the downloaded .zip file to a directory of your choice (e.g., ~/UnrealEngine).

## 3. Run the installer:
- Navigate to the extracted directory and run the SetupUE.sh script (or SetupUE.bat on Windows) as a terminal command.
- Follow the installer prompts to select the installation components and directory.

## 4. Configure the installation:
- Set UE_USE_SYSTEM_DOTNET=1 as an environment variable to use the system-installed .NET 6 instead of the bundled .NET 3.
- Edit the SetupDotnet.sh script to read the host .NET version correctly (as described in one of the search results).

## 5. Launch Unreal Engine:
- Run the UnrealEditor executable from the Engine/Binaries/Linux directory.
- You can also use the Epic Asset Manager to download and manage your projects and assets.

### Additional Notes:

Make sure your system meets the minimum and recommended hardware specifications for Unreal Engine 5.4.4, as listed on the official Unreal Engine website.
If you encounter any issues during installation, refer to the Unreal Engine documentation or seek help from the Epic Games community forums.
By following these steps, you should be able to successfully install Unreal Engine 5.4.4 on your Ubuntu 24.04 system.



========================================================================================

# C++ Boosts Unreal Engine
Unlocking Unreal Engine’s Potential: The Synergy of C++ and Unreal Engine By combining the robustness of C++ with the versatility of Unreal Engine, developers can create rich, immersive experiences that showcase the engine’s capabilities. C++ provides a powerful, performance-oriented language for building game logic, while Unreal Engine offers a comprehensive toolset for creating 3D graphics, physics, and audio.

## Key Benefits

### Performance Optimization: 
C++'s low-level memory management and compilation allow for fine-grained control over performance-critical components, enabling developers to optimize game logic and reduce latency.

### Customizability: 
C++'s flexibility enables developers to create custom plugins, tools, and features that integrate seamlessly with Unreal Engine, tailoring the engine to their specific needs.

### Native Integration: 
C++ code can be directly integrated with Unreal Engine’s native components, such as physics, graphics, and audio, allowing for seamless communication and data exchange.

### Advanced Scripting: 
C++'s capabilities enable developers to create complex, data-driven scripts that drive game behavior, interact with Unreal Engine’s systems, and leverage the engine’s built-in features.

## Real-World Applications

### 1. AAA Games: 
C++ and Unreal Engine have been used to create numerous AAA titles, such as Fortnite, Gears of War, and Mass Effect, demonstrating the power of this combination in high-performance game development.

### 2. Simulation and Visualization: 
C++'s performance and Unreal Engine’s graphics capabilities make it an ideal combination for simulation and visualization applications, such as surgical training simulations and architectural visualizations.

### 3. Indie Games: 
C++ and Unreal Engine provide a versatile platform for indie developers to create innovative, visually stunning games with complex mechanics and physics.

## Conclusion

By harnessing the strengths of C++ and Unreal Engine, developers can create immersive experiences that showcase the engine’s capabilities and push the boundaries of game development. Whether building AAA titles, simulation applications, or indie games, this powerful combination enables developers to unlock the full potential of Unreal Engine and create engaging, high-performance experiences.


=====================================================



# Unreal Classes are 'Managed'
Unreal object instances are automatically cleanded up when you no longer hold onto any references to them. (A pointer)
Marking your pointer with UPROPERTY() is what tells unreal to leave it alone. 

# Unreal Object Auto-Cleanup Mechanism
In Unreal Engine, classes are considered “managed” when they inherit from UObject. Managed objects are automatically cleaned up by the garbage collector when they are no longer referenced by any pointers. This means that as long as there are no active references to an object, Unreal Engine will eventually destroy it and reclaim its memory.

However, there is an exception to this rule. If you mark a pointer to a managed object with UPROPERTY(), Unreal Engine will treat that pointer as a special case and not destroy the object, even if there are no other active references to it. This is because UPROPERTY() indicates that the object is being used as a property or member variable, and Unreal Engine assumes that it needs to preserve the object’s existence for serialization or other purposes.

In other words, UPROPERTY() tells Unreal Engine to “leave the object alone” and not garbage-collect it, even if it’s no longer referenced by any other code. This can be useful in certain situations, such as when you need to preserve an object’s state across levels or saves, or when you’re working with assets that need to be loaded and unloaded dynamically.

It’s worth noting that this behavior only applies to managed objects (i.e., objects that inherit from UObject). If you’re working with raw C++ objects or other types of data structures, you’ll need to manage their memory manually using standard C++ techniques, such as smart pointers or manual deletion.

=========================================================


# Unreal Class Default Object
In Unreal Engine, a Class Default Object (CDO) is a template object that represents the default state of a UClass. It is generated by the class constructor during engine initialization and remains unmodified thereafter.

The CDO serves as a prototype for all subsequent instances of the class, providing the default values and behavior for the class. When a new instance of the class is created, it is initialized by copying the CDO’s properties and state.

Here are key characteristics of a CDO:

### 1. Generated by the class constructor: 
The CDO is created during engine initialization, and its construction is handled by the class constructor.

### 2. Unmodified thereafter: 
The CDO remains unchanged throughout the engine’s lifetime, serving as a stable reference point for default values and behavior.

### 3. Prototype for subsequent instances: 
When a new instance of the class is created, it is initialized by copying the CDO’s properties and state.

### 4. Default values and behavior: 
The CDO defines the default values for properties and the default behavior of the class.

In terms of code, the CDO is typically created using the UCLASS macro, which specifies the class’s Unreal Engine metadata and generates the CDO. The GENERATED_BODY macro is also used to set up the class’s infrastructure, ensuring that the CDO is properly initialized and maintained.

In summary, the Class Default Object (CDO) is a crucial component of Unreal Engine’s object model, providing a template for default values and behavior that is used to initialize subsequent instances of a UClass.
