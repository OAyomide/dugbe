### DUGBE: A simple tool to backup Railway.app hosted Databases

Dugbe is a simple tool to backup Railway.app hosted Databases.

Railway.app is a great infrastructure platform but at the moment, your best bet backing up a database is connecting AWS. 
Dugbe aims to make this process easier by providing a simple CLI tool to backup your data. 

While this has not been battle tested as much as I would like, I currently use it to backup my [Orchdio](https://orchdio.com) databases.
I'd be glad to take a look at any issues you may have while trying to use the basic functionality ——backup some database.


### What do you need?
 - A Railway.app account.
 - A project with a database. For now, its recommended to pass a project that has just 1 database plugin installed. This is because
   Dugbe will backup the first DB it gets back from the API.
 - An API token.  You can check the docs [here](https://docs.railway.app/reference/public-api) to see how to you can get started with API.
 - `pg_dump` installed on your machine. You may find [this](https://www.cyberithub.com/how-to-install-pg_dump-and-pg_restore-on-ubuntu-20-04-lts-focal/) helpful on how to see how to install it on your machine.


### How to use
```bash
# Install dugbe
Please install by cloning this repo. Then update your environment variables with your Railway.app API token. A sample .env file is provided.
# Run dugbe
dugbe backup --output './backup.sql` --service <your railway.app db hosted service> --database <the URL of the db you want to backup>
```

### What's next?
 - [ ] Add support for backing up multiple databases.
 - [ ] Better UI and progress update.
 - [ ] Env variable support from flag.
 - [ ] Publish to crates.io
 - [ ] Add support for other DBs. Currently only postgres is supported.

### Whats broken or "weird"?
You may get an error like:
```bash
![Sample Error](..%2F..%2F..%2F..%2F..%2Fvar%2Ffolders%2Fzt%2Fsyr0j_td445cf8t0xrfn9y2h0000gp%2FT%2FTemporaryItems%2FNSIRD_screencaptureui_681OAx%2FScreenshot%202023-07-22%20at%2022.41.35.png)
```

This seems to be a non-fatal error and [this (tracking) issue](https://github.com/timescale/timescaledb/issues/1581) might be helpful. I suspect this might be helpful because
IIRC, Railway.app uses TimescaleDB for its Postgres offering.


### Dependencies
 - [cursive]()
 - [pg_dump]()
 - [pv]()