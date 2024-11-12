# git_afk

`git_afk` is a file watcher that should live as a daemon on the system to watch, commit, and push uncommited changes after a debounce time.

## How it should work

Watch the **selected repositories** for changes in the worktree.
If the changes in the worktree are not changed for **predefined debounce** time, it should **if possible** commit changes with specific commit message and try to push them. If the commit or the push fails. We can just reset the timer.


