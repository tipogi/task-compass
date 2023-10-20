# Overview

![task compass](./docs/assets/cover.png)

A script to follow the weekly tasks. It uses a notification system and like this, it cannot be forgotten to write down, the __done__ and __fail__ tasks in the notebook or spreadsheet. This is not useful to use it as a TODO list, it is more to control the daily objectives: study (🧠, 📚), sport(🚴🏻‍♀️, 🏊, ⛰️), eat healthy (🥩, 🧀, 🥚),...

## Installation

Before start testing this tool, Rust has to be installed in the system. Download [here](https://www.rust-lang.org/learn/get-started). Next, clone the repository and create the release binary

```bash
# Enter in the folder
cd task-compass
# Create the release executable
cargo build --release
# Once the release is ready, you will find in that folder
cd target/release
```

## Commands

Once we are in the release folder, we can test what it does the library:

```bash
# start adding the daily objective
./task-compass --task --add
# initialise the notification system
./task-compass --init
# check the recap of the weekly tasks
./task-compass --task --resume
```

## Preparing for automatise the notification service: Crontab

To set up a crontab, the best option is to copy the `release` folder in another location but this is optional.

```bash
# Create a folder to save the binary
mkdir -p /home/username/tools
# Copy the release folder, in the newly created folder
cp /path/of/the/release /home/username/tools/
# Go to the folder
cd /home/username/tools/
# and renamed
mv release compass
```

When the Rust release binary will run as a crontab, the crontab service needs to be aware of the current working directory where the binary is executed. Crontabs typically have a default working directory that might not be the same as the directory where your binary resides. This can lead to issues when creating or accessing files or folders relative to the working directory. For this reason, the command that we will execute in the crontab, it will first access to the folder that the binary resides. Some useful commands of `crontab`

```bash
# Edit the crontab file
crontab -e
# Delete all the crontabs from the crontab file
crontab -r
```

To set up the `crontab`, copy the following snippet in the file (`crontab -e`). If you do not know how to set up your crontab execution time, check [here](https://cron.help)
With that, the binary is going to be executed during that timeframe

```bash
# Example of job definition:
# .---------------- minute (0 - 59)
# |  .------------- hour (0 - 23)
# |  |  .---------- day of month (1 - 31)
# |  |  |  .------- month (1 - 12) OR jan,feb,mar,apr ...
# |  |  |  |  .---- day of week (0 - 6) (Sunday=0 or 7) OR sun,mon,tue,wed,thu,fri,sat
# |  |  |  |  |
# *  *  *  *  * user-name  command to be executed

# Run each day at 9:05
05 19 * * * cd /home/username/tools/compass && ./task-compass --init
```

Check the logs if our crontab has been executed in the time that we decide to execute

```bash
# Filter in the logs of the system, just the CRONtab related
tail -f /var/log/syslog | grep CRON
# If we want to see the outputs of the crontab install postfix as local
apt-get install postfix
# Then check the crontab output
tail -f /var/mail/username
```
