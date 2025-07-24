pub mod db;
pub mod tools;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup() {
        db::db().await;
        assert!(false)
    }
}
