pub trait Lerp<T> {
    fn lerp(&mut self, end: T, alpha: T);
}

impl Lerp<f32> for f32 {
    fn lerp(&mut self, end: f32, alpha: f32) {
        *self = *self + (end - *self) * alpha
    }
}
