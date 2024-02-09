#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new();
    };
    ( $( $key:expr => $value:expr),+$(,)?) => {
        {
            let mut temp_map = ::std::collections::HashMap::new();
            $(
               temp_map.insert($key, $value);
            )*
            temp_map
        }
    };
}
