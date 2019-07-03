# m editor

**m** editor written in **rust**, it's modern and fast. *m* means minial and modern, vims or emacs are just too old and hard to use. Think about how many time wasted when you try to save a file need permissions of sudo.
So here we re-written a text editor in pure *rust* and designed a pretty new and modern editor which can be cross platform. Our dreaming eidtor is a weapon, so it must be simply and straight forward to use.
Below the final design of our brand new *m editor*:

- `ctrl + s` to save;
- `ctrl + q` to quit (save and quit);
- `ctrl + o` to force quit without save;
- `ctrl + b` jump curse to begin (like terminal);
- `ctrl + e` jump curse to end;
- `ctrl + d` copy current line to next;
- `ctrl + k` delete current line;


with those short cut, you can use this editor in modern mode. beside, the most important thing is copy and paste efficiently, we provide:

- `ctrl + a` select all;
- `ctrl + c` copy select content;
- `ctrl + v` paste copied content;
- `ctrl + z` undo the last operation;
- `shift + arrowkey` select content one the one;

Do u support our brand new design of *m*? 😆 it should save lots of time in this way!! And very much intuitive.


## Install

You should install rust and cargo, so that it can be built like this:

```
cargo build --release
```
if you got some error of `#![feature] may not be used on stable channel`, you can simply do this:

```
rustup override set nightly
```

## Todo

We continues make function fully support of *m*, also make it easy to install from `apt-get` or `brew`. We will have those function does not support yet, if you interested, welcome to PR!

- Terminal window auto wrap (currently does not wrap);
- Indicates password input when try to save a file needs permission;
- Add file create support when creating new file which not exist.


## Copyright

this work is original done by: `gchp`, I just did some modifications and fit to modern editors.