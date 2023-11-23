
pub mod process;
pub mod d3d9;
pub mod wow;
pub mod xml_library;
pub mod profiles;

use std::ops::Index;
use std::path::PathBuf;
use std::thread::current;
use std::usize;
use eframe::egui;
use egui::{Vec2, Widget};
use egui_extras;

use crate::d3d9::d3d9::D3d9;
use crate::process::process_lib::Process;
use crate::profiles::abilities_tab::AbilitiesEnum;
use crate::profiles::profiles_lib::Profiles;
use crate::profiles::profiles_tab::ProfilesEnum;
use crate::wow::wow_hook::WowCheats;
use crate::xml_library::xml_handler::{extract_abilities_from_rotation, load_abilities_from_xml, load_rotations_from_xml};

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

    let profile_rot = unsafe { load_abilities_from_xml(PathBuf::from(r"C:\Users\sohai\RustroverProjects\rust_wow\target\debug\Profiles\Rogue sub pvp\79 Subt_ROGUE_Abilities.xml")) };
    profile_rot.iter().for_each(|s| {
        s.iter().for_each(|str| {
            if !str.is_empty() {
                println!("{str}");
            }
        });
    });

    let mut my_profile_enum = ProfilesEnum::Title("default".to_string());
    let mut stored_current_profile_name = "default".to_string();
    let mut my_abilities_enum = AbilitiesEnum::Title("default".to_string());
    let mut last_stored_rotation = vec![];

    let menu_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(380.0, 330.0)),
        maximized: false,
        resizable: false,
        ..Default::default()
    };
    let mut test_str = String::from("");
    let _ = eframe::run_simple_native("World of Warcraft Executor", menu_options, move |ctx, frame | {
        egui::SidePanel::left("Profiles panel").max_width(140.).resizable(false).show(ctx, |ui| {
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
                            my_profile_enum = current_enum;
                            last_stored_rotation = vec![];
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
                        if last_stored_rotation.is_empty() {
                            last_stored_rotation = load_rotations_from_xml(PathBuf::from(format!(r"C:\Users\sohai\RustroverProjects\rust_wow\target\debug\Profiles\{stored_current_profile_name}\79 Subt_ROGUE_Rotations.xml")));
                        }let first_rotation = last_stored_rotation.index(0);
                        extract_abilities_from_rotation(first_rotation.clone()).iter().for_each(|ability| {
                            let current_ability = ability.clone();
                            let current_enum = AbilitiesEnum::Title(current_ability.clone());
                            if ui.add_sized([ui.available_width(), 0.], egui::SelectableLabel::new(my_abilities_enum == current_enum, current_ability)).clicked() {
                                my_abilities_enum = current_enum;
                            }
                        });
                    }

                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job =
                    egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "lua".into());
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.add (
                    egui::TextEdit::multiline(&mut test_str)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(20)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
                ui.add_space(7.0);
                if ui.button("Execute").clicked() && !test_str.is_empty() {
                    unsafe { wow_cheat.second_run_string(test_str.as_mut_str()) }
                }
            });
        });
    });

}
