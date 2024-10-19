use std::str::FromStr;

use reqwest::*;
use serde_json::*;
// use futures::executor::block_on;

fn get_json() -> String {
    let _client = Client::new();
    let url = "https://s3.eu-west-1.amazonaws.com/hackajob-assets1.p.hackajob/challenges/football_session/football.json";
    let method = Method::GET;
    let _request = Request::new(method, url.try_into().unwrap());
    let get = reqwest::blocking::get(url);
    get.expect("Expected response").text().unwrap()
    
}

#[allow(unused_variables)]
fn run(team_key: &'static str) -> i32 {
	//
	// Write your code below; return type and arguments should be according to the problem\'s requirements
	//
    let json_string = get_json();
    let json = serde_json::Value::from_str(&json_string).unwrap();
    let match_days = json["rounds"].as_array().unwrap();
    let matches = match_days.clone().into_iter().enumerate().flat_map(|(index,match_day)| {
        let vec = {
            //println!("{}{}",index,match_day);
            let match_day = match_days[index]["matches"].clone();
            let iter = match_day.as_array().unwrap().iter();
            let map = iter.cloned();
            map.collect::<Vec<Value>>() 
        };
        vec
    }).collect::<Vec<Value>>();

    let filtered_matches = matches.into_iter().clone().filter(|game| game["team1"]["name"] == team_key || game["team2"]["team2"] == team_key  ).collect::<Vec<Value>>();
    println!("{:#?}",filtered_matches);

    let goals: i64 = filtered_matches.into_iter().map(|game| {
        if game["team1"]["name"] == team_key {
            game["score1"].as_i64().unwrap() 
        } else {
            game["score2"].as_i64().unwrap()
        }
    }).sum();
    goals as i32
}

fn main(){
    println!("{:#?}", run("Arsenal"));
}