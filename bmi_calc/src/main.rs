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
use gtk::{Builder, Button, Window, Entry};

fn main() {
    let application = gtk::Application::new(
        Some("my.app.input_dialog"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn convert_error<T>(result: Result<f32, T>) -> Result<f32, String> {
    return match result {
        Ok(val) => Ok(val),
        Err(_) => Err("Error".into())
    };
}

fn calculate(height_str: String, weight_str: String) -> Result<f32, String> {
    let height = convert_error(height_str.parse::<f32>())?;
    let weight = convert_error(weight_str.parse::<f32>())?;
    if height < 0_f32 || weight < 0_f32 {
        return Err("Error".into());
    }
    let bmi = weight / ((height / 100_f32) * (height / 100_f32));
    if bmi == f32::INFINITY {
        return Err("Error".into());
    }
    return Ok(bmi);
}

fn build_ui(application: &gtk::Application) {
    let src = include_str!("../resources/BmiCalculator.glade");
    let builder = Builder::from_string(src);
    let main_window = builder.object::<Window>("MainWindow").expect("Couldn't get MainWindow");
    main_window.set_application(Some(application));
    main_window.set_title("BMI calculator");
    let height_entry = builder.object::<Entry>("HeightEntry").expect("Couldn't get HeightEntry");
    let weight_entry = builder.object::<Entry>("WeightEntry").expect("Couldn't get WeightEntry");
    let bmi_entry = builder.object::<Entry>("BmiEntry").expect("Couldn't get BmiEntry");
    let calculate_button = builder.object::<Button>("CalculateButton").expect("Couldn't get CalculateButton");
    
    calculate_button.connect_clicked(move |_| {
        let calc_result = calculate(height_entry.text().to_string(), weight_entry.text().to_string());
        match calc_result {
            Ok(bmi) => bmi_entry.set_text(&format!("{}", bmi)),
            Err(message) => bmi_entry.set_text(&message)
        }
    }
    );

    main_window.show_all();
}
