use penrose::{x11rb::RustConn, builtin::actions::key_handler, x::XConnExt};
use tracing::{info, warn};

use crate::KeyHandler;

// returned from the build() function in Dzen
pub struct DzenRunner {
    options: String,
}

impl DzenRunner {
    pub fn kill() {
        std::process::Command::new("pkill")
            .arg("dzen2")
            .spawn()
            .expect("failed to execute process");
    }

    pub fn run(&self, text: &str, shell: &str) {
        // kill any existing dzen2 processes, this can be removed if you want to have multiple dzen2 windows
        // running at the same time. I prefer to have them killed so I don't have to worry about if they're
        // persistent or not
        Self::kill();
        let command = format!("{} | {}", text, self.options);
        std::process::Command::new(shell)
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("failed to execute process");
    }

}

// dzen2 builder trait
pub trait DzenBuilder {
    fn set_foreground(self, fg: &str) -> Self;
    fn set_background(self, bg: &str) -> Self;
    fn set_font(self, font: &str) -> Self;
    fn set_title_align(self, align: char) -> Self;
    fn set_title_width(self, width: i32) -> Self;
    fn set_slave_align(self, align: char) -> Self;
    fn set_lines(self, lines: u32) -> Self;
    fn set_e(self, e: &str) -> Self;
    fn set_e_easy(self) -> Self;
    fn add_menu(self) -> Self;
    fn set_p(self, p: u32) -> Self;
    fn set_x(self, x: u32) -> Self;
    fn set_y(self, y: u32) -> Self;
    fn set_h(self, h: u32) -> Self;
    fn set_w(self, w: u32) -> Self;
    fn remove_option(self, option: &str) -> Self;
}

// dzen2 shitty temp wrapper
// go to https://github.com/robm/dzen for more info on dzen2 functionality
#[derive(Default, Clone, Debug)]
pub struct Dzen {
    // foreground color
    fg: Option<String>,
    // background color
    bg: Option<String>,
    // font
    font: Option<String>,
    // alignement of title window content
    // l = left, c = center, r = right
    t_align: Option<char>,
    // title window width
    t_width: Option<i32>,
    // alignment of slave window
    // see t_align for possible values
    s_align: Option<char>,
    // visible lines
    l: Option<u32>,
    // events and actions
    e: Option<String>,
    // menu mode
    m: Option<bool>,
    // persist EOF (optional timeout in seconds), 0 for no timeout
    p: Option<u32>,
    // window x position
    x: Option<u32>,
    // window y position
    y: Option<u32>,
    // window height
    h: Option<u32>,
    // window width
    w: Option<u32>,
}

impl Dzen {
    pub fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            font: None,
            t_align: None,
            t_width: None,
            s_align: None,
            l: None,
            e: None,
            m: None,
            p: None,
            x: None,
            y: None,
            h: None,
            w: None,
        }
    }

    pub fn new(x: u32, y: u32, height: u32, width: u32) -> Self {
        Self {
            fg: None,
            bg: None,
            font: None,
            t_align: None,
            t_width: None,
            s_align: None,
            l: None,
            e: None,
            m: None,
            p: None,
            x: Some(x),
            y: Some(y),
            h: Some(height),
            w: Some(width),
        }
    }

    pub fn build(&self) -> DzenRunner {
        let mut dzen: Vec<String> = vec![String::from("dzen2")];

        if self.fg.is_some() {
            dzen.push(format!("-fg '{}'", self.fg.as_ref().unwrap()));
        }

        if self.bg.is_some() {
            dzen.push(format!("-bg '{}'", self.bg.as_ref().unwrap()));
        }

        if self.font.is_some() {
            dzen.push(format!("-fn '{}'", self.font.as_ref().unwrap()));
        }

        if self.t_align.is_some() {
            if self.t_align.as_ref().unwrap() == &'l' || self.t_align.as_ref().unwrap() == &'c' || self.t_align.as_ref().unwrap() == &'r' {
                dzen.push(format!("-ta '{}'", self.t_align.as_ref().unwrap()));
            } else {
                info!("invalid t_align value, defaulting to left");
                dzen.push(String::from("-ta 'l'"));
            }
        }

        if self.t_width.is_some() {
            dzen.push(format!("-tw '{}'", self.t_width.as_ref().unwrap()));
        }

        if self.s_align.is_some() {
            if self.s_align.as_ref().unwrap() == &'l' || self.s_align.as_ref().unwrap() == &'c' || self.s_align.as_ref().unwrap() == &'r' {
                dzen.push(format!("-sa '{}'", self.s_align.as_ref().unwrap()));
            } else {
                info!("invalid s_align value, defaulting to left");
                dzen.push(String::from("-sa 'l'"));
            }
        }

        if self.l.is_some() {
            dzen.push(format!("-l '{}'", self.l.as_ref().unwrap()));
        }

        if self.e.is_some() {
            dzen.push(format!("-e '{}'", self.e.as_ref().unwrap()));
        }

        if self.m.is_some() {
            dzen.push(String::from("-m"));
        }

        if self.p.is_some() {
            dzen.push(format!("-p '{}'", self.p.as_ref().unwrap()));
        }

        if self.x.is_some() {
            dzen.push(format!("-x '{}'", self.x.as_ref().unwrap()));
        }

        if self.y.is_some() {
            dzen.push(format!("-y '{}'", self.y.as_ref().unwrap()));
        }

        if self.h.is_some() {
            dzen.push(format!("-h '{}'", self.h.as_ref().unwrap()));
        }

        if self.w.is_some() {
            dzen.push(format!("-w '{}'", self.w.as_ref().unwrap()));
        }

        let dzen = dzen.join(" ");

        DzenRunner {
            options: dzen,
        }
    }
}

impl DzenBuilder for Dzen {
    fn set_foreground(mut self, fg: &str) -> Self {
        self.fg = Some(fg.to_string());
        self
    }

    fn set_background(mut self, bg: &str) -> Self {
        self.bg = Some(bg.to_string());
        self
    }

    fn set_font(mut self, font: &str) -> Self {
        self.font = Some(font.to_string());
        self
    }

    fn set_title_align(mut self, align: char) -> Self {
        self.t_align = Some(align);
        self
    }

    fn set_title_width(mut self, width: i32) -> Self {
        self.t_width = Some(width);
        self
    }

    fn set_slave_align(mut self, align: char) -> Self {
        self.s_align = Some(align);
        self
    }

    fn set_lines(mut self, lines: u32) -> Self {
        self.l = Some(lines);
        self
    }

    // Supported events:
    // onstart             Perform actions right after startup
    // onexit              Perform actions just before exiting
    // onnewinput          Perform actions if there is new input for the slave window
    // button1             Mouse button1 released 
    // button2             Mouse button2 released
    // button3             Mouse button3 released
    // button4             Mouse button4 released (usually scrollwheel)
    // button5             Mouse button5 released (usually scrollwheel)
    // button6             Mouse button6 released
    // button7             Mouse button7 released
    // entertitle          Mouse enters the title window
    // leavetitle          Mouse leaves the title window
    // enterslave          Mouse enters the slave window
    // leaveslave          Mouse leaves the slave window
    // sigusr1             SIGUSR1 received 
    // sigusr2             SIGUSR2 received
    // key_KEYNAME         Keyboard events (*)
    //
    // Supported actions:
    // exec:command1:..:n  execute all given options
    // menuexec            executes selected menu entry
    // exit:retval         exit dzen and return 'retval'
    // print:str1:...:n    write all given options to STDOUT
    // menuprint           write selected menu entry to STDOUT
    // collapse            collapse (roll-up) slave window
    // uncollapse          uncollapse (roll-down) slave window
    // togglecollapse      toggle collapsed state
    // stick               stick slave window
    // unstick             unstick slave window
    // togglestick         toggle sticky state
    // hide                hide title window
    // unhide              unhide title window
    // togglehide          toggle hide state
    // raise               raise window to view (above all others)
    // lower               lower window (behind all others)
    // scrollhome          show head of input
    // scrollend           show tail of input
    // scrollup:n          scroll slave window n lines up   (default n=1)
    // scrolldown:n        scroll slave window n lines down (default n=1)
    // grabkeys            enable keyboard support
    // ungrabkeys          disable keyboard support
    // grabmouse           enable mouse support 
    //                     only needed with specific windowmanagers, such as fluxbox
    // ungrabmouse         release mouse
    //                     only needed with specific windowmanagers, such as fluxbox
    //
    // (*) KEYNAME is the name of the key as defined in /usr/include/X11/keysymdef.h
    // 
    // The command line syntax is as follows:
    //  -e 'event1=action1:option1:...option<n>,...,action<m>;...;event<l>'
    fn set_e(mut self, e: &str,) -> Self {
        self.e = Some(e.to_string());
        self
    }

    // event and action easy defaults
    fn set_e_easy(mut self) -> Self {
        self.e = Some("button1=exit;button2=exit;button3=exit;button4=scrollup:3;button5=scrolldown:3;entertitle=uncollapse;leaveslave=collapse".to_string());
        self
    }

    fn add_menu(mut self) -> Self {
        self.m = Some(true);
        self
    }

    fn set_p(mut self, p: u32) -> Self {
        self.p = Some(p);
        self
    } 

    fn set_x(mut self, x: u32) -> Self {
        self.x = Some(x);
        self
    }

    fn set_y(mut self, y: u32) -> Self {
        self.y = Some(y);
        self
    }

    fn set_h(mut self, h: u32) -> Self {
        self.h = Some(h);
        self
    }

    fn set_w(mut self, w: u32) -> Self {
        self.w = Some(w);
        self
    }

    // remove option from dzen2 call
    // options: fg, bg, font, t_align, t_width, s_align, l, e, m, p, x, y, h, w
    // options are case insensitive
    fn remove_option(mut self, mut option: &str) -> Self {
        option = option.trim();
        match option.to_lowercase().as_str() {
            "fg" => self.fg = None,
            "bg" => self.bg = None,
            "font" => self.font = None,
            "t_align" => self.t_align = None,
            "t_width" => self.t_width = None,
            "s_align" => self.s_align = None,
            "l" => self.l = None,
            "e" => self.e = None,
            "m" => self.m = None,
            "p" => self.p = None,
            "x" => self.x = None,
            "y" => self.y = None,
            "h" => self.h = None,
            "w" => self.w = None,
            _ => {
                warn!("option {} not found", option);
            }
        }
        self
    }

}


// dzen call to display all currently running clients and their tags
pub fn dzen_clients() -> KeyHandler {
    key_handler(move |state, x: &RustConn| {
        let mut text = String::new();

        for w in state.client_set.workspaces() {
            let tag = w.tag();
            let clients: Vec<_> = w.clients().collect::<Vec<_>>();
            for xid in clients {
                let name = x.window_title(*xid).unwrap();
                text.push_str(&format!("{}: {} [{}]\n", tag, name, xid));
            }
        }
        if text.is_empty() {
            let dzen = Dzen::new(
                0,
                0,
                15,
                300
            ).set_p(1).set_title_align('c');
            text.push_str("echo 'No clients running'");
            dzen.build().run(&text, "zsh");
            Ok(())
        } else {
            let mut lines = text.lines().count();

            if lines > 10 {
                lines = 10;
            }

            let dzen = Dzen::new(
                0,
                0,
                15,
                300
            ).set_p(0)
                .set_title_align('c')
                .set_slave_align('l')
                .set_lines(lines as u32)
                .add_menu()
                .set_e_easy();

            
            let text = "CLIENTS>>>\n".to_owned() + &text;
            info!("text: {}", text);

            dzen.build().run(format!("echo '{}'", text).as_str(), "zsh");
            Ok(())

        }
    })
}
