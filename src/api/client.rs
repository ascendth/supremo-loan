use std::io::Error;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::types::{
    AnchorPagination, CalculateLonaResponse, ClientLimit, LoanCreted, LoanInput, OuathCode,
    OuathToken, OuathUser, PaginatedAnchors,
};

/// # Examples
/// ```
///     use supremo_loan::api::client::LoanClient;
///     #[tokio::main]
///     async fn main() {
///     let client = LoanClient::new(
///         String::from("base_url"),
///         String::from("secret_key"),
///         String::from("public_key"),
///         String::from("access"),
///         String::from("logo_url"),
///         String::from("redirect_url"),
///     );
///     // convert to json
///     let json = serde_json::to_string(&client).unwrap();
///     // make sure secret_key are not serialized
///     assert_eq!(
///        json,
///         r#"{"base_url":"base_url","public_key":"public_key","name":"access","logo_url":"logo_url","redirect_url":"redirect_url"}"#
///     );
///     }
/// ```

/// // Examples
/// ```
///     use supremo_loan::api::client::LoanClient;
///     #[tokio::main]
///     async fn main() {
///         let client = LoanClient::new(
///             String::from("base_url"),
///             String::from("secret_key"),
///             String::from("public_key"),
///             String::from("bank_name"),
///             String::from("logo_url"),
///             String::from("redirect_url"),
///         );
///         // convert to json
///         let json = serde_json::to_string(&client).unwrap();
///         // make sure secret_key are not serialized
///         assert_eq!(
///             json,
///             r#"{"base_url":"base_url","public_key":"public_key","name":"bank_name","logo_url":"logo_url","redirect_url":"redirect_url"}"#
///         );
///     }
///```

///    // Examples
/// ```
///     use supremo_loan::api::client::LoanClient;
///    #[tokio::main]
///     async fn main() {
///         let client = LoanClient::new(
///             String::from("http://localhost:8080"),
///             String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
///             String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
///             String::from("access"),
///             String::from("logo_url"),
///             String::from("http://127.0.0.1:8020/"),
///         );
///
///         let user = client.exchange_code_auth("w59HPRYvCiE49eeGjazFIw==").await;
///         match user {
///             Ok(user) => {
///                 println!("user {:?}", user);
///             }
///             Err(e) => {
///                 println!("error {:?}", e.to_string());
///             }
///         }
///     }
/// ```

///  // Examples
/// ```
///     use supremo_loan::api::client::LoanClient;
///     #[tokio::main]
///     async fn main() {
///         let client = LoanClient::new(
///             String::from("http://localhost:8080"),
///             String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
///             String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
///             String::from("access"),
///             String::from("logo_url"),
///             String::from("http://127.0.0.1:8020/"),
///         );
///
///         let user = client.get_anchors("uc3zfHDuYEmzhn9FM7lwNQ==", 1, None).await;
///         match user {
///             Ok(user) => {
///                 println!("user {:?}", user);
///             }
///             Err(e) => {
///                 println!("error {:?}", e.to_string());
///             }
///         }
///     }
/// ```

/// // Examples
///
/// ```
///     use supremo_loan::api::client::LoanClient;
///     #[tokio::main]
///     async fn main() {
///         let client = LoanClient::new(
///             String::from("http://localhost:8080"),
///             String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
///             String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
///             String::from("access"),
///             String::from("logo_url"),
///             String::from("http://127.0.0.1:8020/"),
///         );
///
///         let user = client.client_limit("uc3zfHDuYEmzhn9FM7lwNQ==", 1).await;
///         match user {
///             Ok(user) => {
///                 println!("user {:?}", user);
///             }
///             Err(e) => {
///                 println!("error {:?}", e.to_string());
///             }
///         }
///     }
/// ```
/// // Examples
///
/// ```
///     use supremo_loan::api::client::LoanClient;
///     #[tokio::main]
///     async fn main() {
///         let client = LoanClient::new(
///             String::from("http://localhost:8080"),
///             String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
///             String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
///             String::from("access"),
///             String::from("logo_url"),
///             String::from("http://127.0.0.1:8020/"),
///         );
///         let ouath = client.get_auth_token().await;
///         match ouath {
///             Ok(ouath) => {
///                 println!("ouath {:?}", ouath);
///             }
///             Err(e) => {
///                 println!("error", e.to_string());
///             }
///         }
///     }
/// }
/// ```

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanClient {
    pub base_url: String,
    #[serde(skip_serializing)]
    secret_key: String,
    pub public_key: String,
    pub name: String,
    pub logo_url: String,
    pub redirect_url: String,
}

impl LoanClient {
    pub fn new(
        base_url: String,
        secret_key: String,
        public_key: String,
        name: String,
        logo_url: String,
        redirect_url: String,
    ) -> Self {
        Self {
            base_url,
            secret_key,
            public_key,
            name,
            logo_url,
            redirect_url,
        }
    }

    pub async fn get_auth_token(&self) -> Result<OuathToken, Error> {
        let url = format!("{}/api/v1/oauth/auth/token/", self.base_url);

        let client = reqwest::Client::new();

        let body = json!({
            "grant_type": "client_credentials",
            "redirect_uri" : self.redirect_url,
            "cliend_id" : self.public_key,
        });

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let res = client
            .post(&url)
            .basic_auth(&self.public_key, Some(&self.secret_key))
            .headers(headers)
            .form(&body)
            .send()
            .await;

        match res {
            Ok(res) => {
                // march status code
                match res.status() {
                    reqwest::StatusCode::OK => {
                        let json = res.json::<OuathToken>().await;
                        match json {
                            Ok(json) => Ok(json),
                            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                        }
                    }
                    _ => {
                        let json = res.json::<serde_json::Value>().await;
                        match json {
                            Ok(json) => Err(Error::new(
                                std::io::ErrorKind::Other,
                                serde_json::to_string(&json).unwrap(),
                            )),
                            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                        }
                    }
                }
            }
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    async fn get_aouth_user(&self, bearer_token: String) -> Result<OuathUser, Error> {
        let url = format!("{}/api/v1/oauth/auth/user", self.base_url);
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("Bearer {}", &bearer_token).parse().unwrap(),
        );

        let res = client.get(&url).headers(headers).send().await;

        match res {
            Ok(res) => match res.status() {
                reqwest::StatusCode::OK => {
                    let json = res.json::<OuathUser>().await;
                    match json {
                        Ok(json) => Ok(json),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
                _ => {
                    let json = res.json::<serde_json::Value>().await;
                    match json {
                        Ok(json) => Err(Error::new(
                            std::io::ErrorKind::Other,
                            serde_json::to_string(&json).unwrap(),
                        )),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
            },
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub async fn exchange_code_auth(&self, code: &str) -> Result<OuathUser, Error> {
        let url = format!("{}/api/v1/oauth/auth/token/", self.base_url);
        let client = reqwest::Client::new();

        let body = json!({
            "code": code,
            "grant_type": "authorization_code",
            "redirect_uri": self.redirect_url,
        });
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        // use x-www-form-urlencoded
        let res = client
            .post(&url)
            .basic_auth(&self.public_key, Some(&self.secret_key))
            .headers(headers)
            .form(&body)
            .send()
            .await;

        match res {
            Ok(res) => {
                // march status code
                match res.status() {
                    reqwest::StatusCode::OK => {
                        let json = res.json::<OuathCode>().await;
                        match json {
                            Ok(json) => {
                                let user = self.get_aouth_user(json.access_token).await;
                                match user {
                                    Ok(user) => Ok(user),
                                    Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                                }
                            }
                            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                        }
                    }
                    _ => {
                        let json = res.json::<serde_json::Value>().await;
                        match json {
                            Ok(json) => Err(Error::new(
                                std::io::ErrorKind::Other,
                                serde_json::to_string(&json).unwrap(),
                            )),
                            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                        }
                    }
                }
            }
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub async fn client_limit(
        &self,
        bearer_token: &str,
        client_id: i32,
    ) -> Result<ClientLimit, Error> {
        let url = format!("{}/api/v1/oauth/client-limit/{}", self.base_url, client_id);

        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("Bearer {}", &bearer_token).parse().unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let res = client.get(&url).headers(headers).send().await;

        match res {
            Ok(res) => {
                let json = res.json::<ClientLimit>().await;
                match json {
                    Ok(json) => Ok(json),
                    Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                }
            }
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub async fn get_anchors(
        &self,
        bearer_token: &str,
        client_id: i32,
        pagination: Option<AnchorPagination>,
    ) -> Result<PaginatedAnchors, Error> {
        let url = format!(
            "{}/api/v1/oauth/client-anchors/{}",
            self.base_url, client_id
        );
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", &bearer_token).parse().unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        // make pagination query string in reqwest
        let mut url = reqwest::Url::parse(&url).unwrap();
        if let Some(pagination) = pagination {
            if let Some(page) = pagination.page {
                url.query_pairs_mut().append_pair("page", &page.to_string());
            }
            if let Some(page_size) = pagination.page_size {
                url.query_pairs_mut()
                    .append_pair("page_size", &page_size.to_string());
            }
            if let Some(order) = pagination.order {
                url.query_pairs_mut().append_pair("order", &order);
            }
        }

        let res = client.get(url).headers(headers).send().await;

        match res {
            Ok(res) => match res.status() {
                reqwest::StatusCode::OK => {
                    let json = res.json::<PaginatedAnchors>().await;
                    match json {
                        Ok(json) => Ok(json),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
                _ => {
                    let json = res.json::<serde_json::Value>().await;
                    match json {
                        Ok(json) => Err(Error::new(
                            std::io::ErrorKind::Other,
                            serde_json::to_string(&json).unwrap(),
                        )),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
            },
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub async fn calculate_loan(
        &self,
        bearer_token: String,
        body: Vec<LoanInput>,
    ) -> Result<Vec<CalculateLonaResponse>, Error> {
        let url = format!("{}/api/v1/oauth/calc-loan", self.base_url);

        // make sure body.len() > 0
        if body.len() == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "provide at least one input value",
            ));
        }
        let client_id = body[0].client_id;
        for i in 1..body.len() {
            if body[i].client_id != client_id {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "all client_id must be the same",
                ));
            }
        }

        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", &bearer_token).parse().unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let res = client.post(&url).headers(headers).json(&body).send().await;

        match res {
            Ok(res) => match res.status() {
                reqwest::StatusCode::OK => {
                    let json = res.json::<Vec<CalculateLonaResponse>>().await;
                    match json {
                        Ok(json) => Ok(json),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
                _ => {
                    let json = res.json::<serde_json::Value>().await;
                    match json {
                        Ok(json) => Err(Error::new(
                            std::io::ErrorKind::Other,
                            serde_json::to_string(&json).unwrap(),
                        )),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
            },
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub async fn apply_for_loan(
        &self,
        bearer_token: String,
        body: Vec<LoanInput>,
    ) -> Result<LoanCreted, Error> {
        let url = format!("{}/api/v1/oauth/apply-loan", self.base_url);

        // make sure body.len() > 0
        if body.len() == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "provide at least one input value",
            ));
        }

        let client_id = body[0].client_id;
        for i in 1..body.len() {
            if body[i].client_id != client_id {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "all client_id must be the same",
                ));
            }
        }

        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", &bearer_token).parse().unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let res = client.post(&url).headers(headers).json(&body).send().await;

        match res {
            Ok(res) => match res.status() {
                reqwest::StatusCode::OK => {
                    let json = res.json::<LoanCreted>().await;
                    match json {
                        Ok(json) => Ok(json),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
                _ => {
                    let json = res.json::<serde_json::Value>().await;
                    match json {
                        Ok(json) => Err(Error::new(
                            std::io::ErrorKind::Other,
                            serde_json::to_string(&json).unwrap(),
                        )),
                        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
            },
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_client() {
        let client = LoanClient::new(
            String::from("base_url"),
            String::from("secret_key"),
            String::from("public_key"),
            String::from("bank_name"),
            String::from("logo_url"),
            String::from("redirect_url"),
        );
        // convert to json
        let json = serde_json::to_string(&client).unwrap();
        // make sure secret_key are not serialized
        assert_eq!(
            json,
            r#"{"base_url":"base_url","public_key":"public_key","name":"bank_name","logo_url":"logo_url","redirect_url":"redirect_url"}"#
        );
    }

    // test auth process
    #[tokio::test]
    async fn test_user_auth() {
        let client = LoanClient::new(
            String::from("http://localhost:8080"),
            String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
            String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
            String::from("access"),
            String::from("logo_url"),
            String::from("http://127.0.0.1:8020/"),
        );

        let user = client.exchange_code_auth("w59HPRYvCiE49eeGjazFIw==").await;
        match user {
            Ok(user) => {
                println!("user {:?}", user);
            }
            Err(e) => {
                println!("error {:?}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn test_get_anchors() {
        let client = LoanClient::new(
            String::from("http://localhost:8080"),
            String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
            String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
            String::from("access"),
            String::from("logo_url"),
            String::from("http://127.0.0.1:8020/"),
        );

        let user = client
            .get_anchors("uc3zfHDuYEmzhn9FM7lwNQ==", 1, None)
            .await;
        match user {
            Ok(user) => {
                println!("user {:?}", user);
            }
            Err(e) => {
                println!("error {:?}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn test_client_limits() {
        let client = LoanClient::new(
            String::from("http://localhost:8080"),
            String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
            String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
            String::from("access"),
            String::from("logo_url"),
            String::from("http://127.0.0.1:8020/"),
        );

        let user = client.client_limit("uc3zfHDuYEmzhn9FM7lwNQ==", 1).await;
        match user {
            Ok(user) => {
                println!("user {:?}", user);
            }
            Err(e) => {
                println!("error {:?}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn test_auth_token() {
        let client = LoanClient::new(
            String::from("http://localhost:8080"),
            String::from("QX5MgtTRCY48Nk7oMsXlDawofy2qmP8ngyjf8RMfVS62oaHFAq"),
            String::from("eplvesJPuZSS9oOkNQM1pLmZBvazv"),
            String::from("access"),
            String::from("logo_url"),
            String::from("http://127.0.0.1:8020/"),
        );

        let ouath = client.get_auth_token().await;
        match ouath {
            Ok(ouath) => {
                println!("ouath {:?}", ouath);
            }
            Err(e) => {
                println!("error {:?}", e.to_string());
            }
        }
    }
}
