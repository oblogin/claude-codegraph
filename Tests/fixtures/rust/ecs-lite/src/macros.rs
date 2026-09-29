/// Builds a 2D vector from any numeric expressions.
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        $crate::math::Vec2::new($x as f32, $y as f32)
    };
}
