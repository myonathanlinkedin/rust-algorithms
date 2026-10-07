mod core;
use core::{Canvas, Color, Image, Layer};

fn main() {
    // Basic sanity checks using asserts.
    // 1. Image fill.
    let mut img = Image::new(4, 4);
    img.fill(Color::OPAQUE_WHITE);
    for y in 0..4 {
        for x in 0..4 {
            assert_eq!(img.get_pixel(x, y), Color::OPAQUE_WHITE);
        }
    }

    // 2. Draw rectangle.
    img.fill(Color::TRANSPARENT);
    img.draw_rect(1, 1, 2, 2, Color { r: 10, g: 20, b: 30, a: 255 });
    assert_eq!(img.get_pixel(0, 0), Color::TRANSPARENT);
    assert_eq!(img.get_pixel(1, 1), Color { r: 10, g: 20, b: 30, a: 255 });
    assert_eq!(img.get_pixel(2, 2), Color { r: 10, g: 20, b: 30, a: 255 });
    assert_eq!(img.get_pixel(3, 3), Color::TRANSPARENT);

    // 3. Layer blending.
    let mut canvas = Canvas::new(2, 2);
    let bg_idx = canvas.add_layer(255);
    let fg_idx = canvas.add_layer(128);
    {
        let bg = canvas.layer_mut(bg_idx).unwrap();
        bg.image.fill(Color { r: 200, g: 0, b: 0, a: 255 });
    }
    {
        let fg = canvas.layer_mut(fg_idx).unwrap();
        fg.image.fill(Color { r: 0, g: 0, b: 200, a: 255 });
    }
    let result = canvas.flatten();
    // Expected: red over blue with 50% opacity on top layer.
    // Compute manually: src (blue, a=128) over dst (red, a=255)
    // Resulting color should be approx (100,0,155,255)
    let expected = Color { r: 100, g: 0, b: 155, a: 255 };
    for y in 0..2 {
        for x in 0..2 {
            assert_eq!(result.get_pixel(x, y), expected);
        }
    }

    // 4. Full canvas composite with transparent layer.
    let mut canvas2 = Canvas::new(1, 1);
    let idx0 = canvas2.add_layer(255);
    canvas2.layer_mut(idx0).unwrap().image.fill(Color::TRANSPARENT);
    let idx1 = canvas2.add_layer(255);
    canvas2.layer_mut(idx1).unwrap().image.fill(Color { r: 50, g: 100, b: 150, a: 200 });
    let flat = canvas2.flatten();
    assert_eq!(flat.get_pixel(0, 0), Color { r: 50, g: 100, b: 150, a: 200 });
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_image_fill_and_rect() {
        let mut img = Image::new(3, 3);
        img.fill(Color { r: 5, g: 5, b: 5, a: 255 });
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(img.get_pixel(x, y), Color { r: 5, g: 5, b: 5, a: 255 });
            }
        }
        img.draw_rect(0, 0, 2, 2, Color { r: 255, g: 0, b: 0, a: 128 });
        assert_eq!(img.get_pixel(0, 0), Color { r: 255, g: 0, b: 0, a: 128 });
        assert_eq!(img.get_pixel(2, 2), Color { r: 5, g: 5, b: 5, a: 255 });
    }

    #[test]
    fn test_layer_opacity() {
        let mut layer = Layer::new(1, 1, 128);
        layer.image.fill(Color { r: 0, g: 255, b: 0, a: 255 });
        let mut dest = Image::new(1, 1);
        dest.fill(Color { r: 255, g: 0, b: 0, a: 255 });
        layer.composite_onto(&mut dest);
        // Expect green over red with 50% opacity.
        let expected = Color { r: 128, g: 128, b: 0, a: 255 };
        assert_eq!(dest.get_pixel(0, 0), expected);
    }

    #[test]
    fn test_canvas_flatten_order() {
        let mut canvas = Canvas::new(2, 1);
        let bottom = canvas.add_layer(255);
        let top = canvas.add_layer(255);
        canvas.layer_mut(bottom).unwrap().image.fill(Color { r: 10, g: 10, b: 10, a: 255 });
        canvas.layer_mut(top).unwrap().image.fill(Color { r: 200, g: 0, b: 0, a: 128 });
        let out = canvas.flatten();
        // Top semi‑transparent red over dark gray.
        let expected = Color { r: 105, g: 5, b: 5, a: 255 };
        assert_eq!(out.get_pixel(0, 0), expected);
        assert_eq!(out.get_pixel(1, 0), expected);
    }
}