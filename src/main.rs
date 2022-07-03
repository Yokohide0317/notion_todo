use serde_json::Value;
use std::env;
use std::process;

#[tokio::main]
async fn send_todo(access_token: String, database_id: String, msg: &str) -> String{

    let mut strpass = String::new();
    strpass.push_str(&"Bearer ");
    strpass.push_str(&access_token);

    let mut body_json = String::new();

    body_json.push_str("{\"parent\": { \"database_id\": \"" );
    body_json.push_str(&database_id );
    body_json.push_str("\" }," );
    body_json.push_str("\"properties\": {" );
    body_json.push_str("\"title\": {" );
    body_json.push_str("\"title\": [{" );
    body_json.push_str("\"text\": {" );
    body_json.push_str("\"content\": \"" );
    body_json.push_str(&msg);
    body_json.push_str("\"}}]},\"Status\": {\"multi_select\": [{" );
    body_json.push_str("\"name\": \"Not started\"" );
    body_json.push_str("}]}}}" );
    // println!("{}", &body_json);


    let client = reqwest::Client::new();
    let res = client
        .post("https://api.notion.com/v1/pages")
        .header("Authorization", &strpass)
        .header("Content-Type", "application/json")
        .header("Notion-Version", "2022-02-22")
        .body(body_json)
        .send()
        .await.unwrap();

    return res.text().await.unwrap();
}


#[tokio::main]
async fn get_todo(access_token: String, database_id: String) -> String{

    let mut strpass = String::new();
    strpass.push_str(&"Bearer ");
    strpass.push_str(&access_token);

    let client = reqwest::Client::new();
    let mut url: String = "https://api.notion.com/v1/databases/".to_string();
    url.push_str(&database_id);
    let res = client
        .get(url)
        .header("Authorization", &strpass)
        .header("Notion-Version", "2022-02-22")
        .send()
        .await.unwrap();

    return res.text().await.unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let access_token: String = match env::var("NOTION_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            println!("Error with NOTION_ACCESS_TOKEN");
            process::exit(1);
        },
    };
    let database_id: String = match env::var("NOTION_DATABASE_ID") {
        Ok(val) => val,
        Err(_) => {
            println!("Error with NOTION_DATABASE_ID");
            process::exit(1);
        },
    };

    let args: Vec<String> = std::env::args().collect();
    let subcommand = &args[1];
    if subcommand == "list" {
        println!("Go to \n> Your Dashboard");
        // get_todo(access_token, database_id);
    } else {
        let messages: &[String] = &args[1..];
        let mut mes: String = String::new();
        if messages.len() >= 1 {
            mes.push_str(&messages.join(" "));
        } else {
            mes.push_str(&"From Rust");
        }

        let _res = send_todo(access_token, database_id, &mes);
        let res_json: Value = serde_json::from_str(&_res).unwrap();
        if res_json["object"].to_string() == "\"error\""{
            println!("送信に失敗しました。");
            println!("{:#?}", res_json.to_string());
        } else {
            println!("ToDoを追加しました\n>  {}", mes);
        }

    }
    return Ok(());
}
