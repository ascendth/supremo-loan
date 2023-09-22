use std::{env, io::Error};

use super::client::LoanClient;

/// # Examples
/// ```
/// use supremo_loan::api::actions::{create_clients, add_clients_keys};
/// use supremo_loan::api::client::LoanClient;
/// use std::{io::Error, env};
/// fn main(){
///     let clients_json = serde_json::json!([{
///         "base_url": "base_url",
///         "name" : "bank_name",
///         "logo_url" : "logo_url",
///         "redirect_url" : "redirect_url"
///     }]);
///     env::set_var("BANK_NAME_SECRET_KEY", "secret_key");
///     env::set_var("BANK_NAME_PUBLIC_KEY", "public_key");
///     let clients = add_clients_keys(&clients_json).unwrap();
///     let clients = create_clients(&clients).unwrap();
///     let clients_json = serde_json::to_string(&clients).unwrap();
///     assert_eq!(
///         clients_json,
///         r#"[{"base_url":"base_url","public_key":"public_key","name":"bank_name","logo_url":"logo_url","redirect_url":"redirect_url"}]"#
///     );
/// }
/// ```
///
/// # Examples
/// ```
/// use supremo_loan::api::actions::create_clients;
/// use supremo_loan::api::client::LoanClient;
/// use std::io::Error;
/// fn main() -> Result<(), Error> {
///     let clients_json =serde_json::json!([{
///             "base_url": "base_url",
///             "secret_key" : "secret_key",
///             "public_key" : "public_key",
///             "name" : "bank_name",
///             "logo_url" : "logo_url",
///             "redirect_url" : "redirect_url"
///         }]);
///         
///     let clients = create_clients(&clients_json)?;
///     let clients_json =  serde_json::to_string(&clients)?;
///     assert_eq!(
///       clients_json,
///       r#"[{"base_url":"base_url","public_key":"public_key","name":"bank_name","logo_url":"logo_url","redirect_url":"redirect_url"}]"#
///     );
///     Ok(())
/// }
/// ```

pub fn create_client(client_json: &serde_json::Value) -> Result<LoanClient, Error> {
    let base_url = client_json["base_url"]
        .as_str()
        .unwrap_or_else(|| panic!("base_url is not string"))
        .to_string();
    let secret_key = client_json["secret_key"]
        .as_str()
        .unwrap_or_else(|| panic!("base_url is not string"))
        .to_string();
    let public_key = client_json["public_key"]
        .as_str()
        .unwrap_or_else(|| panic!("base_url is not string"))
        .to_string();
    let name = client_json["name"]
        .as_str()
        .unwrap_or_else(|| panic!("base_url is not string"))
        .to_string();
    let logo_url = client_json["logo_url"]
        .as_str()
        .unwrap_or_else(|| panic!("base_url is not string"))
        .to_string();

    let redirect_url = client_json["redirect_url"]
        .as_str()
        .unwrap_or_else(|| panic!("base_url is not string"))
        .to_string();

    let client = LoanClient::new(
        base_url,
        secret_key,
        public_key,
        name,
        logo_url,
        redirect_url,
    );
    Ok(client)
}

pub fn create_clients(clients_json: &serde_json::Value) -> Result<Vec<LoanClient>, Error> {
    if clients_json.is_array() {
        let clients = clients_json
            .as_array()
            .unwrap()
            .iter()
            .map(|client| {
                let base_url = client["base_url"]
                    .as_str()
                    .unwrap_or_else(|| panic!("base_url is not string"))
                    .to_string();
                let secret_key = client["secret_key"]
                    .as_str()
                    .unwrap_or_else(|| panic!("secret_key is not string"))
                    .to_string();
                let public_key = client["public_key"]
                    .as_str()
                    .unwrap_or_else(|| panic!("public_key is not string"))
                    .to_string();
                let name = client["name"]
                    .as_str()
                    .unwrap_or_else(|| panic!("name is not string"))
                    .to_string();
                let logo_url = client["logo_url"]
                    .as_str()
                    .unwrap_or_else(|| panic!("logo_url is not string"))
                    .to_string();

                let redirect_url = client["redirect_url"]
                    .as_str()
                    .unwrap_or_else(|| panic!("redirect_url is not string"))
                    .to_string();

                LoanClient::new(
                    base_url,
                    secret_key,
                    public_key,
                    name,
                    logo_url,
                    redirect_url,
                )
            })
            .collect();
        Ok(clients)
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "clients_json is not array try to use ```create_client``` function instead",
        ))
    }
}

pub fn add_clients_keys(clients_json: &serde_json::Value) -> Result<serde_json::Value, Error> {
    // add secret_key and public_key to clients
    if clients_json.is_array() {
        let clients = clients_json
            .as_array()
            .unwrap()
            .iter()
            .map(|client| {
                // add secret_key and public_key to client
                let mut client = client.clone(); //TODO::find a way not to clone
                let name = client["name"]
                    .as_str()
                    .unwrap_or_else(|| panic!("name is not string"))
                    .to_string()
                    .replace(" ", "_")
                    .to_uppercase();

                client["secret_key"] = env::var(format!("{}_SECRET_KEY", name))
                    .unwrap_or_else(|_| panic!("env value {}_SECRET_KEY is not set", name))
                    .into();
                client["public_key"] = env::var(format!("{}_PUBLIC_KEY", name))
                    .unwrap_or_else(|_| panic!("env value {}_PUBLIC_KEY is not set", name))
                    .into();
                Ok(client)
            })
            .collect::<Result<serde_json::Value, Error>>()?;
        Ok(clients)
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "clients_json is not array try to use ```create_client``` function instead",
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::api::actions::{add_clients_keys, create_clients};
    #[test]
    fn new_json_clients() {
        let clients_json = serde_json::json!([{
            "base_url": "base_url",
            "secret_key" : "secret_key",
            "public_key" : "public_key",
            "name" : "bank_name",
            "logo_url" : "logo_url",
            "redirect_url" : "redirect_url"
        }]);

        let clients = create_clients(&clients_json).unwrap();
        let clients_json = serde_json::to_string(&clients).unwrap();
        assert_eq!(
            clients_json,
            r#"[{"base_url":"base_url","public_key":"public_key","name":"bank_name","logo_url":"logo_url","redirect_url":"redirect_url"}]"#
        );
    }
    #[test]
    fn add_keys_json_clients() {
        let clients_json = serde_json::json!([{
            "base_url": "base_url",
            "name" : "bank name",
            "logo_url" : "logo_url",
            "redirect_url" : "redirect_url"
        }]);
        env::set_var("BANK_NAME_SECRET_KEY", "secret_key");
        env::set_var("BANK_NAME_PUBLIC_KEY", "public_key");
        let clients = add_clients_keys(&clients_json).unwrap();
        let clients = create_clients(&clients).unwrap();
        let clients_json = serde_json::to_string(&clients).unwrap();
        assert_eq!(
            clients_json,
            r#"[{"base_url":"base_url","public_key":"public_key","name":"bank name","logo_url":"logo_url","redirect_url":"redirect_url"}]"#
        );
    }
}
