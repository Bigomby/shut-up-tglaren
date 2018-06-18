# Shut up, tglaren!

![Imgur](https://i.imgur.com/RpcS8k7.png)

Are you tired of messages like this flooding your Telegram group?

Use this bot to automatically delete this kind of message.

## How it works?

It just uses a regular regular expression to delete "member joined" messages
when his name matches.

You need to set two enviroment vars:

| ENV                  | Description                                  |
| -------------------- | -------------------------------------------- |
| `TELEGRAM_BOT_TOKEN` | Your Telegram bot token                      |
| `NAME_REGEX`         | RegEx for matching the name (e.g. "tglaren") |
