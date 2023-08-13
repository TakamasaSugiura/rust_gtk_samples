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
use gtk::{Builder, Window, Button};

static DIALOG_SRC:&str = include_str!("../resources/Dialog.glade");

fn main() {
    let application = gtk::Application::new(
        Some("my.app.hello_world"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_dialog() -> Window {
	let builder = Builder::from_string(DIALOG_SRC);
	let dialog = builder.object::<Window>("Dialog").expect("Couldn't get Dialog");
	return dialog;
}

fn build_ui(application: &gtk::Application) {
	let window_source = include_str!("../resources/MainWindow.glade");
    let builder = Builder::from_string(window_source);
    let window = builder.object::<Window>("MainWindow").expect("Couldn't get MainWindow");
    window.set_application(Some(application));
    // set default size
    window.set_default_size(300, 300);
    // set min size
    window.set_size_request(300, 300);
    // set title
    window.set_title("Hello world application");

    let open_button = builder.object::<Button>("OpenButton").expect("Couldn't get OpenButton");

    open_button.connect_clicked(|_| {
    	let dialog = build_dialog();
    	dialog.show_all()
    });

    // show main window
    window.show_all();
}
