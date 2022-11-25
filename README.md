kv
==

`kv` (pronounced [keɪ ve:]) is an encrypted and versioned command line key-value store.
It's similar to [pass](https://www.passwordstore.org/), [rpass](https://github.com/tiborschneider/rpass), [gopass](https://github.com/gopasspw/gopass) and all the other cousins and siblings of it. 
In fact it is even compatible with them, but has one major difference: `kv` is a key-value store, not a password store.
It doesn't come with all the bells and whistles of a password manager, and only provides the bare minimum to store and retrieve key-value pairs.
Consequently, `kv` is much simpler and easier to use than the other tools.
The only requirements to run it are `git` and `gpg`.
If you have those (and I bet you have if you are looking for a command line key-value storage solution), you can use `kv` right away.

## Features

* Everything encrypted by default
    * Never store your secrets in plain text
    * Uses `gpg` for encryption
* Versioned
    * Never lose your secrets
    * Uses `git` for versioning
* Simple
    * No complicated command line options
    * No configuration files
    * No setup wizard
    * No database
    * No web interface
    * No daemon
    * No cloud
    * No bullshit

## Installation

### From source

```bash
git clone git@github.com:trusch/kv.git
cd kv

make release
sudo make install

# optionally install shell completion for zsh...
cp completions/kv.zsh ~/.oh-my-zsh/completions/_kv
# ...or bash
sudo cp completions/kv.bash /etc/bash_completion.d/kv
```

## Usage

```
Usage: kv [OPTIONS] <COMMAND>

Commands:
  set     set a key value pair
  get     get a value
  list    list keys
  delete  delete a key
  push    Push changes to remote origin
  pull    Pull changes from remote origin
  help    Print this message or the help of the given subcommand(s)

Options:
      --root <VALUE>  [env: KV_ROOT=.] [default: ~/.kv]
      --gpg <VALUE>   [env: KV_GPG_ID=]
  -h, --help          Print help information
  -V, --version       Print version information
```


## Completion

There is support for completion in zsh and bash shells.
Those completions are generated using clap but hand-tuned afterwards to support dynamic completion of keys.
To enable completion for zsh, copy the `completions/kv.zsh` file to your zsh completions directory (for oh-my-zsh users that would be `~/.oh-my-zsh/completions`). Bash users can copy the `completions/kv.bash` file to `/etc/bash_completion.d/`.

## Examples

### Store and retrieve some data

```bash
kv set data "This is the data"
kv get data
# prints "This is the data"
```

### Use pipes
```bash 
echo "This is the data" | kv set data
kv get data | tr '[:lower:]' '[:upper:]'
# prints "THIS IS THE DATA"
```

### Use environment variables
```bash
export KV_ROOT="/mnt/secure"
export KV_GPG_ID="me@super-secure.xyz"
kv set data "This is the data"
# data is stored in /mnt/secure/data.gpg with the specified key
```

### Search for keys
```bash
kv set one/complicated/path/foo "This is the right data"
kv set second/complicated/path/bar "This is the wrong data"
key=$(kv list | grep foo)
kv get $key
# prints "This is the right data"
```

### Setup a remote origin
```bash
cd ~/.kv
git remote add origin git@github.com:my-user/kv-store.git
git push --set-upstream origin main
kv set data "This is the data"
kv push
```

