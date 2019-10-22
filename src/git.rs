use crate::settings::Settings;
use crate::gitlab;
use crate::gitlab::ReqParam;
use std::process::Command;

#[derive(Debug)]
struct Repo {
    name: String,
    status: bool,
    git: String,
    msg: String,
}

#[derive(Debug)]
struct Result {
    git: String,
    msg: String,
    new_git: String,
}

pub fn clone(s: Settings) {
    let mut results: Vec<Result> = Vec::new();

    let mut req = ReqParam {
        url: s.to.url,
        name: String::new(),
        group: s.to.group,
        group_id: 0,
        token: s.to.personal_token,
        description: String::new(),
    };
    let group_id = gitlab::get_group(&req);
    req.group_id = group_id;
    println!("req: {:?}", req);
    let gits = &s.from.git;
    for g in gits {
        let mut repo = exec_clone(g);
        if repo.status {
            req.name = repo.name;
//            req.name = format!("{}-{}", repo.name, "gmv");
//            println!("req.name: {}", req.name);
            req.description = format!("Move from {}", g);
            repo.git = gitlab::create_project(&req);
            if !repo.git.is_empty() {
                push(&req.name, &repo.git);
                repo.msg = "SUCCESS".to_string();
            } else {
                repo.msg = "FAILURE".to_string();
            }
        }

        // add log
        results.push(Result{
            git: g.to_string(),
            msg: repo.msg,
            new_git: repo.git,
        });
    }
    println!("\n");
    println!("[result]");
    for r in results {
        println!("git: {}, msg: {}, new_git: {}", r.git, r.msg, r.new_git);
    }
}

pub fn push(name: &str, url: &str) {
    let git_url = to_git_ssh_url(url);
    println!("git_url: {}", git_url);
    let child = Command::new("push.bat")
        .arg(name)
        .arg(git_url)
        .status().unwrap();

    if child.success() {
        println!("push success");
    } else {
        println!("push error");
    }
}

// exec command git clone
fn exec_clone(url: &str) -> Repo {
    let mut name = String::new();
    let mut status = false;
    let git = String::new();
    let mut msg = String::new();

    let child = Command::new("git")
        .arg("clone")
        .arg(url)
        .status().unwrap();

    if child.success() {
        let p: Vec<&str> = url.split("/").collect();
        for g in p {
            if g.contains(".git") {
                name = g.replace(".git", "");
                status = true;
                break;
            }
        }
    } else {
        msg = format!("repository '{}' does not exist", url);
    }
    return Repo { name, status, git, msg };
}

fn to_git_ssh_url(http: &str) -> String {
    let u = http.replace("http://", "");
    let p: Vec<&str> = u.split("/").collect();
    let i: Vec<&str> = p[0].split(":").collect();
    format!("git@{}:{}/{}", i[0], p[1], p[2])
}