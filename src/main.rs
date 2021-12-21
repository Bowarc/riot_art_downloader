use sublime_fuzzy::{FuzzySearch, Scoring};
use colored::Colorize;
use std::{io, env};
use image;

mod champion;

const DDRAGON_VERSION_URL: &str = "https://ddragon.leagueoflegends.com/api/versions.json";
const CHAMPION_DATA_URL: &str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion/%CHAMPION%.json";
const LANGUAGE_URL: &str = "https://ddragon.leagueoflegends.com/cdn/languages.json";
const CHAMPION_LIST_URL: &str = "https://ddragon.leagueoflegends.com/cdn/%VERSION%/data/%LANGUAGE%/champion.json";
const CHAMPION_SKIN_URL: &str = "https://ddragon.leagueoflegends.com/cdn/img/champion/splash/%CHAMPION%_%SKIN_NUMBER%.jpg";

fn get_closest_match(input: String, data: Vec<String>) -> String{
    fn new_best_match(score: isize, name: &str) -> (isize, String) {
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
                    best_match = new_best_match(score, i)
                }
            },
            None => ()
        }
        if input == i.to_lowercase(){
            best_match = new_best_match(-1, i);

            break
        }
    }
    println!("Best match: {}", best_match.1);

    best_match.1
}

async fn request_by_url(url: &str) -> reqwest::Response{
    let r: reqwest::Response = reqwest::get(url).await.expect(&format!("Couldn't get a response for the given url: \n {}.", url));

    if r.status() != 200{
        // Problem here, do something
        println!("Problem,\nStatus: {}\nURL: {}", r.status(), url);
    }

    r

}

async fn get_ddragon_version() -> String{
    let version_data: String = request_by_url(DDRAGON_VERSION_URL).await.text().await.unwrap();
    let version_list: Vec<String> = serde_json::from_str(&version_data).expect(&format!("Couldn't transfrom version data to vec of string."));

    return version_list[0].clone()
}

async fn get_language_list() -> Vec<String> {
    let languages_data: String = request_by_url(LANGUAGE_URL).await.text().await.unwrap();
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

        let closest_match = get_closest_match(user_input, language_list.clone());
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

    let champion_data: String = request_by_url(&url).await.text().await.unwrap();

    let champion_list: champion::ChampionList = serde_json::from_str(&champion_data).unwrap();
    
    champion_list
}

fn ask_input() -> String{
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input = user_input.trim_end().to_string();

    user_input
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Not enought arguments, please input the output directory name");
        std::process::exit(1)
    }
    let dir_path = &args[1];
    println!("Output directory: {}", dir_path);
    let last_ddragon_version: String = get_ddragon_version().await;

    let selected_language: String = ask_language(get_language_list().await).await;

    let champion_list: champion::ChampionList = get_champion_list(selected_language.clone(), last_ddragon_version.clone()).await;

    let mut sorted_champ_list: Vec<String> = champion_list.data.keys().cloned().collect();
    sorted_champ_list.sort();
    for champ_name in sorted_champ_list.iter(){
        println!("{}", champ_name.underline());
        let url = CHAMPION_DATA_URL
            .replace("%CHAMPION%", &champ_name)
            .replace("%LANGUAGE%", &selected_language)
            .replace("%VERSION%", &last_ddragon_version);        
        let detailled_champion_data_string = request_by_url(&url).await;

        let detailled_champion
        : champion::DetailedChampionList = serde_json::from_str(&detailled_champion_data_string.text().await.unwrap()).unwrap();

        let champion = detailled_champion.data.get(champ_name).unwrap().clone();

        for (_index, skin) in champion.skins.iter().enumerate(){
            println!("{}", skin.name);

            let skin_url = CHAMPION_SKIN_URL
                .replace("%CHAMPION%", champ_name)
                .replace("%SKIN_NUMBER%", &skin.num.to_string());

            let img_string = request_by_url(&skin_url).await;

            let image_buffer = img_string.bytes().await.unwrap(); //image::load_from_memory(&img_string.as_bytes()).unwrap();

            let img  = image::load_from_memory(&image_buffer).unwrap();

            let path = format!("{}/{}_{}.png", dir_path, champ_name, skin.num);

            img.save(path).unwrap();
        }
        println!("\n");
    }
}
