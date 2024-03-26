//my solution
#[macro_export]
macro_rules! hashmap {
    // + is used instead of * cause pattern should repeat at least once
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
//Some random guy on exercism
#[macro_export]
macro_rules! hashmap {
    () => { ::std::collections::HashMap::new() };
    ($( $key: expr => $val: expr ),+ $(,)?) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

