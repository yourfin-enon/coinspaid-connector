use serde::{Deserialize, Serialize};
use std::fmt;
use std::slice::Iter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TakeAddressRequest {
    pub foreign_id: String,
    pub currency: String,
    pub convert_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAccountBalancesResponse {
    pub data: Vec<AccountBalance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountBalance {
    #[serde(rename = "type")]
    pub currency_type: String,
    pub currency: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TakeAddressResponse {
    pub data: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawCryptoRequest {
    pub foreign_id: String,
    pub amount: String,
    pub currency: String,
    pub address: String,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawCryptoResponse {
    pub data: Withdrawal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Withdrawal {
    pub id: i32,
    pub foreign_id: String,
    #[serde(rename = "type")]
    pub withdrawal_type: String,
    pub status: String,
    pub sender_amount: String,
    pub sender_currency: String,
    pub receiver_amount: String,
    pub receiver_currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositAddress {
    pub id: i32,
    pub currency: String,
    pub address: String,
    pub tag: Option<String>,
    pub foreign_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub id: i32,
    pub currency: String,
    pub convert_to: Option<String>,
    pub address: String,
    pub tag: Option<i64>,
    pub foreign_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallbackData {
    pub id: i32,
    pub foreign_id: Option<String>,
    #[serde(rename = "type")]
    pub callback_type: String,
    pub crypto_address: DepositAddress,
    pub error: String,
    pub status: String,
    pub currency_sent: CurrencySentData,
    pub currency_received: CurrencyReceivedData,
    pub transactions: Vec<Transaction>,
    pub fees: Vec<Fee>,
}

impl CallbackData {
    pub fn get_foreign_id(&self) -> Option<String> {
        if let Some(foreign_id) = self.foreign_id.clone() {
            return Some(foreign_id);
        }

        if let Some(foreign_id) = self.crypto_address.foreign_id.clone() {
            return Some(foreign_id);
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencySentData {
    pub currency: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyReceivedData {
    pub currency: String,
    pub amount: String,
    pub amount_minus_fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "id")]
    pub id: i32,
    pub currency: String,
    pub transaction_type: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub tag: Option<String>,
    pub amount: String,
    #[serde(rename = "txid")]
    pub tx_id: Option<String>,
    #[serde(rename = "riskscore")]
    pub risk_score: Option<String>,
    pub confirmations: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fee {
    #[serde(rename = "type")]
    pub fee_type: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Debug, Clone)]
pub enum TransactionType {
    Blockchain = 0,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = format!("{:?}", self).to_lowercase();

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub enum TransactionDataType {
    Deposit = 0,
    Withdrawal = 1,
}

impl TransactionDataType {
    pub fn iterator() -> Iter<'static, Self> {
        static VALUES: [TransactionDataType; 2] = [
            TransactionDataType::Deposit,
            TransactionDataType::Withdrawal,
        ];
        VALUES.iter()
    }
}

impl TryFrom<String> for TransactionDataType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        for item in TransactionDataType::iterator() {
            if item.to_string() == value {
                return Ok(item.to_owned());
            }
        }

        Err("Not supported value".to_string())
    }
}

impl fmt::Display for TransactionDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = format!("{:?}", self).to_lowercase();

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    NotConfirmed = 0,
    Confirmed = 1,
    Canceled = 2,
    Declined = 3,
}

impl TransactionStatus {
    pub fn iterator() -> Iter<'static, TransactionStatus> {
        static VALUES: [TransactionStatus; 3] = [
            TransactionStatus::Confirmed,
            TransactionStatus::NotConfirmed,
            TransactionStatus::Canceled,
        ];
        VALUES.iter()
    }
}

impl TryFrom<String> for TransactionStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        for item in TransactionStatus::iterator() {
            if item.to_string() == value {
                return Ok(item.to_owned());
            }
        }

        Err("Not supported value".to_string())
    }
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            TransactionStatus::Confirmed => format!("{:?}", self).to_lowercase(),
            TransactionStatus::NotConfirmed => "not_confirmed".to_string(),
            TransactionStatus::Canceled => format!("{:?}", self).to_lowercase(),
            TransactionStatus::Declined => format!("{:?}", self).to_lowercase(),

        };

        write!(f, "{}", str)
    }
}
