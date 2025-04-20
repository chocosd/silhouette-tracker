use image::{GrayImage, Luma};
use ndarray::Array2;

/// A grayscale matrix of pixel values (0.0 to 255.0)
#[derive(Debug, Clone)]
pub struct GrayMatrix(pub Array2<f32>);

/// A wrapper around a processed silhouette image
#[derive(Debug, Clone)]
pub struct Silhouette(pub GrayImage);
