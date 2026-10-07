use std::cmp::min;

/// Simple RGBA color.
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,

}

impl Color {
    pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const OPAQUE_WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    pub fn blend_over(self, dst: Color) -> Color {
        // Alpha compositing: src over dst
        let src_a = self.a as u16;
        let dst_a = dst.a as u16;
        let out_a = src_a + dst_a * (255 - src_a) / 255;
        if out_a == 0 {
            return Color::TRANSPARENT;
        }
        let r = (self.r as u16 * src_a + dst.r as u16 * dst_a * (255 - src_a) / 255) / out_a;
        let g = (self.g as u16 * src_a + dst.g as u16 * dst_a * (255 - src_a) / 255) / out_a;
        let b = (self.b as u16 * src_a + dst.b as u16 * dst_a * (255 - src_a) / 255) / out_a;
        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: out_a as u8,
        }
    }
}

/// Image buffer storing RGBA pixels row‑major.
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>, // length = width * height * 4

}

impl Image {
    /// Create a new blank (transparent) image.
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![0u8; width * height * 4];
        Self { width, height, data }
    }

    /// Fill entire image with a solid color.
    pub fn fill(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color.clone());
            }
        }
    }

    /// Draw a filled rectangle (clamped to image bounds).
    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        let x_end = min(x + w, self.width);
        let y_end = min(y + h, self.height);
        for yy in y..y_end {
            for xx in x..x_end {
                self.set_pixel(xx, yy, color.clone());
            }
        }
    }

    /// Get a copy of the pixel at (x, y).
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let idx = (y * self.width + x) * 4;
        Color {
            r: self.data[idx],
            g: self.data[idx + 1],
            b: self.data[idx + 2],
            a: self.data[idx + 3],
        }
    }

    /// Set pixel at (x, y) to `color`.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let idx = (y * self.width + x) * 4;
        self.data[idx] = color.r;
        self.data[idx + 1] = color.g;
        self.data[idx + 2] = color.b;
        self.data[idx + 3] = color.a;
    }

    /// Blend `src` pixel over the current pixel at (x, y).
    pub fn blend_pixel(&mut self, x: usize, y: usize, src: Color) {
        let dst = self.get_pixel(x, y);
        let out = src.blend_over(dst);
        self.set_pixel(x, y, out);
    }
}

/// A single layer with its own image and opacity (0‑255).
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub image: Image,
    pub opacity: u8, // 0 = fully transparent, 255 = fully opaque

}

impl Layer {
    pub fn new(width: usize, height: usize, opacity: u8) -> Self {
        Self {
            image: Image::new(width, height),
            opacity,
        }
    }

    /// Blend this layer onto `dest` using the layer's opacity.
    pub fn composite_onto(&self, dest: &mut Image) {
        for y in 0..self.image.height {
            for x in 0..self.image.width {
                let src_px = self.image.get_pixel(x, y);
                if src_px.a == 0 && self.opacity == 0 {
                    continue;
                }
                // Apply layer opacity to source alpha.
                let src = Color {
                    a: ((src_px.a as u16 * self.opacity as u16) / 255) as u8,
                    ..src_px
                };
                dest.blend_pixel(x, y, src);
            }
        }
    }
}

/// Canvas holding ordered layers.
#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,

}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layers: Vec::new(),
        }
    }

    /// Add a new blank layer with given opacity.
    pub fn add_layer(&mut self, opacity: u8) -> usize {
        let layer = Layer::new(self.width, self.height, opacity);
        self.layers.push(layer);
        self.layers.len() - 1
    }

    /// Get mutable reference to a layer by index.
    pub fn layer_mut(&mut self, idx: usize) -> Option<&mut Layer> {
        self.layers.get_mut(idx)
    }

    /// Composite all layers from bottom to top into a single image.
    pub fn flatten(&self) -> Image {
        let mut result = Image::new(self.width, self.height);
        for layer in &self.layers {
            layer.composite_onto(&mut result);
        }
        result
    }
}