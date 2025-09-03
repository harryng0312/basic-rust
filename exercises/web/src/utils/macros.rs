#[allow(warnings)]
// #[allow(
//     unused_variables,
//     dead_code,
//     unreachable_code,
//     missing_docs,
//     unused_mut,
//     unused_imports
// )]
#[macro_export]
macro_rules! record {
    {
        $(#[$attr:meta])*
        $s_name:ident { $( $f_name:ident : $f_type:ty),* $(,)? }} => {paste::paste! {
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            $(#[$attr])*
            pub struct $s_name {
                $( pub $f_name: $f_type ),*
            }

            impl $s_name {
                pub fn new($( $f_name: $f_type ),*) -> Self {
                    Self {
                        $( $f_name ),*
                    }
                }

                $(
                pub fn $f_name(&self) -> &$f_type {
                    &self.$f_name
                }

                pub fn [<set_ $f_name>] (&mut self, val: $f_type) {
                    self.$f_name = val;
                }
                )*
        }}
    };
}
