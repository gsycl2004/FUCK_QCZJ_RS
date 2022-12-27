
use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let args:Vec<String>= env::args().collect();
    let client = reqwest::Client::new();
    if args.len() < 3{
        println!("参数过少");
        return;
    }
    let openid = &args[1];
    let nid = &args[2];
    let card_no = &args[3];
    let token = get_access_token(&client, openid).await;
    if let Some(course) = get_current_course(&client, &token).await {
        get_join(&client,&token,&course,nid,card_no).await;
    }else{
        println!("出现了一些错误");
    }


}

fn get_time_stamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

async fn get_access_token<'a>(client: &'a Client, oid: &'a str) -> String {
    let stamp = get_time_stamp();
    let url = format!("https://qczj.h5yunban.com/qczj-youth-learning/cgi-bin/login/we-chat/callback?callback=https%3A%2F%2Fqczj
    .h5yunban.com%2Fqczj-youth-learning%2Findex.php&scope=snsapi_userinfo&appid=wx56b888a1409a2920&openid={0}
    &nickname=ZhangSan&headimg=&time={1}&source=common&sign=&t={1}", oid, stamp);
    return client.get(url.as_str()).send().await.unwrap().text().await.unwrap()[45..81].to_string();
}

async fn get_current_course(client: &Client, access_token: &String) -> Option<String> {
    let url = format!("https://qczj.h5yunban.com/qczj-youth-learning/cgi-bin/common-api/course/current?accessToken={0}", access_token);
    let raw = client.get(url).send().await.unwrap().text().await.unwrap();
    let jobj: Value = serde_json::from_str(raw.as_str()).unwrap();
    if jobj["status"].as_i64().unwrap() == 200 {
        let result = jobj["result"]["id"].as_str().unwrap();
        println!("获得最新课程:{}", result);
        return Some(result.to_string());
    } else {
        return None;
    }
}

async fn get_join(client: &Client, access_token: &String, current_course: &String, nid: &str, card_no: &str) {
    let mut map = HashMap::new();
    map.insert("course",current_course.as_str());
    map.insert("nid",nid);
    map.insert("cardNo",card_no);
    let url = format!("https://qczj.h5yunban.com/qczj-youth-learning/cgi-bin/user-api/course/join?accessToken={}",access_token);
    let result:Value= serde_json::from_str(client.post(url).json(&map).send().await.unwrap().text().await.unwrap().as_str()).unwrap();
    println!("result:{}",result);
    if result["status"].as_i64().unwrap() == 200 {
        println!("签到成功了")
    }else {
        println!("签到失败了")
    }
}