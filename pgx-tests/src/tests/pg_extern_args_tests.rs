#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_extern(immutable)]
    fn is_immutable() {}

    #[pg_test]
    fn test_immutable() {
        let result = Spi::get_one::<bool>(
            "SELECT provolatile = 'i' FROM pg_proc WHERE proname = 'is_immutable'",
        )
        .expect("failed to get SPI result");
        assert!(result)
    }
}
