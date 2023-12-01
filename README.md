## Environment Variables

|  Variable   |               Type                | Default Value |
| :---------: | :-------------------------------: | :-----------: |
| GOOBY_TOKEN |              String               |     None      |
| ENVIRONMENT | **Development** \| **Production** |  Production   |

## Getting Started

- To get started, first you need to create a bot application via [Discord developer portal](https://discord.com/developers/applications) and copy your bot's **token**. For more detailed guide, you can check [official documentation](https://discordjs.guide/preparations/setting-up-a-bot-application.html#creating-your-bot).

- ```bash
  git clone https://github.com/emrhnpla02/gooby && cd gooby/
  touch .env && echo "GOOBY_TOKEN=APP_TOKEN_HERE" >> .env
  cargo run
  ```
