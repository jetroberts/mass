
struct Weight {
    name: String,
    mass: f32,
    reps: u16,
}

impl Weight {
    fn new(name: String, mass: f32, reps: u16) -> Weight {
        return Weight {
            name,
            mass,
            reps,
        };
    }
}
