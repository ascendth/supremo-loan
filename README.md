# SUPREMO LOAN API IMPLEMENTATION

this is official implementation of the Supremo Loan API. It is a wrapper for the Supremo Loan API. It provides a simple way to interact with the Supremo Loan API using Rust.

## Getting Started

We dont have a public test API yet. You can contact us at [Supremo Loan](https://supremoloans.com) to get access to the API.

## Installation

```toml
[dependencies]
supremo_loan = "0.1.0"
```

## Usage

```rust
use supremo_loan::api::actions::{add_clients_keys, create_clients};

fn main() {
    let clients_json = serde_json::json!([{
        "base_url": "base_url",
        "name" : "bank name",
        "logo_url" : "logo_url",
        "redirect_url" : "redirect_url"
    }]);
    env::set_var("BANK_NAME_SECRET_KEY", "secret_key"); //this should be set in the environment from .env file to avoid leaking the secret key
    env::set_var("BANK_NAME_PUBLIC_KEY", "public_key"); //this should be set in the environment from .env file
    let clients = add_clients_keys(&clients_json).unwrap();
    let clients = create_clients(&clients).unwrap();
    let clients_json = serde_json::to_string(&clients).unwrap();
    assert_eq!(
        clients_json,
        r#"[{"base_url":"base_url","public_key":"public_key","name":"bank name","logo_url":"logo_url","redirect_url":"redirect_url"}]"#
    );
}

```

## To authorize user

for users to link their loaning account with your app they need to obtain code from [LENDER_API_BASE_URL/api/v1/oauth/auth/authorize?response_type=code&client_id={YOUR_CLIENT_ID}](https://LENDER_API_BASE_URL/api/v1/oauth/auth/authorize?response_type=code&client_id={YOUR_CLIENT_ID}) endpoint from their frontend app.

the code is then exchanged for account info as shown below replacing `USER_CODE` with the code obtained from the above endpoint.

```rust
use supremo_loan::api::actions::{add_clients_keys, create_clients};

fn main() {
    // ...
    //...
    let access_bank = clients[1]
    let user = access_bank.exchange_code_auth("USER_CODE");
        match user {
            Ok(user) => {
                println!("user {:?}", user);
                // use the user to do whatever you want such as connect to their account in your app
            }
            Err(e) => {
                println!("error {:?}", e.to_string());
                // handle error maybe ask user to get code again
            }
        }
}
```

## Other helper function

```rust

    // as you see from above example `clients` is vector while here we are using a single client
    let client_json = serde_json::json!({
        "base_url": "base_url",
        "name" : "bank name",
        // add other client fields needed
    });
    let clients = create_client(&client_json).unwrap();


```

## Other client methods

once you have a client correctlt setup you have access to other methods such as `client_limit`, `get_anchors` and `get_auth_token` as shown below that help do things faster

```rust
    // ...
    // create client
    let clients = create_client(&client).unwrap();


    // get bearer token to use to perform other actions
    let ouath = client.get_auth_token();


    // get user account loan limits
    let limits = client.client_limit("token");
    // `token` is access_token from oauth.access_token

    // use limit to make your app even faster by knowing when loan calculation/application will fail before
    // making the call (you could fetch this once a day) info will not be guarenteed to be up to date at time of
    // loan calculation/application as it might have changed from the time you fetched it


    // get client anchors
    let anchors = client.get_anchors(user.id, "token" Some(AnchorPagination{page: Some(10), page_size : Some(10), order :Some("-id")}));


    // calculate loan
    let loan_input = Vec<LoanInput> = vec![LoanInput{
        amount : 1000, // amount to loan
        anchor_id : user.anchor_id, // this is the anchor id you get from `get_anchors` (anchor id must be ancho to the client)
        client_id : user.id, // this is the id you get from `exchange_code_auth`
        loan_term : 30, // number of days
        loan_type : "api_request".to_string(), // needs to be api_request
        metadata : serde_json::json!({}), // this is optional data you need to send
        }]; //you could send multiple loans at once
    let loans_charges = client.calculate_loan("token", loan_input);


    // create loan
    let loan_res = client.apply_for_loan("token", loan_input);

```

if name is for instance `bank name` then the secret key environment variable should be `BANK_NAME_SECRET_KEY` and the public key environment variable should be `BANK_NAME_PUBLIC_KEY`. [BANK_NAME] should be replaced with the name of the bank or loaning institution.

from the above example, the `add_clients_keys` function takes a json array of clients and adds the public and secret keys to each client object. The `create_clients` function takes a json array of clients and creates each client in the Supremo Loan API.

As the example shows you can create multiple clints that will be added to the Supremo Loan API.

the clients you add need to be using the supremo loan api client library to be able to interact with the Supremo Loan API.

check out the [Supremo Loan API Client](https://docs.rs/reqwest/latest/reqwest/struct.StatusCode.html)

**NOTE:** you need to have the secret and public keys for each client you want to add to the Supremo Loan API.

to finds clinets who are suing the software.

## License

[MIT License](https://github.com/ascendth/supremo-loan/blob/main/LICENSE)
