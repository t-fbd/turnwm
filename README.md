# turnwm - my personal implementation of the [Penrose](https://github.com/sminez/penrose) library.

Extremely minimal, dual-screen no workspaces. Intended for heavy use with tmux and my personal setup. This very well may not work the best or at all for you.

        "M-j"               Focus Down
        "M-k"               Focus Up
        "M-S-j"             Swap Down
        "M-S-k"             Swap Up
        "M-w"               Kill Focused
        "M-Tab"             Switch Screen Focus
        "M-S-Tab"           Switch Master Screen
        "M-grave"           Switch Focused Client to Other Screen
        "M-bracketright"    Next Layout
        "M-bracketleft"     Previous Layout 
        "M-u"               Inc Main +1
        "M-d"               Inc Main -1
        "M-l"               Expand Main
        "M-h"               Shrink Main
        "M-r"               Rofi
        "M-Return"          Alacritty
        "M-A-Escape"        Power Menu (modified version of same named function found [here](https://github.com/sminez/my-penrose-config/blob/develop/src/actions.rs))

        Layouts:
            Gaps: 
                Outer gap = 10
                Inner gap = 5

            - Main And Stack (side)
            - Main And Stack (bottom)
            - Monocle
                

See below for more on penrose:
- https://github.com/sminez/Penrose
- https://docs.rs/penrose
- https://sminez.github.io/penrose/

Thank you @sminez for this great library!
