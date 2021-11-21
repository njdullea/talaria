# Talaria - Arbitrage Trading Program

## Completed Steps
- Pulling and formatting data from coinbase, binance, and kraken and saving to files in data folder.
- Executing backtest and calculating fees over last 4 months.

## Next Steps
- Grab code for setting up websocket connection to binance.
- Figure out how to setup websocket connection to kraken (lower fees than coinbase).
- Transfer over XLM from coinbase to binance and kraken.
- Update main to handle both websockets and buy/sell actions and run the program!!
- Setup command line handling for: `-- backtest`, `-- prod`, `-- reload_data` and anything else.
- Build a separate service for tracking exchange data?

## Notes
- Kraken OHLC data doesn't seem to let requests go before a certain prior date.
- Coinbase OR binance seem to only work with a max length of 4 weeks. Not sure which one or why.