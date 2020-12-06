use druid::Point;
use tactician_core::nalgebra::Vector2;


pub trait MathVec2ToUiVec2 {
    fn convert(&self, window_width: f64, window_height: f64) -> Point;
}

impl MathVec2ToUiVec2 for Vector2<f64> {
    fn convert(&self, window_width: f64, window_height: f64) -> Point {
        let window_x = self.x + (window_width/2.0);
        let window_y = (window_height / 2.0) - self.y;
        return Point::from((window_x, window_y))
    }
}
