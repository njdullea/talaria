use crate::parse;
use crate::record;
use crate::{time_range::TimeRange, traits::Exchange};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error;
use std::{fmt::Display, str::FromStr, sync::mpsc};
use tungstenite::{connect, Message};

pub struct KuCoinExchange;

#[derive(Serialize)]
struct KuCoinWSSSubscription {
    id: u64,
    #[serde(rename = "type")]
    type_: String,
    topic: String,
    #[serde(rename = "privateChannel")]
    private_channel: bool,
    response: bool,
}

#[derive(Serialize)]
struct KuCoinPing {
    id: u64,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Deserialize, Debug)]
struct KuCoinKLineData {
    symbol: String,
    candles: KuCoinKLine<u64, f64>,
    time: u64,
}

#[derive(Deserialize, Debug)]
struct KuCoinKLineSubscriptionData {
    #[serde(rename = "type")]
    type_: String,
    topic: String,
    subject: String,
    data: KuCoinKLineData,
}

impl KuCoinExchange {
    pub fn test_ws() -> Result<(), Box<dyn error::Error>> {
        let url = String::from("https://api.kucoin.com/api/v1/bullet-public");
        let client = reqwest::blocking::Client::new();
        let res = client.post(url).send()?;

        let data: KuCoinTokenRequestResponse = res.json()?;
        let mut ws_url = String::from(data.data.instance_servers[0].endpoint.clone());
        ws_url.push_str("?token=");
        ws_url.push_str(data.data.token.as_str());
        ws_url.push_str("&connectId=");
        let connect_id = "gQdf7jkn1we5ydthhh";
        ws_url.push_str(connect_id);

        let (mut socket, _response) =
            connect(reqwest::Url::parse(&ws_url).unwrap()).expect("Can't connect");
        let subscription_request = KuCoinWSSSubscription {
            id: 92458671349721,
            type_: String::from("subscribe"),
            topic: String::from("/market/candles:ATOM-USDT_1min"),
            private_channel: false,
            response: true,
        };

        let mut send_subscribe = false;

        loop {
            let msg = socket.read_message().expect("Error reading message");
            println!("Received: {}", msg);

            let kline: Option<KuCoinKLineSubscriptionData> =
                match serde_json::from_str(&msg.to_string().as_str()) {
                    Ok(kl) => Some(kl),
                    Err(_e) => None,
                };

            println!("Here is the kline: {:?}", kline);

            if !send_subscribe {
                let ping = KuCoinPing {
                    id: 9245910220728,
                    type_: String::from("ping"),
                };

                let pong = KuCoinPing {
                    id: 9245910220729,
                    type_: String::from("pong"),
                };

                let _ping_msg = Message::Ping(serde_json::to_vec(&ping).unwrap());
                let _pong_msg = Message::Pong(serde_json::to_vec(&pong).unwrap());
                let subscribe =
                    Message::Text(serde_json::to_string(&subscription_request).unwrap());

                match socket.write_message(subscribe) {
                    Ok(()) => println!("Message sent!"),
                    Err(e) => println!("Error sending message : {:?}", e),
                }

                send_subscribe = true;
            }
        }
    }
}

impl Exchange for KuCoinExchange {
    // KuCoin has limit of 1500 data points per request.
    fn save_testing_data(time_range: TimeRange) -> Result<(), Box<dyn std::error::Error>> {
        let mut records: Vec<record::Record> = vec![];

        time_range.for_each(|sr: TimeRange| {
            let mut new_records = get_kline_data(sr.start, sr.end).unwrap();
            new_records.reverse();
            records.append(&mut new_records);
        });

        record::save_records_to_file("data/ATOM-USD-KuCoin.txt", records);

        Ok(())
    }

    fn subscribe_to_data(_tx: mpsc::Sender<record::Record>) -> Result<(), Box<dyn error::Error>> {
        let url = String::from("https://api.kucoin.com/api/v1/bullet-public");
        let client = reqwest::blocking::Client::new();
        let res = client.post(url).send()?;

        let data: KuCoinTokenRequestResponse = res.json()?;
        let mut ws_url = String::from(data.data.instance_servers[0].endpoint.clone());
        ws_url.push_str("?token=");
        ws_url.push_str(data.data.token.as_str());
        ws_url.push_str("&connectId=");
        let connect_id = "gQdf7jkn1we5ydthhh";
        ws_url.push_str(connect_id);

        let (mut socket, _response) =
            connect(reqwest::Url::parse(&ws_url).unwrap()).expect("Can't connect");
        let subscription_request = KuCoinWSSSubscription {
            id: 92458671349721,
            type_: String::from("subscribe"),
            topic: String::from("/market/candles:ATOM-USDT_1min"),
            private_channel: false,
            response: true,
        };

        let mut send_subscribe = false;

        loop {
            let msg = socket.read_message().expect("Error reading message");
            println!("Received: {}", msg);

            let kline: Option<KuCoinKLineSubscriptionData> =
                match serde_json::from_str(&msg.to_string().as_str()) {
                    Ok(kl) => Some(kl),
                    Err(_e) => None,
                };

            println!("Here is the kline: {:?}", kline);

            // TODO: make record and transmit to receiver

            if !send_subscribe {
                let subscribe =
                    Message::Text(serde_json::to_string(&subscription_request).unwrap());

                match socket.write_message(subscribe) {
                    Ok(()) => println!("Message sent!"),
                    Err(e) => println!("Error sending message : {:?}", e),
                }

                send_subscribe = true;
            }
        }
    }
}

fn get_kline_data(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<record::Record>, Box<dyn std::error::Error>> {
    let start_seconds = start.timestamp().to_string();
    let end_seconds = end.timestamp().to_string();

    let mut url =
        String::from("https://api.kucoin.com/api/v1/market/candles?symbol=ATOM-USDT&startAt=");
    url.push_str(start_seconds.as_str());
    url.push_str("&endAt=");
    url.push_str(end_seconds.as_str());
    url.push_str("&type=1min");

    let json: KuCoinResponse = reqwest::blocking::get(url)?.json()?;
    let mut records: Vec<record::Record> = vec![];

    for kline in json.data {
        records.push(record::Record {
            date: kline.0 as u64,
            open: kline.1,
            close: kline.2,
            high: kline.3,
            low: kline.4,
            volume: kline.5,
        });
    }

    Ok(records)
}

// THESE STRUCTS ARE FOR DESERIALIZING THE KLINE HISTORICAL DATA
// start time, open, close, high, low, transaction volume, transaction amount
#[derive(Deserialize, Debug)]
struct KuCoinKLine<T, U>(
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "T: FromStr, T::Err: Display"))]
    T,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
);

#[derive(Deserialize, Debug)]
struct KuCoinResponse {
    #[serde(deserialize_with = "parse::u64_from_string")]
    code: u64,
    data: Vec<KuCoinKLine<u64, f64>>,
}

// THESE STRUCTS ARE FOR DESERIALIZING THE TOKEN REQUEST
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KuCoinEndpoint {
    endpoint: String,
    encrypt: bool,
    protocol: String,
    ping_interval: u64,
    ping_timeout: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KuCoinTokenData {
    token: String,
    instance_servers: Vec<KuCoinEndpoint>,
}

#[derive(Deserialize, Debug)]
struct KuCoinTokenRequestResponse {
    #[serde(deserialize_with = "parse::u64_from_string")]
    code: u64,
    data: KuCoinTokenData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_kucoin_klines() {
        let system_time = std::time::SystemTime::now();
        let start = DateTime::<Utc>::from(system_time)
            .checked_sub_signed(chrono::Duration::minutes(5))
            .unwrap();

        let end = start
            .clone()
            .checked_add_signed(chrono::Duration::minutes(4))
            .unwrap();

        let r = get_kline_data(start, end);
        match r {
            Ok(_) => println!("Okay!"),
            Err(e) => println!("Err! {:?}", e),
        }
    }

    #[test]
    fn confirm_kucoin_lines_ordered() {
        let records = record::read_records_from_file("data/ATOM-USD-KuCoin.txt");
        let mut previous_dt = 0;

        for record in records {
            println!(
                "Previous Dt and current DT: {:?} {:?} ",
                previous_dt, record.date
            );
            assert!(record.date.gt(&previous_dt));
            previous_dt = record.date;
        }
    }
}
