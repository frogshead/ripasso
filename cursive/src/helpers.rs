/*  Ripasso - a simple password manager
    Copyright (C) 2019-2020 Joakim Lundborg, Alexander Kjäll

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use cursive::views::{Dialog, OnEventView, TextView};

use cursive::event::Key;
use cursive::Cursive;

use ripasso::pass;
use clipboard::ClipboardProvider;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

use pass::Result;

/// Displays an error in a cursive dialog
pub fn errorbox(ui: &mut Cursive, err: &pass::Error) {
    let text = match err {
        pass::Error::RecipientNotInKeyRing(key_id) => super::CATALOG
            .gettext("Team member with key id {} isn't in your GPG keyring, fetch it first")
            .to_string()
            .replace("{}", key_id),
        _ => format!("{}", err),
    };

    let d = Dialog::around(TextView::new(text))
        .dismiss_button(super::CATALOG.gettext("Ok"))
        .title(super::CATALOG.gettext("Error"));

    let ev = OnEventView::new(d).on_event(Key::Esc, |s| {
        s.pop_layer();
    });

    ui.add_layer(ev);
}

/// Copies content to the clipboard.
/// It first tries to copy to a wayland clipboard, and if that's not availible due to that the
/// user runs x11/mac/windows we instead try the more generic clipboard crate.
pub fn set_clipboard(content: String) -> Result<()> {
    let opts = Options::new();
    let result = opts.copy(Source::Bytes(content.clone().into_bytes().into()), MimeType::Autodetect);
    match result {
        Err(_) => {
            let mut ctx = clipboard::ClipboardContext::new()?;
            ctx.set_contents(content)?;
        },
        _ => {}
    }
    Ok(())
}