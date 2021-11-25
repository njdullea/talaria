# Talaria - Arbitrage Trading Program

## Completed Steps
- Pulling and formatting data from coinbase, binance, and kraken and saving to files in data folder.
- Executing backtest and calculating fees over last 4 months.

## Next Step Ideas
- Grab code for setting up websocket connection to binance.
- Figure out how to setup websocket connection to kraken (lower fees than coinbase).
- Transfer over XLM from coinbase to binance and kraken.
- Update main to handle both websockets and buy/sell actions and run the program!!
- Setup command line handling for: `-- backtest`, `-- prod`, `-- reload_data` and anything else.
- Build a separate service for tracking exchange data?

## Next Steps (smallest amount to running)
- Setup BTreeMap for sorting records by datetime in backtest (helps resolve issue with gaps in time from maintenance)
- Setup websocket connection to kraken using tunstenite. See binance crate for example.
- Setup saving data and trade actions to files. 
- Run with recording what actions would have occured and how it would have affected balance.
- Run for real
 
## Notes
- Kraken OHLC data doesn't seem to let requests go before a certain prior date.
- Coinbase OR binance seem to only work with a max length of 4 weeks. Not sure which one or why.