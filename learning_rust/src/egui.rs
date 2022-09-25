use eframe::egui;
use eframe::epaint::CircleShape;
use egui::RichText;
use epaint::Stroke;

pub struct MyApp 
{
    name: String,
    age: u32,
}

impl Default for MyApp 
{
    fn default() -> Self 
    {
        Self 
        {
            name: "Muhamed".to_owned(),
            age: 21,
        }
    }
}


struct eguiCircle
{
    shape: epaint::CircleShape
}


impl egui::Widget for &mut eguiCircle
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response 
    { 
        // Teken de cirkel op het scherm
        let circle_shape = egui::Shape::circle_stroke(
            epaint::Pos2{x: self.shape.center.x ,y: self.shape.center.y}, 
            self.shape.radius, 
            self.shape.stroke
        );

        todo!(); //TODO: return response
    }
}


impl eframe::App for MyApp 
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {
        egui::CentralPanel::default().show(ctx, |ui| 
        {
            ui.heading("My egui Application");
            ui.horizontal(|ui| 
            {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Click each year").clicked() 
            {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // Elke stad is nu een button... Moet nog een cirkel widget maken        

        });
    }
}