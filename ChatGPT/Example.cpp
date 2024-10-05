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
