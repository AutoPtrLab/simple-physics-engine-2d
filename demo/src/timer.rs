//struct that defines the cooldown of something, thinking in imporving to host much more intervals in the same structure

//the time magnitude is ms
#[derive(Clone, Copy)]
pub struct Timer {
    count: f32,      // how many second have passed since the last target
    pub target: f32, // the end of the count
}

impl Timer {
    pub fn new(target: f32) -> Timer {
        Timer { count: 0.0, target }
    }

    //update the timer and check if its ready
    pub fn update_and_check(&mut self, dt: f32) -> bool {
        self.count += dt;
        if self.count >= self.target {
            self.count = 0.0; // we reset the count to zero
            return true;
        }
        false
    }
}
