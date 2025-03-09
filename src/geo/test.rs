#[cfg(test)]
pub mod test {
    use crate::canvas::Canvas;
    use crate::color::Color;
    use crate::geo::matrix::{Matrix, Rotation};
    use crate::geo::point::Point;

    use super::*;

    #[test]
    fn draw_circle() {
        let mut canvas = Canvas::new(100, 100);
        let radius = 40.0;
        let point = Point::new(radius, 0., 0.); // point radius units from origin

        for i in 0..16 {
            let angle = i as f64 * std::f64::consts::PI / 8.;
            let rot = Matrix::rotation(angle, Rotation::Z);
            let point = rot * point;
            // Translate after rotation
            let point = point + Point::new(50., 50., 0.);
            let color = Color::new(255., 0., 0.);
            canvas.draw_point(point, color);
        }

        canvas.save_ppm("circle.ppm");
    }

    #[test]
    fn test_matrix_3x3() {
        let mut matrix = Matrix::new(3, 3);
        matrix[0][0] = 1.;
        matrix[0][1] = 2.;
        matrix[0][2] = 3.;
        matrix[1][0] = 4.;
        matrix[1][1] = 5.;
        matrix[1][2] = 6.;
        matrix[2][0] = 7.;
        matrix[2][1] = 8.;
        matrix[2][2] = 9.;
        assert_eq!(matrix[0][0], 1.);
    }
}
