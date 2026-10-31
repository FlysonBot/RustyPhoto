//! The document model: a canvas size and a stack of pixel layers.
//!
//! This is a deliberately small starting point. Pixel format, bit depth,
//! masks, groups and tiling are open decisions (see docs/decisions/).

use std::fmt;

/// One RGBA pixel, 8 bits per channel, straight (not premultiplied) alpha.
pub type Rgba8 = [u8; 4];

/// Largest width or height a document may have, in pixels.
pub const MAX_DIMENSION: u32 = 30_000;

/// Errors from document operations. Product code returns these instead of panicking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocError {
    /// Width or height is zero or above [`MAX_DIMENSION`].
    BadSize { width: u32, height: u32 },
    /// A layer index that does not exist.
    NoSuchLayer(usize),
}

impl fmt::Display for DocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocError::BadSize { width, height } => write!(
                f,
                "size {width}x{height} is invalid (each side must be 1..={MAX_DIMENSION})"
            ),
            DocError::NoSuchLayer(i) => write!(f, "layer {i} does not exist"),
        }
    }
}

impl std::error::Error for DocError {}

/// A single raster layer.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    /// 0.0 (transparent) to 1.0 (opaque).
    pub opacity: f32,
    /// Row-major pixels, `width * height` entries.
    pub pixels: Vec<Rgba8>,
}

/// An image document.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    width: u32,
    height: u32,
    /// Bottom layer first.
    pub layers: Vec<Layer>,
    /// Index of the layer that tools and commands act on by default.
    pub active: usize,
}

fn check_size(width: u32, height: u32) -> Result<(), DocError> {
    if width == 0 || height == 0 || width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(DocError::BadSize { width, height });
    }
    Ok(())
}

impl Document {
    /// A new document with one layer filled with `fill`.
    pub fn new(width: u32, height: u32, fill: Rgba8) -> Result<Self, DocError> {
        check_size(width, height)?;
        let mut doc = Document {
            width,
            height,
            layers: Vec::new(),
            active: 0,
        };
        doc.add_layer("Background", fill);
        Ok(doc)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Number of pixels in one layer.
    pub fn pixel_count(&self) -> usize {
        (self.width as usize).saturating_mul(self.height as usize)
    }

    /// Adds a layer filled with `fill` above all others and makes it active.
    pub fn add_layer(&mut self, name: &str, fill: Rgba8) -> usize {
        self.layers.push(Layer {
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            pixels: vec![fill; self.pixel_count()],
        });
        self.active = self.layers.len().saturating_sub(1);
        self.active
    }

    pub fn layer(&self, index: usize) -> Result<&Layer, DocError> {
        self.layers.get(index).ok_or(DocError::NoSuchLayer(index))
    }

    pub fn layer_mut(&mut self, index: usize) -> Result<&mut Layer, DocError> {
        self.layers
            .get_mut(index)
            .ok_or(DocError::NoSuchLayer(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document_has_one_filled_layer() {
        let doc = Document::new(4, 3, [255, 255, 255, 255]).unwrap();
        assert_eq!(doc.layers.len(), 1);
        assert_eq!(doc.layers[0].pixels.len(), 12);
        assert_eq!(doc.active, 0);
    }

    #[test]
    fn bad_sizes_are_errors_not_crashes() {
        assert!(Document::new(0, 10, [0; 4]).is_err());
        assert!(Document::new(10, MAX_DIMENSION + 1, [0; 4]).is_err());
    }

    #[test]
    fn missing_layer_is_an_error() {
        let doc = Document::new(2, 2, [0; 4]).unwrap();
        assert_eq!(doc.layer(5), Err(DocError::NoSuchLayer(5)));
    }
}
