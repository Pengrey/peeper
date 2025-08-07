use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Field {
    #[serde(rename = "type")]
    field_type: String,

    #[serde(default)]
    value: Value,
}

#[derive(Deserialize, Debug)]
struct Credential {
    #[serde(default)]
    title: String,

    #[serde(default)]
    fields: Vec<Field>,
}

pub fn extract_credentials_desktop(text: &str, verbose: bool) {
    let mut strings_list: Vec<String> = Vec::new();

    let re = Regex::new(r#"(\{"title":.*?,"custom":\[]\})"#).unwrap();

    for cap in re.captures_iter(text) {
        if let Some(group1) = cap.get(1) {
            let mut matched_str = group1.as_str().to_string();

            if let Some(cut_index) = matched_str.find("}  ") {
                matched_str = matched_str[..cut_index + 1].trim_end().to_string();
            } else {
                matched_str = matched_str.trim_end().to_string();
            }

            if !strings_list.contains(&matched_str) && matched_str.len() > 20 {
                let result: Result<Credential, _> = serde_json::from_str(&matched_str);

                match result {
                    Ok(credential) => {
                        println!("[+] Found Credential:");
                        let mut user = None;
                        let mut password = None;

                        for field in &credential.fields {
                            match field.field_type.as_str() {
                                "login" => {
                                    if let Some(first_val) = field.value.as_array().and_then(|arr| arr.get(0)).and_then(|v| v.as_str()) {
                                        user = Some(first_val.to_string());
                                    }
                                }
                                "password" => {
                                    if let Some(first_val) = field.value.as_array().and_then(|arr| arr.get(0)).and_then(|v| v.as_str()) {
                                        password = Some(first_val.to_string());
                                    }
                                }
                                _ => {}
                            }
                        }

                        println!("\t[>] Title: {}", credential.title);
                        if let Some(u) = user {
                            println!("\t[>] User: {}", u);
                        } else {
                            println!("\t[>] User: not found in fields");
                        }
                        if let Some(p) = password {
                            println!("\t[>] Password: {}", p);
                        } else {
                            println!("\t[>] Password: not found in fields");
                        }
                    }
                    Err(_e) => {
                        if verbose {
                            println!("[+] Found Credential:");
                            println!("[!] Failed to parse JSON: {}", matched_str);
                        }
                    }
                }

                strings_list.push(matched_str);
            }
        }
    }
}

pub fn extract_credentials_chrome(text: &str, verbose: bool) {
    let mut strings_list: Vec<String> = Vec::new();

    let re = Regex::new(r#"(\"title\":.*?(\"custom\"|\"fields\"):\[\])"#).unwrap();

    for cap in re.captures_iter(text) {
        if let Some(group1) = cap.get(1) {
            let mut matched_str = group1.as_str().to_string();

            if let Some(cut_index) = matched_str.find("}  ") {
                matched_str = matched_str[..cut_index + 1].trim_end().to_string();
            } else {
                matched_str = matched_str.trim_end().to_string();
            }

            matched_str = format!("{{{}}}", &matched_str);

            if !strings_list.contains(&matched_str) && matched_str.len() > 20 {
                let result: Result<Credential, _> = serde_json::from_str(&matched_str);

                match result {
                    Ok(credential) => {
                        println!("[+] Found Credential:");
                        let mut user = None;
                        let mut password = None;

                        for field in &credential.fields {
                            match field.field_type.as_str() {
                                "login" => {
                                    if let Some(first_val) = field.value.as_array().and_then(|arr| arr.get(0)).and_then(|v| v.as_str()) {
                                        user = Some(first_val.to_string());
                                    }
                                }
                                "password" => {
                                    if let Some(first_val) = field.value.as_array().and_then(|arr| arr.get(0)).and_then(|v| v.as_str()) {
                                        password = Some(first_val.to_string());
                                    }
                                }
                                _ => {}
                            }
                        }

                        println!("\t[>] Title: {}", credential.title);
                        if let Some(u) = user {
                            println!("\t[>] User: {}", u);
                        } else {
                            println!("\t[>] User: not found in fields");
                        }
                        if let Some(p) = password {
                            println!("\t[>] Password: {}", p);
                        } else {
                            println!("\t[>] Password: not found in fields");
                        }
                    }
                    Err(_e) => {
                        if verbose {
                            println!("[+] Found Credential:");
                            println!("[!] Failed to parse JSON: {}", matched_str);
                        }
                    }
                }

                strings_list.push(matched_str);
            }
        }
    }
}

pub fn extract_cookies(text: &str, _verbose: bool) {
    let mut strings_list: Vec<String> = Vec::new();

    let re = Regex::new(r#"(\{"expiry":.*?,"data":.*?})"#).unwrap();

    for cap in re.captures_iter(text) {
        if let Some(group1) = cap.get(1) {
            let mut matched_str = group1.as_str().to_string();

            if let Some(cut_index) = matched_str.find("}  ") {
                matched_str = matched_str[..cut_index + 1].trim_end().to_string();
            } else {
                matched_str = matched_str.trim_end().to_string();
            }

            if !strings_list.contains(&matched_str) && matched_str.len() > 20 {
                println!("[+] Found Cookie: {}", matched_str);

                strings_list.push(matched_str);
            }
        }
    }
}
