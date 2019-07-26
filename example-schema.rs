// This is not a real rust file, but a schema definition file that defines the
// structure of a scripted component.

// This states which type contains the root component data. It *must* be defined
// as `type Component = TypeName`. It is the only `type` statement allowed.
type Component = Position;

// Define component data struct.
struct Position {
    x: i32,
    /* Inline and
     * multiline comment
     * */ y: i8,
    number: f32,
    boolean: bool,
    custom: Awesomeness, // Trailing commas
}

// Another type used by the component data.
struct Awesomeness<T, U> {
    test: AnotherType,
    value: u32 // No trailing comma
}

struct AnotherType {
    nothing_special: char
}
