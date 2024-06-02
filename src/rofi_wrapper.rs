use penrose::x::XConn;
use penrose::Xid;
use penrose::{x11rb::RustConn, builtin::actions::key_handler, x::XConnExt};

use crate::KeyHandler;
use crate::DZEN_CENTER_X;
use crate::dzen_wrapper::{Dzen, DzenBuilder};

use rofi;

pub fn rofi_clients() -> KeyHandler {
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
                DZEN_CENTER_X - 150,
                0,
                15,
                300
            ).set_p(1).set_title_align('c');
            text.push_str("echo 'No clients running'");
            dzen.build().run(&text, "zsh");
            Ok(())
        } else {
            let t: Vec<String> = text.lines().map(|x| x.to_string()).collect();
            let choice = match rofi::Rofi::new(&t).run() {
                Ok(choice) => {
                    let c: Vec<String> = choice.split(' ').map(|x| x.to_string()).collect();
                    let choice_xid: String = c.last().unwrap().trim_matches(&['[', ']']).into();
                    let choice_xid: u32 = choice_xid.parse().unwrap();
                    let ctx = format!("Selected => {:?}", choice);
                    x.focus(Xid::from(choice_xid)).unwrap();
                    ctx
                }
                Err(rofi::Error::Interrupted) => {
                    "No Selection Made".to_string()
                }
                Err(e) => {
                    let ctx_err = format!("Error => {}", &e);
                    ctx_err
                }

            };

            let dzen = Dzen::new(
                0,
                0,
                15,
                300
            ).set_p(5).set_title_align('c');
            dzen.build().run(format!("echo '{}'" ,choice).as_str(), "zsh");
            Ok(())

        }
    })
}
