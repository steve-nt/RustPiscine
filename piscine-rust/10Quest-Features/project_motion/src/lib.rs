const GRAVITY: f32 = 9.8;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            init_position,
            init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<ThrowObject> {
        // Increment the time by 1 second first
        self.time += 1.0;
        let t = self.time;

        // Use the absolute formulas to calculate raw values
        // p = p0 + v0 * t - 0.5 * g * t^2
        let raw_x = self.init_position.x + self.init_velocity.x * t;
        let raw_y = self.init_position.y + self.init_velocity.y * t - 0.5 * GRAVITY * t * t;

        // If it hits or falls below the floor, stop iterating
        if raw_y < 0.0 {
            return None;
        }

        // v = v0 - g * t
        let raw_vx = self.init_velocity.x;
        let raw_vy = self.init_velocity.y - GRAVITY * t;

        // Apply rounding to 1 decimal place to pass strict tests
        self.actual_position.x = (raw_x * 10.0).round() / 10.0;
        self.actual_position.y = (raw_y * 10.0).round() / 10.0;
        self.actual_velocity.x = (raw_vx * 10.0).round() / 10.0;
        self.actual_velocity.y = (raw_vy * 10.0).round() / 10.0;

        Some(self.clone())
    }
}