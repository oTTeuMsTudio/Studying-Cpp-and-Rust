// Parent class: AShapeActor
UCLASS()
class AShapeActor : public AActor
{
    GENERATED_BODY()
    // ...
};

// Child class: ACubeActor
class ACubeActor : public AShapeActor
{
    GENERATED_BODY()
    // ...
    void SetLength(int new_length) { ... }
    void SetWidth(int new_width) { ... }
    void SetHeight(int new_height) { ... }
};
