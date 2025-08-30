#[allow(
    unused_variables,
    dead_code,
    unreachable_code,
    missing_docs,
    unused_mut,
    unused_imports
)]
#[macro_export]
// use Copy;
// use Sync;
// use Send;
macro_rules! record {
    (serde_def, $struct_name:ident {$( $field_name:ident : $field_type:ty = $field_val:expr ),* $(,)? }) => {
        // #[derive(Debug,Serialize,Deserialize)]
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $struct_name {
            $(
                $field_name: $field_type
            ),*
        }
        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name {
                    $( $field_name : $field_val ),*
                }
            }
        }
    };

    (serde, $struct_name:ident {$( $field_name:ident : $field_type:ty = $field_val:expr ),* $(,)? }) => {
        // #[derive(Debug,Serialize,Deserialize)]
        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct $struct_name {
            $(
                $field_name: $field_type
            ),*
        }
    };

    ($struct_name:ident { $( $field_name:ident : $field_type:ty ),* $(,)?}) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(
                pub $field_name : $field_type
            ),*
        }
    };
}
