use crate::{Request, RequestBody};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DtbpCheck {
    Both,
    Entry,
    Exit,
}
impl Default for DtbpCheck {
    fn default() -> Self {
        Self::Entry
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TradeConfirmEmail {
    All,
    #[serde(rename = "none")]
    Zero,
}
impl Default for TradeConfirmEmail {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct AccountConfigurations {
    pub dtbp_check: DtbpCheck,
    pub trade_confirm_email: TradeConfirmEmail,
    pub suspend_trade: bool,
    pub no_shorting: bool,
}
impl AccountConfigurations {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Debug)]
pub struct GetAccountConfigurations;
impl Request for GetAccountConfigurations {
    type Body = ();
    type Response = AccountConfigurations;

    fn endpoint(&self) -> Cow<str> {
        "account/configurations".into()
    }
}

#[derive(Clone, Debug)]
pub struct PatchAccountConfigurations(AccountConfigurations);
impl Request for PatchAccountConfigurations {
    type Body = AccountConfigurations;
    type Response = AccountConfigurations;
    const METHOD: Method = Method::PATCH;

    fn endpoint(&self) -> Cow<str> {
        "account/configurations".into()
    }

    fn body(&self) -> RequestBody<&AccountConfigurations> {
        RequestBody::Json(&self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_account_configurations() {
        let _m = mock("GET", "/account/configurations")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(
                r#"{
               	  "dtbp_check": "entry",
 		  "no_shorting": false,
 		  "suspend_trade": false,
 		  "trade_confirm_email": "all" 
		}"#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetAccountConfigurations).await.unwrap();
    }

    #[tokio::test]
    async fn test_patch_account_configurations() {
        let _m = mock("PATCH", "/account/configurations")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(r#"{"dtbp_check":"entry","trade_confirm_email":"all","suspend_trade":false,"no_shorting":false}"#)
            .with_body(
                r#"{
               	  "dtbp_check": "entry",
 		  "no_shorting": false,
 		  "suspend_trade": false,
 		  "trade_confirm_email": "all" 
		}"#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client
            .send(PatchAccountConfigurations(AccountConfigurations::new()))
            .await
            .unwrap();
    }
}
