use std::io::Error;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::types::{
    AnchorPagination, CalculateLonaResponse, ClientLimit, LoanCreted, LoanInput, OuathCode,
    OuathUser, PaginatedAnchors,
};

/// # Examples
/// ```
/// use loan_api::api::client::LoanClient;
/// fn main() {
/// let client = LoanClient::new(
///     String::from("base_url"),
///     String::from("secret_key"),
///     String::from("public_key"),
///     String::from("access"),
///     String::from("logo_url"),
///     String::from("redirect_url"),
/// );
/// // convert to json
/// let json = serde_json::to_string(&client).unwrap();
/// // make sure secret_key are not serialized
/// assert_eq!(
///    json,
///     r#"{"base_url":"base_url","public_key":"public_key","name":"access","logo_url":"logo_url","redirect_url":"redirect_url"}"#
/// );
/// }
/// ```

#[derive(Serialize, Deserialize, Debug)]
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

    pub async fn get_aouth_user(&self, bearer_token: String) -> Result<OuathUser, Error> {
        let url = format!("{}/api/v1/oauth/auth/user", self.base_url);
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&bearer_token).unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let res = client.get(&url).headers(headers).send().await;

        match res {
            Ok(res) => {
                let json = res.json::<OuathUser>().await;
                match json {
                    Ok(json) => Ok(json),
                    Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                }
            }
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub async fn exchange_code_auth(&self, code: String) -> Result<OuathUser, Error> {
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

        let res = client
            .post(&url)
            .headers(headers)
            .basic_auth(&self.public_key, Some(&self.secret_key))
            .json(&body)
            .send()
            .await;

        match res {
            Ok(res) => {
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
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub async fn client_limit(
        &self,
        bearer_token: String,
        client_id: i32,
    ) -> Result<ClientLimit, Error> {
        let url = format!("{}/api/v1/oauth/client-limit/{}", self.base_url, client_id);

        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&bearer_token).unwrap(),
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
        bearer_token: String,
        client_id: i32,
        pagination: Option<AnchorPagination>,
    ) -> Result<PaginatedAnchors, Error> {
        let url = format!(
            "{}/api/v1/anchors/client-anchors/{}",
            self.base_url, client_id
        );
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&bearer_token).unwrap(),
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
            Ok(res) => {
                let json = res.json::<PaginatedAnchors>().await;
                match json {
                    Ok(json) => Ok(json),
                    Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                }
            }
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
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&bearer_token).unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let res = client.post(&url).headers(headers).json(&body).send().await;

        match res {
            Ok(res) => {
                let json = res.json::<Vec<CalculateLonaResponse>>().await;
                match json {
                    Ok(json) => Ok(json),
                    Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                }
            }
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
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&bearer_token).unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let res = client.post(&url).headers(headers).json(&body).send().await;

        match res {
            Ok(res) => {
                let json = res.json::<LoanCreted>().await;
                match json {
                    Ok(json) => Ok(json),
                    Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
                }
            }
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_client() {
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
}
