# telegram-remote-hopr-chat

## Description [EN] (below in Russian)

telegram-remote-hopr-chat is your Telegram bot for remote access to [Hopr Chat](https://docs.hoprnet.org/home/getting-started/saentis-testnet/quickstart), which allows you to receive the output of the Hopr Chat process and send commands for input.

<div align="center">
  <kbd>
    <img src=https://raw.githubusercontent.com/savelev1/telegram-remote-hopr-chat/master/media/example.gif />
  </kbd>
</div>

## Installation

### Installation of Dependencies

[Install](https://docs.hoprnet.org/home/getting-started/saentis-testnet/quickstart) Hopr Chat if not installed.

[Install](https://github.com/git-guides/install-git) Git if not installed.

[Install](https://www.rust-lang.org/learn/get-started) Rust if not installed. 

You need to create your Telegram bot and **get its token**: in Telegram, write the bot @BotFather command `/start`, then `/newbot` and follow the instructions.

### Installation of telegram-remote-hopr-chat

`git clone https://github.com/savelev1/telegram-remote-hopr-chat $HOME/telegram-remote-hopr-chat`

telegram-remote-hopr-chat will install in the directory *$HOME/telegram-remote-hopr-chat*. You can change it at your discretion.

Open the directory where the bot is installed and create the config.json file from the config.example.json:

`cd $HOME/telegram-remote-hopr-chat && cp config.example.json config.json`

Open config.json to configure
`nano config.json`

### Description of config.json file parameters
 
`telegram_bot_token` - Telegram bot token

`telegram_user_ids` - array of Telegram user ids, that are allowed access to the bot

`hopr_chat_directory` -  the absolute path to the directory where the Hopr Chat launch file (start-hopr-chat.*) is located

`randobot_xhopr_address` - RandoBot xHOPR address

`coverbot_xhopr_address` - CoverBot xHOPR address

**You need** to enter your Telegram bot token into `telegram_bot_token` and fill the field `hopr_chat_directory` in the config.json file.

## Run the bot

If Hopr Chat is running, end it with `quit` command.

Open the directory where the bot is installed and run it:

`cd $HOME/telegram-remote-hopr-chat && cargo run`

The first launch may take a few minutes.

You can run the bot in a new session (e.g. [Tmux](https://github.com/tmux/tmux/wiki)) so that the bot continues to run when you close the terminal.

Write `/start` to your bot in Telegram.

Bot will write a welcome message and your Telegram id. Insert this id into the config.json file in the `telegram_user_ids` field. 

It should look something like this:

```
{
...
    "telegram_user_ids": [123456789],
...
}
```

You can list different Telegram ids that will have access to the bot by separating them with commas.

Restart the bot to apply the new config.json file.

⚠️ If you leave the `telegram_user_ids` field empty, anyone can use your bot!

Write your bot in Telegram `/start` then `/start_hopr` to start Hopr Chat.

**✅That's all**.

### Update telegram-remote-hopr-chat

Go to the bot directory and pull out the updates:

```cd $HOME/telegram-remote-hopr-chat && git pull```

Restart the bot to apply the updates.

## Описание [RU]

telegram-remote-hopr-chat это ваш Telegram бот для удаленного доступа к [Hopr Chat](https://docs.hoprnet.org/home/getting-started/saentis-testnet/quickstart), который позволяет получать вывод процесса Hopr Chat и отправлять команды на ввод.

<div align="center">
  <kbd>
    <img src=https://raw.githubusercontent.com/savelev1/telegram-remote-hopr-chat/master/media/example.gif />
  </kbd>
</div>

## Установка

### Установка зависимостей

[Установите](https://docs.hoprnet.org/home/getting-started/saentis-testnet/quickstart) Hopr Chat если не установлен.

[Установите](https://github.com/git-guides/install-git) Git если не установлен.

[Установите](https://www.rust-lang.org/learn/get-started) Rust если не установлен. 

Вам необходимо создать свой Telegram бот и **получить его токен**: в Telegram напишите боту @BotFather команду `/start`, затем `/newbot` и следуйте инструкциям.

### Установка telegram-remote-hopr-chat

`git clone https://github.com/savelev1/telegram-remote-hopr-chat $HOME/telegram-remote-hopr-chat`

telegram-remote-hopr-chat установится в директорию *$HOME/telegram-remote-hopr-chat*. Вы можете ее изменить на свое усмотрение.

Откройте директорию в которую установился бот, и создайте файл config.json из config.example.json:

`cd $HOME/telegram-remote-hopr-chat && cp config.example.json config.json`

Откройте config.json для настройки 
`nano config.json`

### Описание параметров файла config.json
 
`telegram_bot_token` - токен вашего Telegram бота

`telegram_user_ids` - массив id Telegram пользователей, которым разрешен доступ к боту

`hopr_chat_directory` - абсолютный путь к директории, в которой размещен файл запуска (start-hopr-chat.*) Hopr Chat

`randobot_xhopr_address` - xHOPR адрес бота RandoBot

`coverbot_xhopr_address` - xHOPR адрес бота CoverBot

**Вам необходимо** вписать токен вашего Telegram бота в `telegram_bot_token` и заполнить поле `hopr_chat_directory` в файле config.json

## Запуск бота

Если Hopr Chat запущен, завершите его командой `quit`

Откройте директорию в которую установился бот и запустите его:

`cd $HOME/telegram-remote-hopr-chat && cargo run`

Первый запуск может занять несколько минут.

Вы можете запустить бота в отдельной сессии (например [Tmux](https://github.com/tmux/tmux/wiki)), чтобы бот продолжал работать когда вы закроете терминал.

Напишите `/start` своему боту в Telegram.

Бот напишет приветственное сообщение и ваш Telegram id. Вставьте этот id в файл config.json в поле `telegram_user_ids`. 

Должно получится примерно так:

```
{
...
    "telegram_user_ids": [123456789],
...
}
```

Вы можете перечислить разные Telegram id у которых будет доступ к боту, разделяя их запятыми.

Перезапустите бота чтобы применился новый файл config.json.

⚠️ Если вы оставите поле `telegram_user_ids` пустым кто угодно сможет пользоваться вашим ботом!

Напишите своему боту в Telegram `/start` затем `/start_hopr` для запуска Hopr Chat.

**✅Вот и все**.

### Обновление telegram-remote-hopr-chat

Перейдите в директорию расположения бота и вытяните обновления:

```cd $HOME/telegram-remote-hopr-chat && git pull```

Перезапустите бота.