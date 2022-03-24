use crate::Robot;
use reqwest::header::HeaderMap;
use reqwest::header::AUTHORIZATION;
use rocket::serde::json::Value;
use std::collections::HashMap;

//   "app_id": "cli_a2a1b2a7d079100b",
//   "app_secret": "w1Lsda1m0uLT3a8prcBEHgpAUiXiqIYr"

const APP_ID: &'static str = "cli_a2a1b2a7d079100b";
const APP_SECRET: &'static str = "w1Lsda1m0uLT3a8prcBEHgpAUiXiqIYr";

// 获取机器人token
pub async fn get_token() -> String {
    let client = reqwest::Client::new();
    let mut data = HashMap::new();
    data.insert("app_id", APP_ID);
    data.insert("app_secret", APP_SECRET);
    let res = client
        .get("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal")
        .json(&data)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    let token = res.get("tenant_access_token").unwrap().as_str().unwrap();
    token.to_string()
}

// 发送信息
pub async fn send_message(robot: &Robot, chat_id: &str, chat_name: &str) {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    let token = format!("Bearer {}", robot.token);
    headers.insert(AUTHORIZATION, token.parse().unwrap());
    let mut data = HashMap::new();
    let content = format!(
        "{{\"text\":\"大家好我是【{}】机器人，很高兴加入【{}】群\"}}",
        robot.name, chat_name
    );
    println!("{content}");
    data.insert("receive_id", chat_id);
    data.insert("content", &content);
    data.insert("msg_type", "interactive");
    let _res = client
        .post("https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type=chat_id")
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
}

// 发送信息
pub async fn send_card(robot: &Robot, chat_id: &str, card: Value) {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    let token = format!("Bearer {}", robot.token);
    headers.insert(AUTHORIZATION, token.parse().unwrap());
    let mut data = HashMap::new();

    let content = format!("{}", card);
    data.insert("receive_id", chat_id);
    data.insert("content", &content);
    data.insert("msg_type", "interactive");
    println!("{data:#?}");
    let res = client
        .post("https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type=chat_id")
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    println!("{res}");
}
