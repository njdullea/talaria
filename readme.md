# Talaria - Arbitrage Trading Program

## Completed Steps
- Pulling and formatting data from coinbase, binance, and kraken and saving to files in data folder.
- Executing backtest for confirmation of historical price discrepancies.
- Connecting to exchanges (FTX and Kucoin) and monitoring current prices for symbol.
- Placing orders to each exchange when large price discrepancy occurs.

## Testing and Results
I setup about $150 on FTX and Kucoin, and would sell on one and buy on the other when a large price discrepancy. After testing for several days I found the slippage from market orders would cover any profits, and limit orders were too likely to not be accepted. It was great at wash trading though!
 
## Notes
- Kraken OHLC data doesn't seem to let requests go before a certain prior date.
- Coinbase OR binance seem to only work with a max length of 4 weeks.
