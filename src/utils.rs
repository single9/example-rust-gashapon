pub fn rng(seed: &mut usize) -> usize {
    *seed = (*seed).wrapping_mul(1103515245).wrapping_add(12345);
    (*seed >> 16) & 0x7FFF
}

pub fn randomize<T>(data: Vec<T>, seed: &mut usize) -> Vec<T>
where
    T: Clone,
{
    let mut item = data.clone();
    for i in (0..item.len()).rev() {
        let j = rng(seed) % (i + 1);
        item.swap(i, j);
    }
    item.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        let mut seed = 12345;
        let random_number = rng(&mut seed);
        assert_eq!(random_number, 21468);
    }

    #[test]
    fn test_randomize() {
        let mut seed = 12345;
        let data = vec![1, 2, 3, 4, 5];
        let randomized_data = randomize(data.clone(), &mut seed);
        assert_ne!(randomized_data, data);
        assert_eq!(randomized_data.len(), data.len());
    }
}
