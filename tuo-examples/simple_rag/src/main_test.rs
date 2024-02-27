#[cfg(test)]
mod tests {
    use crate::read_file;
    use super::*;

    #[tokio::test]
    async fn test_main() {
       let result =  read_file().await;
        assert!(result.is_ok());
    }
}