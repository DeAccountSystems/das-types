#[macro_export]
macro_rules! out_point {
    ($tx_hash:expr, $index:expr) => {
        OutPoint::new_builder()
            .tx_hash(Hash::from($tx_hash))
            .index(Uint32::from($index))
            .build()
    };
}

#[macro_export]
macro_rules! script {
    ($code_hash:expr, $hash_type:expr, $args:expr) => {
        Script::new_builder()
            .code_hash(Hash::from($code_hash))
            .hash_type(Byte::new($hash_type))
            .args(Bytes::from($args))
            .build()
    };
}
