// Copyright (C) 2023  Takamasa Sugiura

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
use gtk::prelude::*;
use gtk::{Builder, Window, Label};

fn main() {
    let _ = gtk::init();
    let builder = Builder::from_file("resources/HelloWorld.glade");

    if let Some(window) = builder.object::<Window>("HelloWorldWindow") {
        window.connect_delete_event(|_,_| {
            gtk::main_quit();
            Inhibit(false)
        });
        // set default size
        window.set_default_size(300, 300);
        // set min size
        window.set_size_request(300, 300);
        // set title
        window.set_title("Hello world application");

        let label = builder.object::<Label>("HelloWorldLabel").unwrap();
        // change label text
        label.set_text("Hello, world.");
        // show main window
        window.show_all();
        gtk::main();
    }
}