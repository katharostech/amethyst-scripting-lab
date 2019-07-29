// This is not a real rust file, but an example schema definition file that
// defines the structure of a scripted component.

// Line comments are supported

// This is the player object. Player is defined as a component which is the same
// as a struct except it creates a scripted component.
#[Component]
struct Player {
    position: Position,
    state: PlayerState,
    // You can have type parameters:
    life: Option<u8>, // Struct fields can have trailing commas
    //          ^^^^
}

struct Position {
    x: /* You can stick inline,
        * multiline comments anywhere */ f32,
    y: f32 // Trailing commas aren't required
}

// Structs can have type parameters, too.
struct HashMap<T, U> {
    keys: Vec<T>,
    values: Vec<U>,
}

enum PlayerState {
    Walking,
    Running,
    Standing,
    Falling,
    Jumping,
}
