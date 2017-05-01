use hyper;
use api::route::Route;

static HOST: &'static str = "https://api.steampowered.com";
static VERSION: &'static str = "V001";

#[derive(Debug)]
pub enum Url {
    ResolveVanityUrl,
    GetMatchHistory,
    GetMatchHistoryBySequenceNum,
    GetMatchDetails,
    GetPlayerSummaries,
    GetHeroes,
    GetFriendList,
    GetLeagueListing,
    GetLiveLeagueGames
}

impl Url {
    pub fn value(&self) -> hyper::Url {
        use api::url::Url::*;

        let path = match *self {
            ResolveVanityUrl => (Route::SteamUser, "ResolveVanityURL"),
            GetMatchHistory => (Route::Dota2Match, "GetMatchHistory"),
            GetMatchHistoryBySequenceNum => (Route::Dota2Match, "GetMatchHistoryBySequenceNum"),
            GetMatchDetails => (Route::Dota2Match, "GetMatchDetails"),
            GetPlayerSummaries => (Route::SteamUser, "GetPlayerSummaries"),
            GetHeroes => (Route::Dota2Econ, "GetHeroes"),
            GetFriendList => (Route::SteamUser, "GetFriendList"),
            GetLeagueListing => (Route::Dota2Match, "GetLeagueListing"),
            GetLiveLeagueGames => (Route::Dota2Match, "GetLiveLeagueGames")
        };
        let url_string = format!("{}/{}/{}/{}", HOST, path.0.value(), path.1, VERSION);
        let url = match hyper::Url::parse(&url_string) {
            Ok(url) => url,
            Err(_) => panic!("Unable to parse url"),
        };
        return url;
    }
}
