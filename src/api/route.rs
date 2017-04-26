#[derive(Debug)]
pub enum Route {
    ISteamUser,
}

impl Route {
    pub fn value(&self) -> String {
        use api::route::Route::*;

        return match *self {
            ISteamUser => "ISteamUser".to_string(),
        };
    }
}
