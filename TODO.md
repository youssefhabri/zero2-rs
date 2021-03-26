# Zero Two v4 TODOs

- General works to be done:

  - [ ] Move utils functions to a separate crate
  - [x] Generalize the duplicate pagination functions
  - [ ] Per-guild on/off switches for features

    - [x] Load/Save configs (using PostgreSQL)
    - [ ] Config menu for boolean configs
      - [ ] Use "wait for user answer" for non-boolean configs

  - [ ] Setup the commands permissions
  - [ ] Fix the markdown parsing
  - [x] Change all `ToString` impls to `Display`
  - [ ] Add descriptions and usage to all commands
  - [ ] Move commands to separate crate
  - [ ] Start implementing Slash commands
  - [ ] Improve error logging (preferably a backtrace?) (Use Sentry.io)

    - [ ] Setup sentry in dokku
    - [ ] Add sentry error reporting EVERYWHERE!!!

  - [ ] Lack of Error logging is becoming a big issue

  - [ ] Save menu entries into disk, to allow for cross-restart menu interactions

  - Moderation:
    - [ ] Log user (mod-only?) actions

- Bot features:

  - [x] Monitors

    - [x] AniList links monitor
    - [x] Discord id monitor

  - [ ] Commands

    - AniList commands:

      - [x] `airing`
      - [x] `anime`
      - [x] `character`
      - [x] `manga`
      - [x] `staff`
      - [ ] `source`
      - [x] `user`
      - [x] `studio`

    - Knowledge commands:

      - [x] `urban`

    - Fun commands:

      - [x] `bigtext`
      - [x] `cookie`
      - [ ] `golender`
      - [x] `fortune`
      - [x] `owo`
      - [x] `next`
      - [x] `giphy`
      - [x] `nlimage`

    - Meta commands:

      - [x] `avatar`
      - [x] `bot_info`
      - [x] `who`
      - [x] `ping`

    - System commands:

      - [ ] `cleanup`
      - [ ] `dm`
      - [ ] `echo`
      - [ ] `embed`
      - [ ] `log`
      - [ ] `shutdown`
      - [ ] `reboot`
