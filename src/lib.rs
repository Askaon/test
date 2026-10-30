//! Kalkulator z interfejsem graficznym (egui) dla Androida.

mod calc;

use eframe::egui;

/// Układ przycisków: 5 rzędów po 4.
const PRZYCISKI: [[&str; 4]; 5] = [
    ["C", "(", ")", "DEL"],
    ["7", "8", "9", "÷"],
    ["4", "5", "6", "×"],
    ["1", "2", "3", "-"],
    ["0", ",", "^", "+"],
];

#[derive(Default)]
struct Kalkulator {
    wyrazenie: String,
    komunikat: Option<String>,
    /// Po "=" wpisanie cyfry zaczyna nowe wyrażenie.
    po_wyniku: bool,
}

impl Kalkulator {
    fn nacisnij(&mut self, klawisz: &str) {
        self.komunikat = None;
        match klawisz {
            "C" => {
                self.wyrazenie.clear();
                self.po_wyniku = false;
            }
            "DEL" => {
                self.wyrazenie.pop();
                self.po_wyniku = false;
            }
            "=" => match calc::oblicz(&self.wyrazenie) {
                Ok(w) => {
                    self.wyrazenie = calc::formatuj(w);
                    self.po_wyniku = true;
                }
                Err(e) => self.komunikat = Some(e),
            },
            _ => {
                let to_cyfra = klawisz.chars().all(|c| c.is_ascii_digit() || c == ',');
                if self.po_wyniku && (to_cyfra || klawisz == "(") {
                    self.wyrazenie.clear();
                }
                self.po_wyniku = false;
                self.wyrazenie.push_str(klawisz);
            }
        }
    }
}

impl eframe::App for Kalkulator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let odstep = 8.0;
            ui.spacing_mut().item_spacing = egui::vec2(odstep, odstep);

            // --- Wyświetlacz ---
            ui.add_space(24.0);
            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                let tekst = if self.wyrazenie.is_empty() { "0" } else { &self.wyrazenie };
                ui.label(egui::RichText::new(tekst).size(40.0).strong());

                // Podgląd wyniku na żywo albo komunikat błędu.
                let podglad = if let Some(blad) = &self.komunikat {
                    egui::RichText::new(blad).size(22.0).color(egui::Color32::LIGHT_RED)
                } else if !self.po_wyniku && !self.wyrazenie.is_empty() {
                    match calc::oblicz(&self.wyrazenie) {
                        Ok(w) => egui::RichText::new(format!("= {}", calc::formatuj(w)))
                            .size(24.0)
                            .weak(),
                        Err(_) => egui::RichText::new(" ").size(24.0),
                    }
                } else {
                    egui::RichText::new(" ").size(24.0)
                };
                ui.label(podglad);
            });
            ui.add_space(16.0);

            // --- Klawiatura ---
            let rzedy = PRZYCISKI.len() as f32 + 1.0; // +1 na "="
            let szer = ((ui.available_width() - 3.0 * odstep) / 4.0).max(40.0);
            let wys = ((ui.available_height() - (rzedy - 1.0) * odstep - 16.0) / rzedy)
                .clamp(48.0, 120.0);

            let mut nacisniety: Option<&str> = None;

            for rzad in PRZYCISKI.iter() {
                ui.horizontal(|ui| {
                    for &k in rzad.iter() {
                        let kolor = match k {
                            "C" | "DEL" => egui::Color32::from_rgb(120, 45, 45),
                            "÷" | "×" | "-" | "+" | "^" | "(" | ")" => {
                                egui::Color32::from_rgb(55, 65, 90)
                            }
                            _ => egui::Color32::from_rgb(50, 50, 50),
                        };
                        let przycisk = egui::Button::new(
                            egui::RichText::new(k).size(28.0).color(egui::Color32::WHITE),
                        )
                        .fill(kolor);
                        if ui.add_sized([szer, wys], przycisk).clicked() {
                            nacisniety = Some(k);
                        }
                    }
                });
            }

            let rowna_sie = egui::Button::new(
                egui::RichText::new("=").size(32.0).color(egui::Color32::WHITE),
            )
            .fill(egui::Color32::from_rgb(30, 110, 60));
            if ui
                .add_sized([szer * 4.0 + odstep * 3.0, wys], rowna_sie)
                .clicked()
            {
                nacisniety = Some("=");
            }

            if let Some(k) = nacisniety {
                self.nacisnij(k);
            }
        });
    }
}

/// Punkt wejścia wywoływany przez Androida (NativeActivity).
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let opcje = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "Kalkulator",
        opcje,
        Box::new(|_cc| Ok(Box::new(Kalkulator::default()))),
    )
    .unwrap();
}
