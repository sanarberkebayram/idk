#[macro_export]
macro_rules! create_config_struct {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$inner:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty => $env_var:literal
                $(, @default $default_value:expr)?
                $(, @expect $expect_msg:literal)?
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis struct $name {
            $(
                $(#[$inner])*
                $field_vis $field_name: $field_type,
            )*
        }

        impl $name {
            pub fn new() -> Self {
                let _ = dotenvy::dotenv();

                $(
                    let $field_name: $field_type = $crate::create_config_struct!(@parse_env_var $field_type, $env_var
                        $(, @default $default_value)?
                        $(, @expect $expect_msg)?
                    );
                )*

                $name {
                    $(
                        $field_name,
                    )*
                }
            }
        }

        pub static DEFAULT: once_cell::sync::Lazy<$name> = once_cell::sync::Lazy::new(|| $name::new());
    };

    (@parse_env_var $field_type:ty, $env_var:literal, @default $default_value:expr) => {
        std::env::var($env_var)
            .ok()
            .and_then(|x| x.parse().ok())
            .unwrap_or($default_value)
    };

    (@parse_env_var String, $env_var:literal, @default $default_value:expr) => {
        std::env::var($env_var)
            .unwrap_or($default_value.to_string())
    };

    (@parse_env_var $field_type:ty, $env_var:literal, @expect $expect_msg:literal) => {
        std::env::var($env_var)
            .expect($expect_msg)
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse {} from {}", stringify!($field_type), $env_var))
    };

    (@parse_env_var String, $env_var:literal, @expect $expect_msg:literal) => {
        std::env::var($env_var)
            .expect($expect_msg)
    };

    (@parse_env_var $field_type:ty, $env_var:literal) => {
        std::env::var($env_var)
            .unwrap_or_else(|_| panic!("Environment variable {} must be set", $env_var))
            .parse()
            .unwrap_or_else(|_| panic!("Failed to parse {} from {}", stringify!($field_type), $env_var))
    };
}
