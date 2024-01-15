# turnwm - my personal implementation of the [Penrose](https://github.com/sminez/penrose) library.

Extremely minimal, dual-screen no workspaces. Intended for heavy use with tmux and my personal setup. This very well may not work the best or at all for you.

        M-b               Bar On
        M-C-b             Bar Off
        M-j               Focus Down
        M-k               Focus Up
        M-S-j             Swap Down
        M-S-k             Swap Up
        M-space           Swap Focus and Head
        M-S-space         Rotate Focus to Head
        M-w               Kill Focused
        M-Tab             Toggle Tag
        M-bracketright    Next Layout
        M-bracketleft     Previous Layout
        M-u               Increase Main
        M-d               Decrease Main
        M-l               Expand Main
        M-h               Shrink Main
        M-r               Rofi Launcher
        M-Return          Alacritty Terminal
        M-A-Escape        Power Menu
        M-C-f             Float Focused
        M-C-s             Sink Focused Floating
        M-C-S-s           Sink All Floating
        M-C-Right         Resize Floating Window Right
        M-C-Left          Resize Floating Window Left
        M-C-Up            Resize Floating Window Up
        M-C-Down          Resize Floating Window Down
        M-C-l             Reposition Floating Window Right
        M-C-h             Reposition Floating Window Left
        M-C-k             Reposition Floating Window Up
        M-C-j             Reposition Floating Window Down
        M-1               Move to Tag 1
        M-2               Move to Tag 2
        M-3               Move to Tag 3
        M-S-1             Move Focused to Tag 1
        M-S-2             Move Focused to Tag 2
        M-S-3             Move Focused to Tag 3

        Layouts:
            Gaps: 
                Outer gap = 10
                Inner gap = 5

            - Main And Stack (side)
            - Main And Stack (bottom)
            - FlexTall
            - Monocle
                

See below for more on penrose:
- https://github.com/sminez/Penrose
- https://docs.rs/penrose
- https://sminez.github.io/penrose/

Thank you @sminez for this great library!
