use idk::create_config_struct;

create_config_struct! {
    #[derive(Debug, Clone)]
    pub struct Config {
        pub port: u16 => "PORT", @default 8000,
        pub db_url: String => "DATABASE_URL", @expect "DATABASE_URL must be set"
    }
}

fn main() {
    let port = &DEFAULT.port; // 8000
    let db_url = &DEFAULT.db_url; // panic!

    println!("port: {} - db_url: {}", port, db_url);
}
