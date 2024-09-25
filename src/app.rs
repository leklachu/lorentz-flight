// Notes::
//
// and bugs: apparent precision loss in parameters, on 'launch' or drag
// could show dist & accel in SI, separate to input in ly|g
// And: Final distance is surprisingly higher than specified
//
// and todo: format outputs nicely
// have more segments but fit to width or scrollable

use crate::voyage;
use crate::voyage::Event;

const _LY: f64 = 3e8 * 3600.0 * 24.0 * 365.0;

pub struct App {
   // label: String,
   lightyears: f64,
   gees: f64,
   launched: bool,
   arrived: bool,
   progress: usize,
   events: [Option<Event>; 101],
   latest: Event,
   journeys_end: f64,
   acceleration: f64,
}

impl Default for App {
   fn default() -> Self {
      Self {
         // label: "G'day, me!".to_owned(),
         lightyears: 14.0,
         gees: 1.5,
         launched: false,
         arrived: false,
         progress: 0,
         events: [None; 101],
         latest: Event::default(),
         journeys_end: 0.0,
         acceleration: 0.0,
      }
   }
}

impl App {
   pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
      // n.b. this is where one can customise look&feel with
      // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

      // if let Some(storage)...

      // Default::default()
      let mut e = App::default();
      e.events[0] = Some(Event::default());
      return e;
   }
}

impl eframe::App for App {
   // fn save

   /// Called each time the UI needs repainting
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      // If launched, progress the ship
      // if self.launched && self.progress < 1.0 {
      if self.launched && !self.arrived {
         self.progress += 1;
         let milestone = self.journeys_end / 100.0 * self.progress as f64;
         let acceleration = if self.progress <= 50 {
            self.acceleration
         } else {
            -self.acceleration
         };
         self.latest = voyage::arrive(self.latest, acceleration, milestone);
         // println!(
         //    "diff/speed: {0}s",
         //    (self.latest.here - milestone) / self.latest.speed
         // );
         self.events[self.progress] = Some(self.latest.clone());
         if self.progress >= 100 || self.latest.speed < 0.0 {
            self.arrived = true;
         }
      } else if self.arrived {
         if let Some(e) = self.events[self.progress] {
            self.latest = e;
         } else {
            self.latest = Event::default();
         }
      }

      // Draw the UI

      egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
         egui::menu::bar(ui, |ui| {
            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
               ui.menu_button("File", |ui| {
                  if ui.button("Quit").clicked() {
                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                  }
               });
               ui.add_space(16.0);
            }

            // egui::widgets::global_dark_light_mode
         });
      });

      egui::CentralPanel::default().show(ctx, |ui| {
         ui.heading("The Lorentz Journey");

         ui.horizontal(|ui| {
            ui.label("How many lightyears will you travel?");
            if !self.launched {
               ui.add(
                  egui::DragValue::new(&mut self.lightyears)
                     .range(0.1..=64.0)
                     .speed(0.1),
               );
            } else {
               let l = self.lightyears;
               ui.add(egui::DragValue::new(&mut self.lightyears).range(l..=l));
            }
         });

         ui.horizontal(|ui| {
            ui.label("And how many g's will you pull?");
            if !self.launched {
               ui.add(
                  egui::DragValue::new(&mut self.gees)
                     .range(0.1..=10.0)
                     .speed(0.1),
               );
            } else {
               let g = self.gees;
               ui.add(egui::DragValue::new(&mut self.gees).range(g..=g));
            }
         });

         if !self.launched {
            if ui.button("Launch!").clicked() {
               self.launched = true;
               self.journeys_end = self.lightyears * 3e8 * 3600.0 * 24.0 * 365.0;
               self.acceleration = self.gees * 9.81;
            }
         }

         ui.separator();

         let e = self.latest;
         ui.vertical(|ui| {
            ui.label(format!(
               "Distance: {0} ly",
               e.here / 3e8 / 3600.0 / 24.0 / 365.0
            ));
            ui.label(format!("Time: {0} y", e.now / 3600.0 / 24.0 / 365.0));
            ui.label(format!("Ship Time: {0} y", e.mtime / 3600.0 / 24.0 / 365.0));
            ui.label(format!("Speed: {0}c", e.speed / 3e8));
         });

         ui.add(
            egui::ProgressBar::new((self.progress as f32) / 100.0)
               .show_percentage()
               .animate(self.launched && !self.arrived),
         );

         if self.arrived {
            ui.add(
               egui::Slider::new(&mut self.progress, 0..=100)
                  // .integer()
                  // .step_by(1)
                  .suffix("%"),
            );
         }

         // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
         //    powered_by_egui_and_eframe(ui);
         // });
      });
   }
}

// /// The result of one leg of the journey
// fn _leg(ui: &mut egui::Ui, event: Option<Event>) {
//    if let Some(e) = event {
//       ui.vertical(|ui| {
//          ui.label(format!(
//             "Distance: {0} ly",
//             e.here / 3e8 / 3600.0 / 24.0 / 365.0
//          ));
//          ui.label(format!("Time: {0} y", e.now / 3600.0 / 24.0 / 365.0));
//          ui.label(format!("Ship Time: {0} y", e.mtime / 3600.0 / 24.0 / 365.0));
//          ui.label(format!("Speed: {0}c", e.speed / 3e8));
//       });
//    } else {
//       ui.label("...");
//    }
// }
//
// fn _powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//    ui.horizontal(|ui| {
//       ui.spacing_mut().item_spacing.x = 0.0;
//       ui.label("Displayed with ");
//       ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//       ui.label(" and ");
//       ui.hyperlink_to(
//          "eframe",
//          "https://github.com/emilk/egui/tree/master/crates/eframe",
//       );
//       ui.label(".");
//    });
// }
