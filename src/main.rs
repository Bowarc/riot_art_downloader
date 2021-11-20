// struct riot_bg_downloader {}
// use serde_json::Value; //Result

use sublime_fuzzy::{FuzzySearch, Scoring};
use std::io;

mod champion;

const DDRAGON_VERSION_URL: &str = "https://ddragon.leagueoflegends.com/api/versions.json";
const CHAMPION_DATA_URL: &str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion/%CHAMPION%.json";
const LANGUAGE_URL: &str = "https://ddragon.leagueoflegends.com/cdn/languages.json";
const CHAMPION_LIST_URL:&str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion.json";

fn getClosestMatch(input: String, data: Vec<String>) -> String{
    fn newBestMatch(score: isize, name: &str) -> (isize, String) {
        (score, name.to_string())
    }

    let mut best_match: (isize, String) = (0, String::new());

    for i in data.iter(){
        let result = FuzzySearch::new(&input, &i.to_lowercase())
            .score_with(&Scoring::emphasize_word_starts())
            .best_match();

        match result{
            Some(r) => {
                let score = r.score();
                if score > best_match.0{
                    best_match = newBestMatch(score, i)
                }
            },
            None => ()
        }
        if input == i.to_lowercase(){
            best_match = newBestMatch(-1, i);

            break
        }
    }
    println!("Best match: {}", best_match.1);

    best_match.1
}

async fn requestByURL(url: &str) -> String{
    let r: reqwest::Response = reqwest::get(url).await.expect(&format!("Couldn't get a response for the given url: \n {}.", url));

    if r.status() != 200{
        // Problem here, do something
        println!("Problem,\nStatus: {}\nURL: {}", r.status(), url);
    }

    let response_text: String = r.text().await.expect(&format!("Couldn't get the text from the response."));

    response_text
}

async fn get_ddragon_version() -> String{
    let version_data: String = requestByURL(DDRAGON_VERSION_URL).await;
    let version_list: Vec<String> = serde_json::from_str(&version_data).expect(&format!("Couldn't transfrom version data to vec of string."));

    // self.ddragon_latest_version = version_list[0].clone();

    // println!("version: {}", self.ddragon_latest_version);

    return version_list[0].clone()
}

async fn get_language_list() -> Vec<String> {
    let languages_data: String = requestByURL(LANGUAGE_URL).await;
    let language_list: Vec<String> = serde_json::from_str(&languages_data).expect(&format!("Couldn't create a json object form the response's text."));

    language_list
}

async fn ask_language(language_list: Vec<String>) -> String {
    if language_list.is_empty(){
            panic!("Yikes, the language_list is empty");
        }
        let mut selected_language = String::new();

        while selected_language == String::new(){
            for lang in language_list.chunks(5){
                let s = format!("{:02?}", lang).replace('"', "").replace("[", "").replace("]", "").replace(" ", "  ");

                println!("{}",s);
            }
            println!("Please choose a language from the list above ^:");

            let user_input = ask_input();

            let closest_match = getClosestMatch(user_input, language_list.clone());
            if closest_match == String::new(){
                println!("Failes to recognise the selected_language.");
            }else{
                selected_language = closest_match;
            }
        }
        selected_language

}

async fn get_champion_list(selected_language: String, ddragon_version: String) ->  champion::ChampionList{
    let url = CHAMPION_LIST_URL.replace("%LANGUAGE%", &selected_language).replace("%VERSION%", &ddragon_version);

    let champion_data: String = requestByURL(&url).await;

    let champion_list: champion::ChampionList = serde_json::from_str(&champion_data).unwrap();
    
    champion_list
    // self.champion_list = champion_list;
}

fn ask_input() -> String{
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input = user_input.trim_end().to_string();

    user_input
}


#[tokio::main]
async fn main() {
    let last_ddragon_version: String = get_ddragon_version().await;

    let selected_language: String = ask_language(get_language_list().await).await;

    let champion_list: champion::ChampionList = get_champion_list(selected_language.clone(), last_ddragon_version.clone()).await;

    let mut sorted_champ_list: Vec<String> = champion_list.data.keys().cloned().collect();
    sorted_champ_list.sort();
    for champ in sorted_champ_list.iter(){
        let url = CHAMPION_DATA_URL
            .replace("%CHAMPION%", &champ)
            .replace("%LANGUAGE%", &selected_language)
            .replace("%VERSION%", &last_ddragon_version);
        println!("url: {}", url);
    }
}

// for (champ_name, _champ_data) in champion_list.data.iter(){
//     c +=1;
//     let url = CHAMPION_DATA_URL
//         .replace("%CHAMPION%", &champ_name)
//         .replace("%LANGUAGE%", &selected_language)
//         .replace("%VERSION%", &last_ddragon_version);
//     println!("url: {}", url);
//     // let skin_list = requestByURL(&url).await;
// }