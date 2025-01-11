_tongfang-keyboard-control() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="tongfang__keyboard__control"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        tongfang__keyboard__control)
            opts="-b -c -p -h --brigthness --color --pattern --completions --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --brigthness)
                    COMPREPLY=($(compgen -W "off low med high max" -- "${cur}"))
                    return 0
                    ;;
                -b)
                    COMPREPLY=($(compgen -W "off low med high max" -- "${cur}"))
                    return 0
                    ;;
                --color)
                    COMPREPLY=($(compgen -W "blue cyan gold green orange pink purple red white yellow" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "blue cyan gold green orange pink purple red white yellow" -- "${cur}"))
                    return 0
                    ;;
                --pattern)
                    COMPREPLY=($(compgen -W "static breathing wave-left wave-right wave-up wave-down reactive reactive-on-input rainbow ripple ripple-on-input marquee raindrop aurora aurora-on-input gamemod spark spark-on-input music" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "static breathing wave-left wave-right wave-up wave-down reactive reactive-on-input rainbow ripple ripple-on-input marquee raindrop aurora aurora-on-input gamemod spark spark-on-input music" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _tongfang-keyboard-control -o nosort -o bashdefault -o default tongfang-keyboard-control
else
    complete -F _tongfang-keyboard-control -o bashdefault -o default tongfang-keyboard-control
fi
