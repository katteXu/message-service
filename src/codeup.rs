use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
pub struct Flow {
    pub user_name: String,
    pub project_name: String,
    pub date_time: String,
    pub message: String,
    pub url: String,
}

impl Flow {
    pub fn card(&self) -> Value {
        let Self {
            user_name,
            project_name,
            date_time,
            message,
            url,
        } = &self;

        let card = json!({
            "elements":[
                {
                    "fields": [
                      {
                        "is_short": false,
                        "text": {
                          "content": format!("**👤  构建人员**：{user_name}"),
                          "tag": "lark_md",
                        },
                      },
                      {
                        "is_short": false,
                        "text": {
                          "content": "",
                          "tag": "lark_md",
                        },
                      },
                      {
                        "is_short": false,
                        "text": {
                          "content": format!("**📄  构建项目**：{project_name}"),
                          "tag": "lark_md",
                        },
                      },
                      {
                        "is_short": false,
                        "text": {
                          "content": "",
                          "tag": "lark_md",
                        },
                      },
                      {
                        "is_short": true,
                        "text": {
                          "content": format!("**📅  构建时间**：{date_time}"),
                          "tag": "lark_md",
                        },
                      },
                    ],
                    "tag": "div",
                  },
                  {
                    "tag": "hr",
                  },
                  {
                    "tag": "note",
                    "elements": [
                      {
                        "tag": "plain_text",
                        "content": format!("{message}"),
                      },
                    ],
                  },
                  {
                    "tag": "action",
                    "actions": [
                      {
                        "tag": "button",
                        "text": {
                          "tag": "plain_text",
                          "content": "查看",
                        },
                        "type": "primary",
                        "url": url,
                      },
                    ],
                  },
            ],
            "header": {
                "template": "green",
                "title": {
                  "content": format!("👍【{project_name}构建完成】"),
                  "tag": "plain_text",
                },
            },
            "config": {
                "wide_screen_mode": true,
                "enable_forward": true,
                "update_multi": true
            },
        });
        card
    }
}
