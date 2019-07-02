# NoName

this editor written in **rust**, it's modern and fast. And also simply. Compare to complicated emacs or vim, this editor has those features:

- `ctrl + s` to save;
- `ctrl + q` to quit (save and quit);
- `ctrl + w` to quit without save;
- `ctrl + b` jump curse to begin (like terminal);
- `ctrl + e` jump curse to end;
- `ctrl + d` copy current line to next;
- `ctrl + k` delete current line;

add a line, any changes here?
it finally works now!

with those short cut, you can use this editor in modern mode. beside, the most important thing is copy and paste efficiently, we provide:

- `ctrl + a` select all;
- `ctrl + c` copy select content;
- `ctrl + v` paste copied content;
- `ctrl + z` undo copy or undo paste;
- `shift + arrowkey` select content one the one;

with all those short cuts, you can handle most usage circumstances in daily work and it's much more convenient than vim or emacs!


## Install

You should install rust and cargo, so that it can be built like this:

```
cargo build --release
```


## Copyright

this work is original done by: `gchp`, I just did some modifications and fit to modern editors.