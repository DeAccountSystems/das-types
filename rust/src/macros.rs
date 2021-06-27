#[macro_export]
macro_rules! out_point {
    ($tx_hash:expr, $index:expr) => {
        OutPoint::new_builder()
            .tx_hash(Hash::from($tx_hash))
            .index(Uint32::from($index))
            .build()
    };
}