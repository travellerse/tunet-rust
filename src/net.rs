use super::*;
use crypto::digest::Digest;
use crypto::md5::Md5;
use reqwest;
use std::string;

pub struct NetConnect {
    credential: NetCredential,
    client: reqwest::Client,
}

const NET_LOG_URI: &'static str = "http://net.tsinghua.edu.cn/do_login.php";
const NET_FLUX_URI: &'static str = "http://net.tsinghua.edu.cn/rad_user_info.php";

impl NetConnect {
    pub fn new() -> NetConnect {
        NetConnect {
            credential: NetCredential {
                username: string::String::new(),
                password: string::String::new(),
            },
            client: reqwest::Client::new(),
        }
    }

    pub fn from_cred(u: &str, p: &str) -> NetConnect {
        NetConnect {
            credential: NetCredential {
                username: u.to_string(),
                password: p.to_string(),
            },
            client: reqwest::Client::new(),
        }
    }
}

impl NetHelper for NetConnect {
    fn login(&self) -> Result<string::String> {
        let mut cry = Md5::new();
        cry.input_str(&self.credential.password);
        let mut password_md5 = string::String::from("{MD5_HEX}");
        password_md5.push_str(&cry.result_str());
        let params = [
            ("action", "login"),
            ("ac_id", "1"),
            ("username", &self.credential.username),
            ("password", &password_md5),
        ];
        let mut res = self.client.post(NET_LOG_URI).form(&params).send()?;
        Ok(res.text()?)
    }
    fn logout(&self) -> Result<string::String> {
        let params = [("action", "logout")];
        let mut res = self.client.post(NET_LOG_URI).form(&params).send()?;
        Ok(res.text()?)
    }
}

impl NetConnectHelper for NetConnect {
    fn flux(&self) -> Result<NetFlux> {
        let mut res = self.client.get(NET_FLUX_URI).send()?;
        Ok(NetFlux::from(&res.text()?)?)
    }
}
