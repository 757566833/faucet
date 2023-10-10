use rand::{thread_rng, Rng};

pub fn rand_num() -> u32 {
    let mut rng = thread_rng();

    let random_number: u32 = rng.gen_range(100_000..1_000_000);
    return random_number;
}
pub fn rand_num_str() -> String {
    
    return rand_num().to_string();
}

#[cfg(test)]
mod tests {
    use crate::utils::rand::rand_num_str;

   

    #[test]
    fn test_rand() {
       println!("{}", rand_num_str())
    }
}
