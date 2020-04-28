# IRC Chat Notifier

This program displays a desktop notification when someone chat on the channel.

## Installation

Currently, you can install via `cargo install irc-chat-notifier`.

## Example

```bash
irc-chat-notifier --nick john.doe --server irc.freenode.org --channel '#linux' --password abcd1234 --ignore-nick john.doe
```

For Twitch channel, you can use the [Twitch Chat OAuth Password Generator](https://twitchapps.com/tmi) to generate the password. The command would look like:

```bash
irc-chat-notifier --nick john.doe --server irc.chat.twitch.tv --channel '#dexbonus' --password abcd1234 --ignore-nick john.doe
```