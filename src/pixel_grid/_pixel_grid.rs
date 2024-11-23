use bevy::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Component, Clone, Debug)]
pub struct PixelGrid {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl PixelGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.pixels.get(self.index(x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        let index = self.index(x, y);
        self.pixels.get_mut(index)
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl Index<(usize, usize)> for PixelGrid {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = self.index(index.0, index.1);
        &self.pixels[index]
    }
}

impl IndexMut<(usize, usize)> for PixelGrid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.index(index.0, index.1);
        &mut self.pixels[index]
    }
}
