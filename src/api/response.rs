#[derive(Debug, Deserialize)]
pub struct Vanity {
    #[serde(rename="response")]
    pub response: VanityResponse,
}

#[derive(Debug, Deserialize)]
pub struct VanityResponse {
    #[serde(rename="steamid")]
    pub steam_id: String,
    #[serde(rename="success")]
    pub success: i32,
    #[serde(rename="message")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatchHistory {
    #[serde(rename="result")]
    pub result: MatchHistoryResult,
}

#[derive(Debug, Deserialize)]
pub struct MatchHistoryResult {
    #[serde(rename="status")]
    pub status: u32,
    #[serde(rename="league_id")]
    pub league_id: Option<u64>,
    #[serde(rename="num_results")]
    pub num_results: u32,
    #[serde(rename="total_results")]
    pub total_results: u32,
    #[serde(rename="results_remaining")]
    pub results_remaining: u32,
    #[serde(rename="matches")]
    pub matches: Vec<MatchSummary>
}

#[derive(Debug, Deserialize)]
pub struct MatchSummary {
    #[serde(rename="series_id")]
    pub series_id: Option<u32>,
    #[serde(rename="series_type")]
    pub series_type: Option<u32>,
    #[serde(rename="match_id")]
    pub match_id: u64,
    #[serde(rename="match_seq_num")]
    pub match_seq_num: u64,
    #[serde(rename="start_time")]
    pub start_time: u32,
    #[serde(rename="lobby_type")]
    pub lobby_type: u32,
    #[serde(rename="radiant_team_id")]
    pub radiant_team_id: u32,
    #[serde(rename="dire_team_id")]
    pub dire_team_id: u32,
    #[serde(rename="players")]
    pub players: Vec<PlayerSummary>
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummary {
    #[serde(rename="account_id")]
    pub account_id: u32,
    #[serde(rename="player_slot")]
    pub player_slot: u32,
    #[serde(rename="hero_id")]
    pub hero_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct MatchDetails {
    #[serde(rename="result")]
    pub result: Match,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    #[serde(rename="error")]
    pub error: String,
    #[serde(rename="players")]
    pub players: Vec<Player>,
    #[serde(rename="radiant_win")]
    pub radiant_win: bool,
    #[serde(rename="duration")]
    pub duration: u32,
    #[serde(rename="pre_game_duration")]
    pub pregame_duration: u32,
    #[serde(rename="start_time")]
    pub start_time: u32,
    #[serde(rename="match_id")]
    pub match_id: u64,
    #[serde(rename="match_seq_num")]
    pub match_seq_num: u64,
    #[serde(rename="tower_status_radiant")]
    pub tower_status_radiant: u32,
    #[serde(rename="tower_status_dire")]
    pub tower_status_dire: u32,
    #[serde(rename="barracks_status_radiant")]
    pub barracks_status_radiant: u32,
    #[serde(rename="barracks_status_dire")]
    pub barracks_status_dire: u32,
    #[serde(rename="cluster")]
    pub cluster: u32,
    #[serde(rename="first_blood_time")]
    pub first_blood_time: u32,
    #[serde(rename="lobby_type")]
    pub lobby_type: u32,
    #[serde(rename="human_players")]
    pub human_players: u32,
    #[serde(rename="league_id")]
    pub league_id: u32,
    #[serde(rename="positive_votes")]
    pub positive_votes: u32,
    #[serde(rename="negative_votes")]
    pub negative_votes: u32,
    #[serde(rename="game_mode")]
    pub game_mode: u32,
    #[serde(rename="flags")]
    pub flags: u32,
    #[serde(rename="engine")]
    pub engine: u32,
    #[serde(rename="radiant_score")]
    pub radiant_score: u32,
    #[serde(rename="dire_score")]
    pub dire_score: u32,
    #[serde(rename="tournament_id")]
    pub tournament_id: u32,
    #[serde(rename="tournament_round")]
    pub tournament_round: u32,
    #[serde(rename="radiant_team_id")]
    pub radiant_team_id: u32,
    #[serde(rename="radiant_name")]
    pub radiant_name: String,
    #[serde(rename="radiant_logo")]
    pub radiant_logo: u32,
    #[serde(rename="radiant_team_complete")]
    pub radiant_team_complete: u32,
    #[serde(rename="dire_team_id")]
    pub dire_team_id: u32,
    #[serde(rename="dire_name")]
    pub dire_name: String,
    #[serde(rename="dire_logo")]
    pub dire_logo: u32,
    #[serde(rename="dire_team_complete")]
    pub dire_team_complete: u32,
    #[serde(rename="radiant_captain")]
    pub radiant_captain: u32,
    #[serde(rename="dire_captain")]
    pub dire_captain: u32,
    #[serde(rename="picks_bans")]
    pub picks_bans: Vec<PicksBans>
}

#[derive(Debug, Deserialize)]
pub struct PicksBans {
    #[serde(rename="is_pick")]
    pub is_pick: bool,
    #[serde(rename="hero_id")]
    pub hero_id: u32,
    #[serde(rename="team")]
    pub team: u32,
    #[serde(rename="order")]
    pub order: u32,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    #[serde(rename="account_id")]
    pub account_id: u32,
    #[serde(rename="player_slot")]
    pub player_slot: u32,
    #[serde(rename="hero_id")]
    pub hero_id: u32,
    #[serde(rename="item_0")]
    pub item0: u32,
    #[serde(rename="item_1")]
    pub item1: u32,
    #[serde(rename="item_2")]
    pub item2: u32,
    #[serde(rename="item_3")]
    pub item3: u32,
    #[serde(rename="item_4")]
    pub item4: u32,
    #[serde(rename="item_5")]
    pub item5: u32,
    #[serde(rename="kills")]
    pub kills: u32,
    #[serde(rename="deaths")]
    pub deaths: u32,
    #[serde(rename="assists")]
    pub assists: u32,
    #[serde(rename="leaver_status")]
    pub leaver_status: u32,
    #[serde(rename="last_hits")]
    pub last_hits: u32,
    #[serde(rename="denies")]
    pub denies: u32,
    #[serde(rename="gold_per_min")]
    pub gold_per_min: u32,
    #[serde(rename="xp_per_min")]
    pub xp_per_min: u32,
    #[serde(rename="level")]
    pub level: u32,
    #[serde(rename="gold")]
    pub gold: u32,
    #[serde(rename="gold_spent")]
    pub gold_spent: u32,
    #[serde(rename="hero_damage")]
    pub hero_damage: u32,
    #[serde(rename="tower_damage")]
    pub tower_damage: u32,
    #[serde(rename="hero_healing")]
    pub hero_healing: u32,
    #[serde(rename="ability_upgrades")]
    pub ability_upgrades: Vec<AbilityUpgrade>,
}

#[derive(Debug, Deserialize)]
pub struct AbilityUpgrade {
    #[serde(rename="ability")]
    pub ability: u32,
    #[serde(rename="level")]
    pub level: u32,
    #[serde(rename="time")]
    pub time: u32,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummaries {
    #[serde(rename="response")]
    pub response: PlayerSummariesResponse
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummariesResponse {
    #[serde(rename="players")]
    pub players: Vec<PlayerAccount>
}


#[derive(Debug, Deserialize)]
pub struct PlayerAccount {
    #[serde(rename="steamid")]
    pub steam_id: String,
    #[serde(rename="communityvisibilitystate")]
    pub community_visibility_state: u32,
    #[serde(rename="profilestate")]
    pub profile_state: u32,
    #[serde(rename="personaname")]
    pub persona_name: String,
    #[serde(rename="lastlogoff")]
    pub last_logoff: u32,
    #[serde(rename="profileurl")]
    pub profile_url: String,
    #[serde(rename="avatar")]
    pub avatar: String,
    #[serde(rename="avatarmedium")]
    pub avatar_medium: String,
    #[serde(rename="avatarfull")]
    pub avatar_full: String,
    #[serde(rename="personastate")]
    pub persona_state: u32,
    #[serde(rename="realname")]
    pub real_name: String,
    #[serde(rename="primaryclanid")]
    pub primary_clan_id: String,
    #[serde(rename="timecreated")]
    pub time_created: u32,
    #[serde(rename="personastateflags")]
    pub persona_state_flags: u32,
    #[serde(rename="gameextrainfo")]
    pub game_extra_info: String,
    #[serde(rename="gameid")]
    pub game_id: String,
    #[serde(rename="loccountrycode")]
    pub location_country_code: String,
    #[serde(rename="locstatecode")]
    pub locaction_state_code: String,
}


#[derive(Debug, Deserialize)]
pub struct Heroes {
    #[serde(rename="result")]
    pub result: HeroesResult
}

#[derive(Debug, Deserialize)]
pub struct HeroesResult {
    #[serde(rename="count")]
    pub count: i32,
    #[serde(rename="heroes")]
    pub heroes: Vec<Hero>,
    #[serde(rename="status")]
    pub status: u32
}

#[derive(Debug, Deserialize)]
pub struct Hero {
    #[serde(rename="id")]
    pub id: i32,
    #[serde(rename="name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct FriendList {
    #[serde(rename="friendslist")]
    pub friends_list: FriendsList
}

#[derive(Debug, Deserialize)]
pub struct FriendsList {
    #[serde(rename="friends")]
    pub friends: Vec<Friend>
}

#[derive(Debug, Deserialize)]
pub struct Friend {
    #[serde(rename="steamid")]
    pub steam_id: String,
    #[serde(rename="relationship")]
    pub relationship: String,
    #[serde(rename="friend_since")]
    pub friend_since: u64
}

#[derive(Debug, Deserialize)]
pub struct LeagueList {
    #[serde(rename="result")]
    pub result: LeagueListResult
}

#[derive(Debug, Deserialize)]
pub struct LeagueListResult {
    #[serde(rename="leagues")]
    pub leagues: Vec<League>
}

#[derive(Debug, Deserialize)]
pub struct League {
    #[serde(rename="name")]
    pub name: String,
    #[serde(rename="leagueid")]
    pub league_id: u32,
    #[serde(rename="description")]
    pub description: String,
    #[serde(rename="tournament_url")]
    pub tournament_url: String,
    #[serde(rename="itemdef")]
    pub item_def: u32,
}

#[derive(Debug, Deserialize)]
pub struct LiveGames {
    #[serde(rename="result")]
    pub result: LiveGamesResult
}

#[derive(Debug, Deserialize)]
pub struct LiveGamesResult {
    #[serde(rename="games")]
    pub games: Vec<Game>,
    #[serde(rename="status")]
    pub status: u32,
}

#[derive(Debug, Deserialize)]
pub struct Game {
    #[serde(rename="players")]
    pub players: Vec<GamePlayers>,
    #[serde(rename="lobby_id")]
    pub lobby_id: u64,
    #[serde(rename="match_id")]
    pub match_id: u64,
    #[serde(rename="spectators")]
    pub spectators: u32,
    #[serde(rename="series_id")]
    pub series_id: u32,
    #[serde(rename="game_number")]
    pub game_number: u32,
    #[serde(rename="league_id")]
    pub league_id: Option<u32>,
    #[serde(rename="stream_delay_s")]
    pub stream_delay_seconds: u32,
    #[serde(rename="radiant_series_wins")]
    pub radiant_series_wins: u32,
    #[serde(rename="dire_series_wins")]
    pub dire_series_wins: u32,
    #[serde(rename="series_type")]
    pub series_type: u32,
    #[serde(rename="league_series_id")]
    pub league_series_id: u32,
    #[serde(rename="league_game_id")]
    pub league_game_id: u32,
    #[serde(rename="stage_name")]
    pub stage_name: String,
    #[serde(rename="league_tier")]
    pub league_tier: u32,
    #[serde(rename="scoreboard")]
    pub scoreboard: Scoreboard,
    #[serde(rename="dire_team")]
    pub dire_team: PlayersTeam,
    #[serde(rename="radiant_team")]
    pub radiant_team: PlayersTeam
}

#[derive(Debug, Deserialize)]
pub struct GamePlayers {
    #[serde(rename="account_id")]
    pub account_id: u32,
    #[serde(rename="name")]
    pub name: String,
    #[serde(rename="hero_id")]
    pub hero_id: u32,
    #[serde(rename="team")]
    pub team: u32,
}

#[derive(Debug, Deserialize)]
pub struct PlayersTeam {
    #[serde(rename="team_name")]
    pub team_name: String,
    #[serde(rename="team_id")]
    pub team_id: u32,
    #[serde(rename="team_logo")]
    pub team_logo: u64,
    #[serde(rename="complete")]
    pub complete: bool,
}

#[derive(Debug, Deserialize)]
pub struct Scoreboard {
    #[serde(rename="duration")]
    pub duration: f64,
    #[serde(rename="roshan_respawn_timer")]
    pub roshan_respawn_timer: u32,
    #[serde(rename="radiant")]
    pub radiant: TeamState,
    #[serde(rename="dire")]
    pub dire: TeamState,
}

#[derive(Debug, Deserialize)]
pub struct TeamState {
    #[serde(rename="score")]
    pub score: u32,
    #[serde(rename="tower_state")]
    pub tower_state: u32,
    #[serde(rename="barracks_state")]
    pub barracks_state: u32,
    #[serde(rename="picks")]
    pub picks: Vec<HeroSelection>,
    #[serde(rename="bans")]
    pub bans: Vec<HeroSelection>,
    #[serde(rename="players")]
    pub players: Vec<LivePlayer>,
    #[serde(rename="abilities")]
    pub abilities: Vec<LiveAbility>
}

#[derive(Debug, Deserialize)]
pub struct HeroSelection {
    #[serde(rename="hero_id")]
    pub hero_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct LivePlayer {
    #[serde(rename="player_slot")]
    pub player_slot: u32,
    #[serde(rename="account_id")]
    pub account_id: u32,
    #[serde(rename="hero_id")]
    pub hero_id: u32,
    #[serde(rename="kills")]
    pub kills: u32,
    #[serde(rename="death")]
    pub death: u32,
    #[serde(rename="assists")]
    pub assists: u32,
    #[serde(rename="last_hits")]
    pub last_hits: u32,
    #[serde(rename="denies")]
    pub denies: u32,
    #[serde(rename="gold")]
    pub gold: u32,
    #[serde(rename="level")]
    pub level: u32,
    #[serde(rename="gold_per_min")]
    pub gold_per_min: u32,
    #[serde(rename="xp_per_min")]
    pub xp_per_min: u32,
    #[serde(rename="ultimate_state")]
    pub ultimate_state: u32,
    #[serde(rename="ultimate_cooldown")]
    pub ultimate_cooldown: u32,
    #[serde(rename="item0")]
    pub item0: u32,
    #[serde(rename="item1")]
    pub item1: u32,
    #[serde(rename="item2")]
    pub item2: u32,
    #[serde(rename="item3")]
    pub item3: u32,
    #[serde(rename="item4")]
    pub item4: u32,
    #[serde(rename="item5")]
    pub item5: u32,
    #[serde(rename="respawn_timer")]
    pub respawn_timer: u32,
    #[serde(rename="position_x")]
    pub position_x: f64,
    #[serde(rename="position_y")]
    pub position_y: f64,
    #[serde(rename="net_worth")]
    pub networth: u32,
}

#[derive(Debug, Deserialize)]
pub struct LiveAbility {
    #[serde(rename="ability_id")]
    pub ability_id: u32,
    #[serde(rename="ability_level")]
    pub ability_level: u32,
}
