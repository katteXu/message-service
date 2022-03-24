use chrono::Local;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{catch, catchers, post, routes};

mod api;
mod codeup;
use codeup::Flow;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(crate = "rocket::serde", default)]
struct Event {
    app_id: String,
    chat_i18n_names: Value,
    chat_name: String,
    chat_owner_employee_id: String,
    chat_owner_name: String,
    chat_owner_open_id: String,
    open_chat_id: String,
    operator_employee_id: String,
    operator_name: String,
    operator_open_id: String,
    owner_is_bot: bool,
    tenant_key: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct CallBackParams {
    uuid: String,
    event: Event,
    token: String,
    ts: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(crate = "rocket::serde", default)]
struct Context {
    challenge: String,
    token: String,
    r#type: String,
    encrypt: String,
    event: Event,
    ts: String,
    uuid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(crate = "rocket::serde", default)]
struct Task {
    task: Value,
    sources: Value,
    #[serde(rename = "globalParams")]
    global_params: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Return {
    challenge: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Robot {
    name: String,
    avatar: String,
    open_id: String,
    chat_list: Vec<String>,
    token: String,
}

impl Robot {
    async fn new() -> Self {
        println!("初始化机器人");
        let token = api::get_token().await;
        let bot = Self::get_info(&token).await;
        let chat_list = Self::get_chat_list(&token).await;
        println!("{bot},{chat_list:?}");
        Robot {
            name: bot.get("app_name").unwrap().as_str().unwrap().to_string(),
            avatar: bot.get("avatar_url").unwrap().as_str().unwrap().to_string(),
            open_id: bot.get("open_id").unwrap().as_str().unwrap().to_string(),
            chat_list,
            token,
        }
    }

    async fn get_info(token: &str) -> Value {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        let token = format!("Bearer {}", token);
        headers.insert(AUTHORIZATION, token.parse().unwrap());
        let res = client
            .get("https://open.feishu.cn/open-apis/bot/v3/info")
            .headers(headers)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        if *res.get("msg").unwrap() == json!("ok") {
            let bot = res.get("bot");
            if let Some(bot) = bot {
                return json!(bot);
            }
        }
        json!({})
    }

    async fn get_chat_list(token: &str) -> Vec<String> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        let token = format!("Bearer {}", token);
        headers.insert(AUTHORIZATION, token.parse().unwrap());
        let res = client
            .get("https://open.feishu.cn/open-apis/im/v1/chats")
            .headers(headers)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        if *res.get("msg").unwrap() == json!("ok") {
            let data = res.get("data");
            if let Some(data) = data {
                let items = data.get("items").unwrap();
                let items = items.as_array().unwrap();
                let items = items
                    .iter()
                    .map(|value| {
                        return value.get("chat_id").unwrap().as_str().unwrap().to_string();
                    })
                    .collect::<Vec<_>>();
                return items;
            }
        }
        vec![]
    }
}

// 入口函数
#[post("/", format = "json", data = "<context>")]
async fn index(context: Json<Context>) -> Json<Return> {
    println!("参数内容：{:?}", context);
    let ctx = context.into_inner();

    if ctx.r#type == "url_verfication" {
        // url验证
        return Json(Return {
            challenge: ctx.challenge,
        });
    } else if ctx.r#type == "event_callback" {
        let robot = Robot::new().await;
        println!("{:?}", ctx.event);
        // 添加机器人
        if ctx.event.r#type == "add_bot" {
            println!("群<添加>机器人");
            api::send_message(&robot, &ctx.event.open_chat_id, &ctx.event.chat_name).await;
        }

        if ctx.event.r#type == "remove_bot" {
            println!("群<删除>机器人");
            // api::send_message(&robot, &ctx.event.open_chat_id, &ctx.event.chat_name).await;
        }
    } else if ctx.encrypt != "" {
        // 事件订阅验证
        return Json(Return {
            challenge: ctx.challenge,
        });
    } else {
    }

    return Json(Return {
        challenge: ctx.challenge,
    });
}

#[post("/deploy", format = "json", data = "<task>")]
async fn deploy(task: Json<Task>) {
    let task = &task.task;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let flow = Flow {
        user_name: task
            .get("executorName")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
        project_name: task
            .get("pipelineName")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
        date_time: now,
        message: task.get("message").unwrap().as_str().unwrap().to_string(),
        url: task
            .get("pipelineUrl")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
    };

    let card = flow.card();
    let robot = Robot::new().await;
    api::send_card(&robot, &robot.chat_list[0], card).await;
}

#[catch(404)]
async fn not_fount() -> String {
    "404".to_string()
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .register("/", catchers![not_fount])
        .mount("/", routes![index, deploy])
        .launch()
        .await?;
    Ok(())
}
