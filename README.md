# twitter-bot-rs

A Twitter bot written in Rust that scans crypto-currency values and
post periodically a message on Twitter with the value of currencies.

![example of tweet](misc/tweet.png)

## How to run it

1. Edit ```$HOME/.crypto-bot.conf``` and put values your want in your config file
2. Issue ```cargo run```

## Config File

```json
{
  "consumer_key": "<consumer-key>",
  "consumer_secret": "<consumer-secret>",
  "access_key": "<access-key>",
  "access_secret": "<access-secret>",
  "currencies_to_follow" : ["BTC","LTC","ETH", "DASH", "ZEC", "XRP"],
  "interval_sec" : 300
}
```

You can only include the "currencies_to_follow" to just print things. Without an interval_sec you'll get an update every minute to your screen.
