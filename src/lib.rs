use reqwest::blocking::Response;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct UpcloudAccountRoot {
    account: UpcloudAccount,
}

#[derive(Deserialize, Debug)]
pub struct UpcloudAccountResources {
    pub cores: f32,
    //detached_floating_ips: null,
    pub memory: f32,
    pub networks: u32,
    pub public_ipv4: u32,
    pub public_ipv6: u64,
    pub storage_hdd: f32,
    //pub storage_maxiops": null,
    pub storage_ssd: f32,
}

#[derive(Deserialize, Debug)]
pub struct UpcloudAccount {
    pub credits: f32,
    pub resource_limits: UpcloudAccountResources,
    pub username: String
}

pub struct UpcloudApi {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct UpcloudErrorRoot {
    error: UpcloudError,
}

#[derive(Deserialize, Debug)]
struct UpcloudError {
    error_code: String,
    error_message: String,
}

#[derive(Debug)]
struct UpcloudApiError {
    code: String,
    msg: String,
}

impl Display for UpcloudApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

impl Error for UpcloudApiError {}

impl<'a> UpcloudApi {
    pub fn new<S1, S2>(username: S1, password: S2) -> UpcloudApi
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        UpcloudApi {
            username: username.into(),
            password: password.into(),
        }
    }

    fn get(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()?;
        let status = resp.status();
        if status.is_client_error() {
            let result: UpcloudErrorRoot = resp.json()?;
            Err(Box::new(UpcloudApiError {
                code: result.error.error_code,
                msg: result.error.error_message,
            }))
        } else {
            Ok(resp.error_for_status()?)
        }
    }

    fn post(
        &self,
        url: &str,
        map: HashMap<&str, String>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&map)
            .send()?;
        let status = resp.status();
        if status.is_client_error() {
            let result: UpcloudErrorRoot = resp.json()?;
            Err(Box::new(UpcloudApiError {
                code: result.error.error_code,
                msg: result.error.error_message,
            }))
        } else {
            Ok(resp.error_for_status()?)
        }
    }

    fn delete(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .delete(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()?;
        let status = resp.status();
        if status.is_client_error() {
            let result: UpcloudErrorRoot = resp.json()?;
            Err(Box::new(UpcloudApiError {
                code: result.error.error_code,
                msg: result.error.error_message,
            }))
        } else {
            Ok(resp.error_for_status()?)
        }
    }

    pub fn get_account_info(&self) -> Result<UpcloudAccount, Box<dyn std::error::Error>> {
        Ok(self
            .get("https://api.Upcloud.com/1.3/account")?
            .json::<UpcloudAccountRoot>()?
            .account)
    }
}
