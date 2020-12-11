# Zero Two v4 TODOs

- General works to be done:

  - [ ] Move utils functions to a separate crate
  - [x] Generalize the duplicate pagination functions
  - [ ] Per-guild on/off switches for features
    - [x] Load/Save configs (using PostgreSQL)
    - [ ] Config menu for boolean configs
      - [ ] Use "wait for use answer" for non-boolean configs

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

    - Meta commands:

      - [x] `avatar`
      - [ ] `bot_info`
      - [x] `who`
      - [ ] `ping`

    - Neko's Life commands

      - [ ] `nlimage`

    - System commands:

      - [ ] `cleanup`
      - [ ] `dm`
      - [ ] `echo`
      - [ ] `embed`
      - [ ] `log`
      - [ ] `shutdown`
      - [ ] `reboot`

    - No Category commands:

      - [ ] `giphy`
