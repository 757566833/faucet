use rand::{thread_rng, Rng};

pub async fn rand_num() -> String {
    let mut rng = thread_rng();

    let random_number: u32 = rng.gen_range(100_000..1_000_000);
    return random_number.to_string();
}

#[cfg(test)]
mod tests {
    use crate::utils::rand::rand_num;

   

    #[tokio::test]
    async fn test_rand() {
       println!("{}", rand_num().await)
    }
}
