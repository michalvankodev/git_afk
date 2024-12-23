# git_afk

`git_afk` is a file watcher that should live as a daemon on the system to watch, commit, and push uncommited changes after a debounce time.

## Motivation

Watch the **selected repositories** for changes in the worktree.
If the changes in the worktree are not changed for **predefined debounce** time, it should **if possible** commit changes with specific commit message and try to push them. If the commit or the push fails. We can just reset the timer.

## Installation

You can either download [latest binaries from github releases](https://github.com/michalvankodev/git_afk/releases) and put them into `~/.local/bin/` or `/usr/local/bin/` or anywhere into your `$PATH`.

Second option is to install with [`cargo`](https://crates.io/):
`cargo install git_afk`

### Running as a daemon

For the best convenience it is recommended to run the `git_afk watch` as a daemon on your system.

#### Linux

Copy this `git_afk.service` file to your services folders e.g. `~/.config/systemd/user/`


```systemd
[Unit]
Description=git_afk
After=ssh-agent.service
Requires=ssh-agent.service

[Service]
ExecStart=git_afk watch
Environment="SSH_AUTH_SOCK=/run/user/1000/keyring/ssh"

[Install]
WantedBy=default.target
```

> `git_afk` needs to be started with an ability to commit to git repository. If you use `ssh` connection to git it requires `SSH_AUTH_SOCK` to be specified in environment prior to running system service.

#### Other systems

Create a launchd configuration file for your app. This involves creating a new file in /Library/LaunchAgents/ with a .plist extension, containing the necessary details like the executable path and startup order.

I am not able to compile/test functionality on other systems. I would recommend using `launchd`.
Feel free to open issues/pull requests for additional functionality on other platforms.


