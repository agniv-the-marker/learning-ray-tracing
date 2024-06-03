use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut dyn std::io::Write, pixel_color: &Color) -> std::io::Result<()> {
    // https://doc.rust-lang.org/std/keyword.dyn.html
    // 'dynamically dispatched'
    // 'dyn' keyword is used to indicate that the trait is object safe 
    // we don't actually know the concrete type of the object at compile time
    // we only know that it implements the trait of Write
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let color = Color::new(0.5, 0.3, 0.8);
        assert_eq!(color.x(), 0.5);
        assert_eq!(color.y(), 0.3);
        assert_eq!(color.z(), 0.8);
    }
}