// Tests
#[cfg(test)]
mod tests {

    use image::open;

    use crate::{blend_ops::blend, pixelops::{pixel_add, pixel_mult, pixel_sub}};

    #[test]
    fn test_add() {
        let mut img1 = open("test_data/1.jpg").unwrap().into_rgba8();
        let img2 = open("test_data/2.jpg").unwrap().into_rgba8();
        blend(&mut img1, &img2, pixel_add).unwrap();

        img1.save("test_out/add1.png").unwrap();
    }
    #[test]
    fn test_sub() {
        let mut img1 = open("test_data/1.jpg").unwrap().into_rgba8();
        let img2 = open("test_data/2.jpg").unwrap().into_rgba8();
        blend(&mut img1, &img2, pixel_sub).unwrap();

        img1.save("test_out/sub1.png").unwrap();
    }
    #[test]
    fn test_mult() {
        let mut img1 = open("test_data/1.jpg").unwrap().into_rgba8();
        let img2 = open("test_data/2.jpg").unwrap().into_rgba8();
        blend(&mut img1, &img2, pixel_mult).unwrap();

        img1.save("test_out/mult1.png").unwrap();
    }
}
