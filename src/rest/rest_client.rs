use crate::rest::config::CoinsPaidConfig;
use crate::rest::endpoints::CoinsPaidEndpoint;
use crate::rest::errors::{Error};
use crate::rest::models::{
    AccountBalance, Address, GetAccountBalancesResponse, TakeAddressRequest, TakeAddressResponse,
    WithdrawCryptoRequest, WithdrawCryptoResponse, Withdrawal,
};
use crate::rest::request_signer::RequestSigner;
use error_chain::bail;
use http::header::CONTENT_TYPE;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CoinsPaidRestClient {
    signer: RequestSigner,
    pub_key: String,
    host: String,
    inner_client: reqwest::Client,
}

impl CoinsPaidRestClient {
    pub fn new(private_key: String, pub_key: String) -> Self {
        Self::new_with_config(private_key, pub_key, CoinsPaidConfig::default())
    }

    pub fn new_with_config(private_key: String, pub_key: String, config: CoinsPaidConfig) -> Self {
        Self {
            signer: RequestSigner::new(private_key),
            pub_key,
            host: config.rest_api_host,
            inner_client: reqwest::Client::new(),
        }
    }

    pub async fn take_address(
        &self,
        currency: impl Into<String>,
        foreign_id: impl Into<String>,
    ) -> Result<Address, Error> {
        let request = TakeAddressRequest {
            foreign_id: foreign_id.into(),
            currency: currency.into(),
            convert_to: None,
        };
        let request_json = serde_json::to_string(&request)?;
        let resp: TakeAddressResponse = self
            .post_signed(CoinsPaidEndpoint::TakeAddress, request_json)
            .await?;

        Ok(resp.data)
    }

    pub async fn withdraw_crypto(
        &self,
        address: impl Into<String>,
        currency: impl Into<String>,
        foreign_id: impl Into<String>,
        amount: impl Into<String>,
        tag: Option<String>,
    ) -> Result<Withdrawal, Error> {
        let request = WithdrawCryptoRequest {
            foreign_id: foreign_id.into(),
            amount: amount.into(),
            currency: currency.into(),
            address: address.into(),
            tag,
        };
        let request_json = serde_json::to_string(&request)?;
        let resp: WithdrawCryptoResponse = self
            .post_signed(CoinsPaidEndpoint::CryptoWithdrawal, request_json)
            .await?;

        Ok(resp.data)
    }

    pub async fn get_balances(&self) -> Result<Vec<AccountBalance>, Error> {
        let request_json = "{}".to_string();
        let resp: GetAccountBalancesResponse = self
            .post_signed(CoinsPaidEndpoint::AccountBalances, request_json)
            .await?;

        Ok(resp.data)
    }

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: CoinsPaidEndpoint,
        request_json: String,
    ) -> Result<T, Error> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let sign = self.signer.generate_sign(&request_json);

        let headers = self.build_headers(Some(&sign));
        let client = &self.inner_client;
        let response = client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await?;

        self.handler(response, Some(request_json)).await
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: CoinsPaidEndpoint) -> Result<T, Error> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        println!("{}", url);

        let client = &self.inner_client;
        let headers = self.build_headers(None);
        let response = client.get(url.as_str()).headers(headers).send().await?;

        self.handler(response, None).await
    }

    fn build_headers(&self, sign: Option<&str>) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );

        custom_headers.insert(
            HeaderName::from_static("x-processing-key"),
            HeaderValue::from_str(&self.pub_key).unwrap(),
        );

        if let Some(sign) = sign {
            custom_headers.insert(
                HeaderName::from_static("x-processing-signature"),
                HeaderValue::from_str(sign).unwrap(),
            );
        }

        custom_headers
    }

    pub fn build_query(&self, parameters: HashMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{key}={value}&");
            request.push_str(param.as_ref());
        }
        request.pop();
        request
    }

    async fn handler<T: DeserializeOwned>(
        &self,
        response: Response,
        request_json: Option<String>,
    ) -> Result<T, Error> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?),
            StatusCode::CREATED => {
                let json: Result<String, _> = response.text().await;
                let Ok(json) = json else {
                    bail!("Failed to read response body");
                };
                let body: Result<T, _> = serde_json::from_str(&json);
                if let Err(err) = body {
                    bail!("Failed to deserialize body {:?}: {}", err, json);
                }

                Ok(body.unwrap())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let error = response.text().await?;
                bail!(format!(
                    "Received bad request status. Request: {:?}. Response: {:?}",
                    request_json, error
                ));
            }
            s => {
                let error = response.text().await?;

                bail!(format!("Received response code: {s:?} error: {error:?}"));
            }
        }
    }
}
