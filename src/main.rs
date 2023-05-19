fn main() {}

fn encrypt(message: &str) -> Vec<u8> {
    message.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
      let cipher = encrypt("message");
      println!("{:?}", cipher);
    }
}
