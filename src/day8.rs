struct Layer {
    pixels: Vec<char>,
}

impl Layer {
    fn new(pixels: Vec<char>) -> Self {
        Layer { pixels }
    }

    fn count_zeros(&self) -> usize {
        self.pixels.iter().filter(|c| **c == '0').count()
    }

    fn count_ones(&self) -> usize {
        self.pixels.iter().filter(|c| **c == '1').count()
    }

    fn count_twos(&self) -> usize {
        self.pixels.iter().filter(|c| **c == '2').count()
    }

    fn space_image_code(&self) -> usize {
        self.count_ones() * self.count_twos()
    }
}

struct Image {
    layers: Vec<Layer>,
    wide: usize,
    tall: usize,
}

impl Image {
    fn new(s: &str, wide: usize, tall: usize) -> Self {
        let mut layers: Vec<Layer>;
        let input: Vec<_> = s.chars().filter(|c| *c != '\n').collect();
        layers = input
            .chunks(wide * tall)
            .map(|chunk| Layer::new(chunk.to_vec()))
            .collect();
        Image { layers, wide, tall }
    }

    fn anti_corruption_code(self) -> usize {
        self.layers
            .iter()
            .max_by(|a, b| b.count_zeros().cmp(&a.count_zeros()))
            .unwrap()
            .space_image_code()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day8_example1() {
        let raw = "123456789012";
        let image = Image::new(raw, 3, 2);
        assert_eq!(image.anti_corruption_code(), 1);
    }

    #[test]
    fn day8_part1() {
        let raw = include_str!("../input/day8.txt");
        let image = Image::new(raw, 25, 6);
        assert_eq!(image.anti_corruption_code(), 2684);
    }
}
