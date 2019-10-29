# mhicon
Adds / Changes an icon indicating your mental health condition to your mastodon display name, via command line

You will need rusts package manager cargo: https://github.com/rust-lang/cargo

Run via 
> cargo run

On the first run, you will be asked to link the application to a mastodon instance. For a local running mastodon development instance for example give it: http://localhost:3000

When successfully linked, it will parse your username and then ask you how you feel. Type in a number, it will add an a badge to your account corresponding to the answer.
