// This is not a real rust file, but a schema definition file that defines the
// structure of a scripted component.
type Component = Position;

struct Position {
    x: i32,
    y: i8,
    number: f32,
    boolean: bool,
    custom: CustomType,
}
