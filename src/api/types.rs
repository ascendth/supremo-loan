use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnchorPagination {
    pub page: Option<i32>,      // default 1
    pub page_size: Option<i32>, // default 10 max 100
    pub order: Option<String>,  // "-id" or "id"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anchor {
    anchor_id: i32,
    business_logo: Option<String>,
    business_type: Option<String>,
    company_email: Option<String>,
    company_name: Option<String>,
    created_at: String,
    loaned_amount: f64,
    max_loan_amount: f64,
    tener_id: i32,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedAnchors {
    pub data: Vec<Anchor>,
    pub page: i32,
    pub page_size: i32,
    pub total: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoanInput {
    pub amount: f64,
    pub anchor_id: i32,
    pub client_id: i32,
    pub loan_term: i32,
    pub loan_type: String,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalculateLonaResponse {
    pub excise_duty: f64,
    pub facility_fee: f64,
    pub insurance: f64,
    pub interest_amount: f64,
    pub oauth_apply: LoanInput,
    pub processing_fee: f64,
    pub total: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientLimit {
    pub remaining_limit: f64,
    pub total_limit: f64,
    pub used_limit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanCreted {
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OuathUser{
    pub id : i32,
    pub email : String,
    pub company_name : String,
    pub anchor_id : i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OuathCode{
    pub access_token : String,
    pub refresh_token : String,
    pub token_type : String,
    pub expires_in : i32,
    pub scope : String,
}