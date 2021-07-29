use rouille::Request;
use rouille::Response;
use serde_json::Value;
use tokio::runtime::Handle;

const API_KEY: &str = "token_artifact";

#[tokio::main]
async fn main() {
    rouille::start_server("127.0.0.1:80", |e| {
        if e.raw_url().starts_with("/kda/") {
            let player_name = e.raw_url().replace("/kda/", "");
            println!("Ask for {} kda", player_name);
            let mut res: String = String::new();
            for kda in get_kda_list(&player_name, 10) {
                res.push_str(&format!("{}\n", kda));
            }
            Response::text(res)
        } else {
            Response::text("Error, unknow path")
        }
    });
}

fn get_kda_list(player_name: &str, max_matchs: i32) -> Vec<String> {
    let mut counter: i32 = 0;
    let matchs = get_matchs(player_name);
    // println!("filter");
    let reduced_list: Vec<&String> = matchs.iter().filter_map(|e| {
        if counter < max_matchs {
            counter = counter + 1;
            Some(e)
        } else {
            None
        }
    }).collect();
    counter = 0;

    // for e in &reduced_list {
    //     println!("{}", e);
    // }

    let mut list: Vec<String> = vec![];

    for elem in reduced_list { // Change name here
        // println!("{}", elem);
        list.push(get_kda(player_name, &elem));
        // println!("{}", get_kda(player_name, &elem).await); // And here
    }

    list
}

fn get_matchs(player_name: &str) -> Vec<String> {

    let mut http_summoner_info = String::from("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/");
    http_summoner_info.push_str(player_name);

    let summoner_info_value: Value = reqwest::blocking::Client::new().get(http_summoner_info).header("X-Riot-Token", API_KEY).send().unwrap().json().unwrap();

    // let summoner_info = surf::get(http_summoner_info).header("X-Riot-Token", API_KEY).recv_string().await.unwrap();
    
    // let summoner_info_value: Value = serde_json::from_str(&summoner_info).unwrap();

    let account_id = summoner_info_value["accountId"].to_string().replace("\"", "");

    let mut http_matchs = String::from("https://euw1.api.riotgames.com/lol/match/v4/matchlists/by-account/");
    http_matchs.push_str(&account_id);
    // let matchs_info = surf::get(http_matchs).header("X-Riot-Token", API_KEY).recv_string().await.unwrap();
    let match_value: Value = reqwest::blocking::Client::new().get(http_matchs).header("X-Riot-Token", API_KEY).send().unwrap().json().unwrap();
    // let match_value: Value = serde_json::from_str(&matchs_info).unwrap();

    let mut matchsId = Vec::<String>::new();
    for elem in match_value["matches"].as_array().unwrap() {
        // println!("{}", elem["gameId"]);
        matchsId.push(elem["gameId"].to_string());
    }

    return matchsId;

}

fn get_kda(summoner_name: &str, match_id: &str) -> String {

    let mut http_game = String::from("https://euw1.api.riotgames.com/lol/match/v4/matches/");
    http_game.push_str(match_id);

    // let game_info = surf::get(http_game).header("X-Riot-Token", API_KEY).recv_string().await.unwrap();
    let game_value: Value = reqwest::blocking::Client::new().get(http_game).header("X-Riot-Token", API_KEY).send().unwrap().json().unwrap();
    // let game_value: Value = serde_json::from_str(&game_info).unwrap();

    let participant_identities: &Vec<Value> = game_value["participantIdentities"].as_array().unwrap();

    let mut participant_id: String = String::new();

    for elem in participant_identities {
        let current_participant_id = elem["participantId"].to_string();
        if elem["player"]["summonerName"] == summoner_name {
            participant_id = current_participant_id;
        }
    }

    let participants = game_value["participants"].as_array().unwrap();

    let mut kda = String::new();

    for elem in participants {
        if elem["participantId"].to_string() == participant_id {
            let kills = elem["stats"]["kills"].to_string();
            let deaths = elem["stats"]["deaths"].to_string();
            let assists = elem["stats"]["assists"].to_string();
            kda.push_str(&kills);
            kda.push_str("/");
            kda.push_str(&deaths);
            kda.push_str("/");
            kda.push_str(&assists);

        }
    }

    kda
}