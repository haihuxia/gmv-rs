use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ReqParam {
    pub url: String,
    // http://ip:port
    pub name: String,
    // hyper
    pub group: String,
    // 2
    pub group_id: i32,
    // fqz6DaAZxT9hikERzwed
    pub token: String,
    // Move from
    pub description: String,
}

pub fn create_project(req: &ReqParam) -> String {
    match create_project_api(req) {
        Ok(t) => {
            t.http_url_to_repo
        },
        Err(e) => {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Reqwest(e) => eprintln!("reqwest error: {}", e),
                FetchError::Json(e) => eprintln!("json error: {}", e),
            }

            String::new()
        }
    }
}

pub fn get_group(req: &ReqParam) -> i32 {
    match get_group_api(req) {
        Ok(t) => {
            let mut group_id = 0;
            if t.len() > 0 {
                group_id = t[0].id;
            }

            group_id
        },
        Err(e) => {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Reqwest(e) => eprintln!("reqwest error: {}", e),
                FetchError::Json(e) => eprintln!("json error: {}", e),
            }

            0
        }
    }
}

fn get_group_api(req: &ReqParam) -> Result<Vec<GroupResult>, FetchError> {
    let client = client(&req.token)?;
    let url = format!("{}/api/v4/groups?search={}", req.url, req.group);

    let mut res = client.get(&url).send()?;
    if res.status().is_success() {
        let a = res.text().unwrap();
        let res_results: Vec<GroupResult> = serde_json::from_str(&a)?;
        println!("get_group_api res_results: {:?}", res_results);

        Ok(res_results)
    } else {
        println!("get_group_api error: {:?}", res);

        Ok(Vec::new())
    }
}

fn create_project_api(req: &ReqParam) -> Result<ProjectResult, FetchError> {
    let client = client(&req.token)?;
    let url = format!("{}/api/v4/projects", req.url);
    let param = format!("name={}&namespace_id={}&description={}", req.name, req.group_id,
                       req.description);
    let mut res = client.post(&url).body(param).send()?;
    if res.status().is_success() {
        let a = res.text().unwrap();
        let res_results: ProjectResult = serde_json::from_str(&a)?;
        println!("create_project_api res_results: {:?}", res_results);

        Ok(res_results)
    } else {
        println!("create_project_api error: {:?}", res);

        Ok(ProjectResult::default())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GroupResult {
    id: i32,
    name: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct ProjectResult {
    id: i32,
    name: String,
    http_url_to_repo: String,
}

enum FetchError {
    Http(hyper::Error),
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::Reqwest(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

fn client(token: &String) -> Result<reqwest::Client, reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN",
                   header::HeaderValue::from_bytes(&token.as_bytes()).unwrap());
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
}