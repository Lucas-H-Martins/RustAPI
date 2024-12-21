#[derive(Debug, Default)]
pub struct Database {
    pub host: String,
    pub port: String,
    pub user: String,
    pub pass: String,
    pub db: String,
}

#[derive(Debug, Default)]
pub struct Identitty {
    pub host: String,
    pub port: String,
    pub user: String,
    pub pass: String,
}
