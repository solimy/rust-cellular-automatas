# rust-cellular-automatas

## Help

```bash
$ cargo run -- --help
```

## Use as a terminal "screen saver"

### 1. build
```bash
$ cargo build --release
```

### 2. create alias

add this to your [.bashrc | .zshrc | .whatever-you-use]

```bash
alias ss='/[where_you_cloned_the_repository]/rust-cellular-automatas/target/release/rust-cellular-automatas  --text --width=$((($(tput cols)-2)*2)) --height=$((($(tput lines)-2)*4)) --rules=snow --tbt=$(($RANDOM%50)) --reset=$(($(tput cols)*$(tput lines)*2))'
```

### 3. enjoy
```bash
$ source ~/[.bashrc | .zshrc | .whatever-you-use]
$ ss
```
