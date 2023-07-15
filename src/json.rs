/// Create `Deserialize`-able structs for JSON responses
#[macro_export]
macro_rules! json {
    ( $(
       $struct:ident $tt:tt
    )* ) => { $(
        json!(@single $struct $tt);
    )* };

    (@single $struct:ident { $( $key:ident : $type:ty),* $(,)? } ) => {
        /// Deserialized JSON
        #[derive(Debug, serde::Deserialize)]
        #[allow(non_snake_case)]
        pub struct $struct {
            $( pub $key: $type, )*
        }
    };

    (@single $struct:ident ( $( $type:ty ),* $(,)? ) ) => {
        /// Deserialized JSON
        #[derive(Debug, serde::Deserialize)]
        #[allow(non_snake_case)]
        pub struct $struct (
            $( pub $type, )*
        );
    };
}

