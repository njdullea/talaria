use crate::record;
use crate::time_range;
use crate::traits::Exchange;
use coinbase_pro_rs::{Public, Sync, MAIN_URL};
use std::sync::mpsc;
use std::vec;

pub struct CoinbaseExchange;

impl Exchange for CoinbaseExchange {
    fn save_testing_data(
        time_range: time_range::TimeRange,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let coinbase_client: Public<Sync> = Public::new(MAIN_URL);

        let mut coinbase_records: Vec<record::Record> = vec![];

        for sub_range in time_range {
            let coinbase_klines = coinbase_client.get_candles(
                "ATOM-USD",
                Some(sub_range.start.clone()),
                Some(sub_range.end.clone()),
                coinbase_pro_rs::structs::public::Granularity::M1,
            )?;

            coinbase_klines.into_iter().rev().for_each(|f| {
                coinbase_records.push(record::Record {
                    date: f.0 as u64,
                    open: f.3,
                    close: f.4,
                    high: f.2,
                    low: f.1,
                    volume: f.5,
                });
            });
        }

        record::save_records_to_file("data/ATOM-USD-Coinbase.txt", coinbase_records);

        Ok(())
    }

    fn subscribe_to_data(
        _tx: mpsc::Sender<record::Record>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // let keep_running = AtomicBool::new(true); // Used to control the event loop
        // let kline: String = format!("{}", "bnbbtc@kline_1m");

        // let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        //     match event {
        //         WebsocketEvent::Kline(kline_event) => {
        //             let is_final_bar = kline_event.kline.is_final_bar.clone();
        //             if is_final_bar {
        //                 let dp = record::Record {
        //                     date: (kline_event.event_time / 1000_u64) as u64,
        //                     high: kline_event.kline.high.parse::<f64>().unwrap(),
        //                     low: kline_event.kline.low.parse::<f64>().unwrap(),
        //                     open: kline_event.kline.open.parse::<f64>().unwrap(),
        //                     close: kline_event.kline.close.parse::<f64>().unwrap(),
        //                     volume: kline_event.kline.volume.parse::<f64>().unwrap(),
        //                 };
        //                 tx.send(dp).unwrap();
        //             }
        //         }
        //         _ => (),
        //     };
        //     Ok(())
        // });

        // web_socket.connect(&kline).unwrap(); // check error
        // if let Err(e) = web_socket.event_loop(&keep_running) {
        //     match e {
        //         err => {
        //             println!("Error: {:?}", err);
        //         }
        //     }
        // }
        // web_socket.disconnect().unwrap();
        Ok(())
    }
}
