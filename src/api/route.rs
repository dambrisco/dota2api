#[derive(Debug)]
pub enum Route {
    SteamUser,
    Dota2Match,
    Dota2Econ
}

impl Route {
    pub fn value(&self) -> &str {
        use api::route::Route::*;

        return match *self {
            SteamUser => "ISteamUser",
            Dota2Match => "IDOTA2Match_570",
            Dota2Econ => "IEconDOTA2_570"
        };
    }
}
