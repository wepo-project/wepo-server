/// 通知的类型
#[macro_export]
macro_rules! define_num_enum {
    (
        $(#[$outer:meta])*
        $name:ident {$(
        $(#[$inner:meta])*
        [$type:ident => $num:expr]
    ),* $(,)?}) => {
        $(#[$outer])*
        #[allow(dead_code)]
        pub enum $name {
            $(
                $(#[$inner])*
                $type,
            )*
        }
        impl $name {
            pub fn to_i16(&self) -> &'static i16 {
                match self {
                    $(
                        $name::$type => &$num,
                    )*
                }
            }
        }
    };
}