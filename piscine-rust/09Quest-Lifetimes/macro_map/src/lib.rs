#[macro_export]
macro_rules! hash_map {
    // empty
    () => {
        std::collections::HashMap::new()
    };
    // trailing comma
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($k, $v);)+
            map
        }
    };
}
