#compdef kv

autoload -U is-at-least

_kv() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'--root=[]:VALUE: ' \
'--gpg=[]:VALUE: ' \
'--shell-completion-generator=[]: :(bash elvish fish powershell zsh)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
":: :_kv_commands" \
"*::: :->kv" \
&& ret=0
    case $state in
    (kv)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:kv-command-$line[1]:"
        case $line[1] in
            (set)
args=("${(@f)$(kv list $2)}")
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
':key -- key:($args)' \
'::value -- value:' \
&& ret=0
;;
(get)
args=("${(@f)$(kv list $2)}")
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
':key -- key:($args)' \
&& ret=0
;;
(list)
args=("${(@f)$(kv list $2)}")
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
'::dir -- directory:($args)' \
&& ret=0
;;
(delete)
args=("${(@f)$(kv list $2)}")
_arguments "${_arguments_options[@]}" \
'-r[]' \
'--recursive[]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
':key -- key:($args)' \
&& ret=0
;;
(push)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
&& ret=0
;;
(pull)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
&& ret=0
;;
(generate-shell-completion)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
':shell -- shell to generate completion for:(bash elvish fish powershell zsh)' \
'::output -- output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_kv__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:kv-help-command-$line[1]:"
        case $line[1] in
            (set)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(get)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(delete)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(push)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(pull)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(generate-shell-completion)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_kv_commands] )) ||
_kv_commands() {
    local commands; commands=(
'set:set a key value pair' \
'get:get a value' \
'list:list keys' \
'delete:delete a key' \
'push:Push changes to remote origin' \
'pull:Pull changes from remote origin' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'kv commands' commands "$@"
}
(( $+functions[_kv__delete_commands] )) ||
_kv__delete_commands() {
    local commands; commands=()
    _describe -t commands 'kv delete commands' commands "$@"
}
(( $+functions[_kv__help__delete_commands] )) ||
_kv__help__delete_commands() {
    local commands; commands=()
    _describe -t commands 'kv help delete commands' commands "$@"
}
(( $+functions[_kv__generate-shell-completion_commands] )) ||
_kv__generate-shell-completion_commands() {
    local commands; commands=()
    _describe -t commands 'kv generate-shell-completion commands' commands "$@"
}
(( $+functions[_kv__help__generate-shell-completion_commands] )) ||
_kv__help__generate-shell-completion_commands() {
    local commands; commands=()
    _describe -t commands 'kv help generate-shell-completion commands' commands "$@"
}
(( $+functions[_kv__get_commands] )) ||
_kv__get_commands() {
    local commands; commands=()
    _describe -t commands 'kv get commands' commands "$@"
}
(( $+functions[_kv__help__get_commands] )) ||
_kv__help__get_commands() {
    local commands; commands=()
    _describe -t commands 'kv help get commands' commands "$@"
}
(( $+functions[_kv__help_commands] )) ||
_kv__help_commands() {
    local commands; commands=(
'set:set a key value pair' \
'get:get a value' \
'list:list keys' \
'delete:delete a key' \
'push:Push changes to remote origin' \
'pull:Pull changes from remote origin' \
'generate-shell-completion:generate shell completion script' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'kv help commands' commands "$@"
}
(( $+functions[_kv__help__help_commands] )) ||
_kv__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'kv help help commands' commands "$@"
}
(( $+functions[_kv__help__list_commands] )) ||
_kv__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'kv help list commands' commands "$@"
}
(( $+functions[_kv__list_commands] )) ||
_kv__list_commands() {
    local commands; commands=()
    _describe -t commands 'kv list commands' commands "$@"
}
(( $+functions[_kv__help__pull_commands] )) ||
_kv__help__pull_commands() {
    local commands; commands=()
    _describe -t commands 'kv help pull commands' commands "$@"
}
(( $+functions[_kv__pull_commands] )) ||
_kv__pull_commands() {
    local commands; commands=()
    _describe -t commands 'kv pull commands' commands "$@"
}
(( $+functions[_kv__help__push_commands] )) ||
_kv__help__push_commands() {
    local commands; commands=()
    _describe -t commands 'kv help push commands' commands "$@"
}
(( $+functions[_kv__push_commands] )) ||
_kv__push_commands() {
    local commands; commands=()
    _describe -t commands 'kv push commands' commands "$@"
}
(( $+functions[_kv__help__set_commands] )) ||
_kv__help__set_commands() {
    local commands; commands=()
    _describe -t commands 'kv help set commands' commands "$@"
}
(( $+functions[_kv__set_commands] )) ||
_kv__set_commands() {
    local commands; commands=()
    _describe -t commands 'kv set commands' commands "$@"
}

_kv "$@"
