use crate::database_logic::data_structs::Participant;
use crate::database_logic::database::DataBase;
use crate::windows::participant::{
    add_participant_window::AddWindow, edit_participant_window::EditWindow,
};
use egui::{Context, Ui};

#[derive(Default)]
pub struct ParticipantsView {
    db: DataBase,
    search_response: String,
    add_window: AddWindow,
    edit_window: EditWindow,

    selected_participant: Participant,
}

impl ParticipantsView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let participants = self.db.get_all_participant();
        let size = participants.len();
        ui.vertical_centered(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search_response)
                    .hint_text("🔍 Type to search..."),
            );
        });
        ui.separator();
        egui::Grid::new("headings")
            .num_columns(5)
            .spacing([30.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Date of Birth");
                ui.label("Phone Number");
                ui.label("Email");
                ui.label("Support Ratio");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("participant_results")
                .num_columns(5)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for index in 0..size {
                        if ui
                            .button(format!(
                                "{} {}",
                                &participants[index].first_name, &participants[index].last_name
                            ))
                            .clicked()
                        {
                            self.selected_participant = participants[index].clone();
                        }
                        ui.label(match participants[index].dob {
                            None => String::new(),
                            Some(value) => value.format("%d-%m-%Y").to_string(),
                        });
                        ui.label(match participants[index].phone.clone() {
                            None => {String::new()}
                            Some(value) => {value.to_string()}
                        });
                        ui.label(match participants[index].email.clone() {
                            None => {String::new()}
                            Some(value) => {value.to_string()}
                        });
                        ui.label(match &participants[index].support_ratio {
                            None => {String::new()}
                            Some(value) => {format!("1:{}", value)}
                        });
                        ui.end_row();
                    }
                });
        });
    }
    fn right_panel_view(&mut self, ui: &mut Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                if self.selected_participant.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(format!(
                            "{} {}",
                            self.selected_participant.first_name,
                            self.selected_participant.last_name
                        ));
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!(
                            "Name: {} {}",
                            self.selected_participant.first_name,
                            self.selected_participant.last_name
                        ));
                        ui.label(format!(
                            "medicare_number: {}",
                            self.selected_participant.medicare_number
                        ));
                        ui.label(format!(
                            "dob: {}",
                            self.selected_participant
                                .dob
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "address: {}",
                            self.selected_participant
                                .address
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "suburb: {}",
                            self.selected_participant.suburb.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "postcode: {}",
                            self.selected_participant
                                .postcode
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "phone: {}",
                            self.selected_participant.phone.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "email: {}",
                            self.selected_participant.email.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "medical_notes: {}",
                            self.selected_participant
                                .medical_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "dietary_notes: {}",
                            self.selected_participant
                                .dietary_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "physical_notes: {}",
                            self.selected_participant
                                .physical_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "other_notes: {}",
                            self.selected_participant
                                .other_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "support_ratio: {}",
                            self.selected_participant
                                .support_ratio
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "photo_permission: {}",
                            self.selected_participant.photo_permission.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "private_hospital_preference: {}",
                            self.selected_participant
                                .private_hospital_preference
                                .unwrap_or(false)
                        ));
                        ui.label(format!(
                            "private_health_insurancer: {}",
                            self.selected_participant
                                .private_health_insurer
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "private_health_number: {}",
                            self.selected_participant
                                .private_health_number
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "communication_preference: {}",
                            self.selected_participant
                                .communication_preference
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "ndis_plan_number: {}",
                            self.selected_participant
                                .ndis_plan_number
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "ndis_plan_start_date: {}",
                            self.selected_participant
                                .ndis_plan_start_date
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "core_funding: {}",
                            self.selected_participant.core_funding.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "capacity_building_funding: {}",
                            self.selected_participant
                                .capacity_building_funding
                                .unwrap_or(false)
                        ));
                        ui.label(format!(
                            "self_managed: {}",
                            self.selected_participant.self_managed.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "plan_managed: {}",
                            self.selected_participant.plan_managed.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "ndis_plan_end_date: {}",
                            self.selected_participant
                                .ndis_plan_end_date
                                .unwrap_or_default()
                        ));
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.heading("SELECT PARTICIPANT");
                    });
                }
            });
    }

    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .max_height(25.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("➕ Create").clicked() {
                        self.add_window.open = !self.add_window.open;
                    };
                    if ui.button("✏ Edit").clicked() {
                        self.edit_window.open = !self.edit_window.open;
                    };
                    if ui.button("RESET DB").clicked() {
                        self.db.drop_db().unwrap();
                        self.db.create_db().unwrap();
                    };
                });
            });
    }
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ui, ctx);
        self.edit_window
            .ui(ui, ctx, self.selected_participant.clone());
    }
}
