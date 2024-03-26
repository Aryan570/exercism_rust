#[macro_export]
macro_rules! hashmap {
    // + is used instead of * cause pattern should repeat atleast once
    ($($x:expr => $y:expr),+ $(,)?) => {
        {
            let mut tmp_map = ::std::collections::HashMap::new();
            $(
                tmp_map.insert($x,$y);
            )*
            tmp_map
        }
    };
    // case for single comma
    ($single:expr) => {
       compile_error!("")
    };
    // case of empty hashmap
    () => {
        ::std::collections::HashMap::new()
    }
}
