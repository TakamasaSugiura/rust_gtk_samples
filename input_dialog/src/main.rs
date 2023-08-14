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
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Builder, Button, Dialog, Entry, MessageDialog};

fn main() {
    let application = gtk::Application::new(
        Some("my.app.input_dialog"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let builder = Builder::from_file("resources/InputDialog.glade");
    let input_dialog = builder.object::<Dialog>("InputDialog").expect("Couldn't get InputDialog");
    input_dialog.set_application(Some(application));
    // set default size
    input_dialog.set_default_size(300, 300);
    // set min size
    input_dialog.set_size_request(300, 300);
    // set title
    input_dialog.set_title("Input dialog application");
    let name_entry = builder.object::<Entry>("NameEntry").expect("Couldn't get NameEntry");
    let ok_button = builder.object::<Button>("OkButton").expect("Couldn't get OkButton");
    let message_dialog = builder.object::<MessageDialog>("MessageDialog").expect("Couldn't get MessageDialog");
    message_dialog.set_title("Message");
    message_dialog.set_transient_for(Some(&input_dialog));
    message_dialog.set_modal(true);
    let close_button = builder.object::<Button>("CloseButton").expect("Couldn't get CloseButton");
    message_dialog.connect_delete_event(|message_dialog,_| {
       	message_dialog.hide();
        Inhibit(true)
    });
    close_button.connect_clicked(clone!(@weak message_dialog => move |_| message_dialog.close()));
	
    ok_button.connect_clicked(clone!(@weak message_dialog => move |_| {
    	let text = name_entry.text().to_string();
		message_dialog.set_text(Some(&format!("Hello {}.",text)));
    	message_dialog.show_all()
    }
    ));

    input_dialog.show_all();
}
