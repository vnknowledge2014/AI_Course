// a) struct có 2 field bool
struct Point { x: bool, y: bool }

// b) enum 3 variants
enum TrafficLight { Red, Yellow, Green }

// c) struct chứa 2 enum
struct Intersection { north: TrafficLight, east: TrafficLight }

// d) enum với data
enum Shape {
    Circle(bool),           // có tô màu?
    Rectangle(bool, bool),  // tô màu?, có viền?
}

fn main() {}
