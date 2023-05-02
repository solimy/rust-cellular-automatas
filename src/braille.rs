pub struct Braille(pub char);

pub fn mask(arr: &[u8]) -> u8 {
    arr.iter().step_by(1).enumerate().map(|(i, x)| x << i).sum()
}

impl From<&[u8]> for Braille {
    fn from(arr: &[u8]) -> Self {
        Braille(std::char::from_u32(10240 + mask(arr) as u32).unwrap())
    }
}

#[test]
fn test_mask() {
    // for i in 0..255 {
    //     std::char::from_u32(i + 10240).unwrap();
    // }
    // for i in 0..=255 {
    //     println!("{i}: ⣿'{}'", std::char::from_u32(i + 10240).unwrap());
    // }
    // println!("'{}''{}'", std::char::from_u32(10240).unwrap(), std::char::from_u32(10241).unwrap());
    // panic!("test_braille")
}
