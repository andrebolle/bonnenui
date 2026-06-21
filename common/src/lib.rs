pub trait Command: dyn_clone::DynClone {
    fn name(&self) -> &str;
    fn show(&mut self, ui: &mut eframe::egui::Ui) -> Option<i32>;
}

// This implements Clone for Box<dyn Command>
dyn_clone::clone_trait_object!(Command);