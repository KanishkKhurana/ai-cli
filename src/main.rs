use clap::Parser;
use reqwest;
use reqwest::Error;
use serde_json::json;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    command: Vec<String>,
}


fn create_file(content: &str) {
    let mut file = File::create("./src/myfile.js").unwrap();
    file.write_all(content.as_bytes()).unwrap();    
}

fn fleek_login() {
    let mut list_dir = Command::new("fleek");
    list_dir.arg("login");
    list_dir.status().expect("Process Failed to Execute");
}

async fn get_ai_resp(prompt:String, key:&str) -> std::result::Result<(), Error> {
    

    let context: String =
                "it should be a simple javascript function that follows the following format - `
        export const main =()=>{
        //function logic
        }
        `
        only and only output the function, nothing else at all. no documentation, helper text or anything else. just output the javascript code.
"
        .to_string();
    

    let body = json!({
        "model": "claude-3-5-sonnet-20240620",
        "max_tokens": 1024,
        "messages": [
            {"role": "user", "content": prompt + &context}
        ]
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let res_json: serde_json::Value = res.json().await?;

    // println!("complete response: {:?} \n", res_json.to_string());
    if let Some(content) = res_json.get("content") {
        if let Some(val) = content.get(0) {
            if let Some(text) = val.get("text") {
                if let Some(clean_text) = text.as_str() {
                    // println!("only response: {} \n", clean_text);
                    create_file(clean_text);
                }
            }
        }
    }

    Ok(())
}

fn create_and_deploy(name: &str, path: &str) {
    let mut create_command = Command::new("fleek");
    create_command
        .arg("functions")
        .arg("create")
        .arg("--name")
        .arg(name);
    create_command.status().expect("error");

    let mut deploy_command = Command::new("fleek");
    deploy_command
        .arg("functions")
        .arg("deploy")
        .arg("--name")
        .arg(name)
        .arg("--path")
        .arg(path);
    deploy_command.status().expect("error");
}

fn redeploy(name: &str, path: &str) {
    let mut com = Command::new("fleek");
    com.arg("functions")
        .arg("deploy")
        .arg("--name")
        .arg(name)
        .arg("--path")
        .arg(path);
    com.status().expect("error");
}

#[tokio::main]
async fn main() {
    let _my_res;
    let args = Args::parse();
    if args.command.len() == 1 {
        match args.command[0].as_str() {
            "redeploy" => {
                println!("add name of the function: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                let name = name.trim();

                println!("add path of the function: ");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                let path = path.trim();

                redeploy(name, path);
            }
            "create_function" => {
                println!("add name of the function: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                let name = name.trim();

                println!("add path of the function: ");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                let path = path.trim();

                create_and_deploy(name, path);
            }
            "login" => fleek_login(),
            "create" => {
                println!("add prompt: ");
                let mut prompt = String::new();
                io::stdin().read_line(&mut prompt).unwrap();

                println!("add key: ");
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();
                let key = key.trim();

                _my_res = get_ai_resp(prompt, key).await;
                // println!("{:?}", my_res);
            }
            _ => eprintln!("Please follow the documentation to understand "),
        }
    } else {
        eprintln!("Please enter a flag and its command one by one");
    }
}
