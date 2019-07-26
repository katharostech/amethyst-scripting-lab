// This is not a real rust file, but an example schema definition file that
// defines the structure of a scripted component.

// Line comments are supported

// This defines the type that contains the root component data. It *must* be
// defined as `type Component = TypeName`. It is the only `type` statement
// allowed. The purpose is to distinguish which struct out of the ones in the
// schema is to be used as the component struct.
type Component = Player;

// This is the player object
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

// TODO:
// - References?
