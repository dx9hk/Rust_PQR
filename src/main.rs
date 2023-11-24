
pub mod process;
pub mod d3d9;
pub mod wow;
pub mod xml_library;
pub mod profiles;

use std::path::PathBuf;
use std::usize;
use eframe::egui;
use egui::Vec2;
use egui_extras;

use crate::d3d9::d3d9::D3d9;
use crate::process::process_lib::Process;
use crate::profiles::abilities_tab::AbilitiesEnum;
use crate::profiles::profiles_lib::Profiles;
use crate::profiles::profiles_tab::ProfilesEnum;
use crate::wow::pqr_important::initialise_bot;
use crate::wow::wow_hook::WowCheats;
use crate::xml_library::xml_handler::{extract_abilities_name_from_rotation, extract_lua_from_ability, load_abilities_from_xml, load_rotations_from_xml};

fn main() {
    let wow_process = unsafe { Process::find("Wow.exe") };
    let our_process = unsafe { Process::find("rust_wow.exe") };

    let wow_d3d9_dll = unsafe { &wow_process.get_modules()["d3d9.dll"] };
    let our_d3d9_dll = unsafe { &our_process.get_modules()["d3d9.dll"] };
    println!("{wow_d3d9_dll:#X?}");
    println!("{our_d3d9_dll:#X?}");

    let our_d3d9 = unsafe { D3d9::new() };
    let endscene_ptr = our_d3d9.get_endscene();
    //println!("{endscene_ptr:#X?}");

    let wow_cheat = unsafe { WowCheats::new(endscene_ptr as usize) };

    let profiles_test = Profiles::new(PathBuf::from(r"C:\Users\sohai\RustroverProjects\rust_wow\target\debug\Profiles"));

    let mut my_profile_enum = ProfilesEnum::Title("default".to_string());
    let mut stored_current_profile_name = "default".to_string();
    let mut stored_current_profile_abilities = PathBuf::default();
    let mut stored_current_profile_rotations = PathBuf::default();
    let mut my_abilities_enum = AbilitiesEnum::Title("default".to_string());
    let mut last_stored_ability_list = vec![];
    let mut last_stored_rotation_list = vec![];

    let menu_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(600.0, 330.0)),
        maximized: false,
        resizable: false,
        ..Default::default()
    };
    let mut test_str = String::from("");
    let _ = eframe::run_simple_native("pqr_rs", menu_options, move |ctx, frame | {
        egui::SidePanel::left("Profiles panel")
            .max_width(140.)
            .resizable(false)
            .show(ctx, |ui| {
            ui.heading("Profiles");
            egui::ScrollArea::vertical()
                .id_source("scroll_profiles")
                .max_width(140.)
                .max_height(110.)
                .show(ui, |ui| {
                    profiles_test.clone().get_profiles().iter().for_each(|curr_profile| {
                        let current_profile = curr_profile.clone();
                        let current_profile_name = current_profile.get_profile_name();
                        let current_enum = ProfilesEnum::Title(current_profile_name.clone());
                        if ui.add_sized([ui.available_width(), 0.], egui::SelectableLabel::new(my_profile_enum == current_enum, current_profile_name.clone())).clicked() {
                            stored_current_profile_name = current_profile_name.clone();
                            stored_current_profile_abilities = current_profile.clone().get_abilities_path();
                            stored_current_profile_rotations = current_profile.clone().get_rotation_path();
                            my_profile_enum = current_enum;
                            last_stored_ability_list = vec![];
                            last_stored_rotation_list = vec![];
                        }
                    });
            });
            ui.add_space(15.);
            ui.separator();
            ui.add_space(15.);
            ui.heading("Profile Abilities");
            egui::ScrollArea::vertical()
                .id_source("scroll_abilities")
                .max_width(140.)
                .max_height(110.)
                .show(ui, |ui| unsafe {
                    if stored_current_profile_name != "default".to_string() {
                        if last_stored_ability_list.is_empty() {
                            last_stored_ability_list = load_abilities_from_xml(stored_current_profile_abilities.clone());
                            last_stored_rotation_list = load_rotations_from_xml(stored_current_profile_rotations.clone());
                        }
                        last_stored_ability_list.iter().for_each(|ability| {
                            let ability_name = extract_abilities_name_from_rotation(ability.clone());
                            if !ability_name.clone().is_empty() {
                                let ability_lua = extract_lua_from_ability(ability.clone());
                                let current_enum = AbilitiesEnum::Title(ability_name.clone());
                                if ui.add_sized([ui.available_width(), 0.], egui::SelectableLabel::new(my_abilities_enum == current_enum, ability_name)).clicked() {
                                    test_str = ability_lua;
                                    my_abilities_enum = current_enum;
                                }
                            }
                        })
                    }
                });
        });
        egui::TopBottomPanel::bottom("Options panel")
            .default_height(140.)
            .max_height(140.)
            //.resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| unsafe {
                    ui.add_space(7.);
                    //ui.button("Save changes");
                    if ui.button("Run Profile").clicked() {
                        initialise_bot(wow_cheat.clone(), last_stored_rotation_list.clone(), last_stored_ability_list.clone());
                    }
                    ui.add_space(7.);
                });

            });
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.set_height(190.);
                let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    let mut layout_job =
                        egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "lua".into());
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    egui::Window::new("Script Viewer")
                        .constraint_to(ctx.available_rect())
                        .default_size(ui.available_size())
                        //.default_width(300.)
                        //.default_height(180.)
                        .hscroll(true)
                        .movable(true)
                        .open(&mut true)
                        .resizable(true)
                        .title_bar(true)
                        .vscroll(true)
                        .show(ctx, |ui| {
                            ui.add (egui::TextEdit::multiline(&mut test_str)
                                        .font(egui::TextStyle::Monospace)
                                        .code_editor()
                                        .desired_rows(20)
                                        .lock_focus(true)
                                        .desired_width(f32::INFINITY)
                                        .layouter(&mut layouter),
                            );
                            ui.set_max_height(30.);
                        });
                    //ui.add_space(7.0);
                    //if ui.button("Execute").clicked() && !test_str.is_empty() {
                    //    unsafe { wow_cheat.second_run_string(test_str.as_mut_str()) }
                    //}
                });
        });
    });

}
