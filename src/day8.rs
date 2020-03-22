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
        let input: Vec<_> = s.chars().filter(|c| *c != '\n').collect();
        let layers = input
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

    fn decode(&self) -> String {
        let mut result = Vec::with_capacity(self.wide * self.tall);
        for i in 0..self.wide * self.tall {
            let pixel = self
                .layers
                .iter()
                .find(|layer| layer.pixels[i] != '2')
                .unwrap()
                .pixels[i];
            result.push(pixel);
        }
        result.iter().cloned().collect()
    }

    fn display(&self) {
        let message = self.decode();
        (0..self.tall).for_each(|i| {
            println!(
                "{}",
                message
                    .get(self.wide * i..self.wide * (i + 1))
                    .unwrap()
                    .replace("1", " ")
                    .replace("0", "■")
            )
        })
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

    #[test]
    fn day8_example1_part2() {
        let raw = "0222112222120000";
        let image = Image::new(raw, 2, 2);
        assert_eq!(image.decode(), "0110");
    }

    #[test]
    fn day8_part2() {
        let raw = include_str!("../input/day8.txt");
        let image = Image::new(raw, 25, 6);
        let solution = "100010110011100100011111010001100101001010001000100101010000100100101000100001001011011100001000100000100100101010000100100000010001110100100010011110";
        assert_eq!(image.decode(), solution);
    }

    #[test]
    fn day8_part3() {
        let raw = include_str!("../input/day8.txt");
        let image = Image::new(raw, 25, 6);
        image.display();
        //Outputs YGRYZ
    }
}
