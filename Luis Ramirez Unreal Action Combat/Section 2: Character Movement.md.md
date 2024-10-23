Section 2: Character Movement
In this section, we took the time to set up the character's models animation with movement.

Section Intro - Character Movement
This lecture does not contain notes.

Importing the Player and Enemy Models
In this lecture, we imported assets from the Epic Games marketplace to our project. We added two assets called Paragon: Greystone and Paragon: Grux. 

To apply these models to our blueprints, we updated the Mesh component's skeletal mesh setting. 


Skeletal Mesh Asset Setting
The assets we downloaded have various skins of the same model. So, you're more than welcome to use whatever skin you like. 

For the boss, we also had to adjust the position and rotation of the mesh. The mesh should  be inside the collision capsule and pointing in the same direction as the blue arrow, which is the facing direction of the blueprint. Other than that, you should be good to go with using the blueprints.

Adding Animation Blueprints
In this lecture, we learned how to create an animation blueprint. Animation blueprints allow us to tell Unreal what animations are available on a character as well as control which animation to be playing. To create an animation blueprint, you must select a skeleton. It's essential to select the correct skeleton, otherwise the animations may not translate well on the actual blueprint responsible for rendering the character.

Once you create the animation blueprint, you must configure the blueprint with your character to use it. You can do so by setting the following settings:

Animation Mode - Set this to Use Animation Blueprint

Animation Class - The animation blueprint file.

Adding Animation Blendspaces
In this lecture, we added animation blendspaces for the idle and jogging animations for both characters in our game. Animation blendspaces are a feature for smoothly transitioning between animations. If you want to learn how to create an animation blendspace, please watch the video. 

Playing Animation Blendspaces
In this lecture, we learned how to add our animation blendspace to the animation blueprint, which is by simply dragging and dropping it. Next, we updated the labels of each axes and then hooked up a vertical axis.


We're using a variable so that we can update it later. We did so from the event graph.


We're using a few nodes here:

Event Blueprint Update Animation: An event node that runs whenever the animation is updated. We're using it to update the variable frequently.

Try Get Pawn Owner: Gets the current owner of this animation blueprint, which would be the player, so that we can grab their velocity.

Get Velocity: Grabs the current velocity of the actor.

Vector Length: Get's the vector's length as a float so that we can use it to update our variable.

Set: Used to update the CurrentVelocity variable with the Vector Length node's value.

Creating a C++ Anim Instance Class
In this lecture, we learned how to create a C++ class. Whenever you do so, one of the first things you should do is make sure it inherits from the proper class. Typically, the parent class can be found under the Class Settings section of a blueprint under an option called Parent Class. The full proper class name can be found by viewing the actual class itself.


Finding the current class of a blueprint.
Once you find the proper class name, you can search for the class during creation. For this lecture, we created classes from two classes called UAnimInstance and ACharacter.

Unreal Properties
In this lecture, we defined a variable for storing the current velocity inside the UPlayerAnimInstance class.

Copy
UPROPERTY(BlueprintReadWrite, EditAnywhere)
float CurrentVelocity{ 0.0f };
A few things worth noting:

The variable is initialized with direct initialization over copy initialization to reduce the likelihood of unintended type conversion.

We're adding the UPROPERTY macro with the BlueprintReadWrite specifier to expose the variable through the blueprint editor.

The EditAnywhere specifier allows you to modify the variable through the details window. 

Resources
Properties - https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-engine-uproperties

Converting Nodes Into Functions
In this lecture, we converted the nodes that we used for updating the velocity into its C++ equivalent. Here's the final result:

Copy
void UPlayerAnimInstance::UpdateVelocity()
{
	APawn* PawnRef{ TryGetPawnOwner() };

	if (!IsValid(PawnRef)) { return; }

	FVector Velocity{ PawnRef->GetVelocity() };
	CurrentVelocity = static_cast<float>(Velocity.Length());
}
Here's what we did:

TryGetPawnOwner() - Retrieves the actor that is using this animation blueprint.

IsValid() - Checks if a variable holds a reference to an actor. Since the TryGetPawnOwner() function may not be able to find an actor, you should check the reference before doing anything else.

GetVelocity() - Returns the actor's velocity.

Length() - Converts the Vector's length into a float value.

It's important to note that the FVector type is an alias for the TVector type. Check out the resource section for a link on more info.

Lastly, we typecasted the return value of the Length() function to a float since it returns a double by using the static_cast function.

Resources
UAnimInstance Class - https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Engine/Animation/UAnimInstance 

TVector - https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Core/Math/TVector

Function Specifiers
In this lecture, we used function specifiers to expose our function to our blueprint. The UFUNCTION() macro can be used to do that. We must add the BlueprintCallable specifier like so:

Copy
UFUNCTION(BlueprintCallable)
void UpdateVelocity();

When using this node, Unreal adds the target pin so that we can configure what function this object gets called on. By default, it'll call it on whatever class is using it. 

Resources
UFUNCTION Macro - https://dev.epicgames.com/documentation/en-us/unreal-engine/ufunctions-in-unreal-engine
