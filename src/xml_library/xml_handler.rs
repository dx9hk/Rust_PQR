
use std::path::PathBuf;
use windows::core::{ComInterface, PCSTR, PCWSTR, w};
use windows::Win32::Data::Xml::XmlLite::{CreateXmlReader, IXmlReader, XmlReaderProperty_DtdProcessing, DtdProcessing_Prohibit, XmlNodeType_None, XmlNodeType_Element, XmlNodeType_Attribute, XmlNodeType_Text, XmlNodeType_EndElement};
use windows::Win32::System::Com::STGM_READ;
use windows::Win32::UI::Shell::SHCreateStreamOnFileA;

const PROFILE_SIZE: usize = 1024;
// Decode xml input
pub fn decode_xml(xml_input: &str) -> String {
    let decoded_xml = xml_input.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;","'").replace("&amp;","&");
    decoded_xml
}
/// Encode xml input
pub fn encode_xml(xml_input: &str) -> String {
    let encoded_xml = xml_input.replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&apos;").replace("&", "&amp;");
    encoded_xml
}
/// Translate rotation to xml
pub fn translate_rot_to_xml(player_class: &str, rotation_array: Vec<Vec<&str>>) -> String {
    let encoding_xml = "<?xml version=\"1.0\" encoding=\"utf-8\" ?>".to_string();
    let player_class_xml = format!("<{player_class}>");
    let player_end_class_xml = format!("</{player_class}>");
    let mut xml_format = "<Rotation><RotationName>%RNAME%</RotationName><RotationDefault>%RDEFAULT%</RotationDefault><RotationList>%RLIST%</RotationList><RequireCombat>%RCOMBAT%</RequireCombat><RotationNotes>%RNOTES%</RotationNotes></Rotation>".to_string();
    let mut list_of_xml = vec![];
    for i in 0..PROFILE_SIZE {
        if !rotation_array[i][0].is_empty() {
            // Encode name
            let mut xml_value = encode_xml(rotation_array[i][0]);
            xml_format = xml_format.replace("%RNAME%", xml_value.as_str());
            // Encode default
            xml_value = encode_xml(rotation_array[i][1]);
            xml_format = xml_format.replace("%RDEFAULT%", xml_value.as_str());
            // Encode list
            xml_value = encode_xml(rotation_array[i][2]);
            xml_format = xml_format.replace("%RLIST%", xml_value.as_str());
            // Encode combat
            xml_value = encode_xml(rotation_array[i][3]);
            xml_format = xml_format.replace("%RCOMBAT%", xml_value.as_str());
            // Encode notes
            xml_value = encode_xml(rotation_array[i][4]);
            xml_format = xml_format.replace("%RNOTES%", xml_value.as_str());
            // Push onto array
            list_of_xml.push(xml_value);
        }
    }
    // Setup return buffer
    let return_string = format!("{encoding_xml}{player_class_xml}");
    let reduced_array = list_of_xml.iter().fold(String::new(), |str_1, str_2| format!("{str_1}{str_2}"));

    format!("{return_string}{reduced_array}{player_end_class_xml}")
}
/// Translate ability to xml
pub fn translate_ability_to_xml(player_class: &str, ability_array: Vec<Vec<&str>>) -> String {
    let encoding_xml = "<?xml version=\"1.0\" encoding=\"utf-8\" ?>".to_string();
    let player_class_xml = format!("<{player_class}>");
    let player_end_class_xml = format!("</{player_class}>");
    let mut xml_format = "<Ability><Name>%SPELLNAME%</Name><Default>%DEFAULT%</Default><SpellID>%SPELLID%</SpellID><Actions>%ACTIONS%</Actions><Lua>%LUA%</Lua><RecastDelay>%RECAST%</RecastDelay><Target>%TARGET%</Target><CancelChannel>%CANCELCHANNEL%</CancelChannel><LuaBefore>%LBEFORE%</LuaBefore><LuaAfter>%LAFTER%</LuaAfter></Ability>".to_string();
    let mut list_of_xml = vec![];
    for i in 0..PROFILE_SIZE {
        if !ability_array[i][0].is_empty() {
            // Encode name
            let xml_value = encode_xml(ability_array[i][0]);
            xml_format = xml_format.replace("%SPELLNAME%", xml_value.as_str());
            // Encode default
            let xml_value = encode_xml(ability_array[i][1]);
            xml_format = xml_format.replace("%DEFAULT%", xml_value.as_str());
            // Encode list
            let xml_value = encode_xml(ability_array[i][2]);
            xml_format = xml_format.replace("%SPELLID%", xml_value.as_str());
            // Encode combat
            let xml_value = encode_xml(ability_array[i][3]);
            xml_format = xml_format.replace("%ACTIONS%", xml_value.as_str());
            // Encode notes
            let xml_value = encode_xml(ability_array[i][4]);
            xml_format = xml_format.replace("%LUA%", xml_value.as_str());
            // Encode name
            let xml_value = encode_xml(ability_array[i][5]);
            xml_format = xml_format.replace("%RECAST%", xml_value.as_str());
            // Encode default
            let xml_value = encode_xml(ability_array[i][6]);
            xml_format = xml_format.replace("%TARGET%", xml_value.as_str());
            // Encode list
            let xml_value = encode_xml(ability_array[i][7]);
            xml_format = xml_format.replace("%CANCELCHANNEL%", xml_value.as_str());
            // Encode combat
            let xml_value = encode_xml(ability_array[i][8]);
            xml_format = xml_format.replace("%LBEFORE%", xml_value.as_str());
            // Encode notes
            let xml_value = encode_xml(ability_array[i][9]);
            xml_format = xml_format.replace("%LAFTER%", xml_value.as_str());
            // Push onto array
            list_of_xml.push(xml_value);
        }
    }
    // Setup return buffer
    let return_string = format!("{encoding_xml}{player_class_xml}");
    let reduced_array = list_of_xml.iter().fold(String::new(), |str_1, str_2| format!("{str_1}{str_2}"));

    format!("{return_string}{reduced_array}{player_end_class_xml}")
}
/// Load all rotations from xml file into a vector
pub unsafe fn load_rotations_from_xml(xml_file: PathBuf) -> Vec<Vec<String>> {
    let mut return_vec: Vec<Vec<String>> = vec![vec![String::default(); 5]; PROFILE_SIZE];
    let mut xml_text_reader: Option<IXmlReader> = None;

    // Initialise our xml file
    let str_path = PCSTR::from_raw(xml_file.to_str().unwrap().as_bytes().as_ptr());
    println!("{:#?}", str_path.to_string().unwrap().as_str());
    let file_stream = SHCreateStreamOnFileA(
        str_path,
        STGM_READ.0
    ).unwrap();
    // Create Xml reader
   let _ = CreateXmlReader(
        &IXmlReader::IID,
        &mut xml_text_reader as *mut _ as *mut _,
        None
    ).unwrap();
    let xml_text_reader = xml_text_reader.unwrap();
    xml_text_reader.SetProperty(XmlReaderProperty_DtdProcessing.0 as _, DtdProcessing_Prohibit.0 as _).unwrap();
    xml_text_reader.SetInput(&file_stream).unwrap();

    // Store reads
    let mut element_string = w!("");
    let mut element_value = w!("");
    // Store xml input strings
    let mut rotation_name = w!("");
    let mut rotation_default = w!("");
    let mut rotation_list = w!("");
    let mut require_combat = w!("");
    let mut rotation_notes = w!("");

    // Loop through all nodes
    let mut node_type = XmlNodeType_None;
    while xml_text_reader.Read(Some(&mut node_type)).is_ok()
    {
        match node_type {
            XmlNodeType_Element => {
                xml_text_reader.GetLocalName(&mut element_string, None).unwrap();
            },
            XmlNodeType_Attribute => { },
            XmlNodeType_Text => {
                xml_text_reader.GetValue(&mut element_value, None).unwrap();
                if element_string.to_string().unwrap() == "RotationName".to_string() {
                    rotation_name = element_value;
                }
                else if element_string.to_string().unwrap() == "RotationDefault" {
                    rotation_default = element_value;
                }
                else if element_string.to_string().unwrap() == "RotationList" {
                    rotation_list = element_value;
                }
                else if element_string.to_string().unwrap() == "RequireCombat" {
                    require_combat = element_value;
                }
                else if element_string.to_string().unwrap() == "RotationNotes" {
                    rotation_notes = element_value;
                }
            },
            XmlNodeType_EndElement => {
                xml_text_reader.GetLocalName(&mut element_value, None).unwrap();
                if element_value.to_string().unwrap() == "Rotation" && !rotation_name.to_string().unwrap().is_empty() && !rotation_default.to_string().unwrap().is_empty() {
                    for i in 0..PROFILE_SIZE {
                        if return_vec[i][0].is_empty() {
                            return_vec[i][0] = decode_xml(rotation_name.to_string().unwrap().as_str()) + " (SNPSubt)";
                            return_vec[i][1] = decode_xml(rotation_default.to_string().unwrap().as_str());
                            return_vec[i][2] = decode_xml(rotation_list.to_string().unwrap().as_str());
                            return_vec[i][2] = return_vec[i][2].replace("|", " (SNPSubt)|");
                            return_vec[i][2] = return_vec[i][2].clone() + " (SNPSubt)";
                            return_vec[i][3] = decode_xml(require_combat.to_string().unwrap().as_str());
                            return_vec[i][4] = decode_xml(rotation_notes.to_string().unwrap().as_str());
                            rotation_name = w!("");
                            rotation_list = w!("");
                            rotation_default = w!("");
                            require_combat = w!("true");
                            rotation_notes = w!("");

                            break;
                        }
                    }
                }
            },
            other => { if node_type.0 == 0 { break } }
        }
    }
    return_vec
}
/// Load all abilities from xml file into a vector
pub unsafe fn load_abilities_from_xml(xml_file: PathBuf) -> Vec<Vec<String>> {
    let mut return_vec: Vec<Vec<String>> = vec![vec![String::default(); 10]; PROFILE_SIZE];
    let mut xml_text_reader: Option<IXmlReader> = None;

    // Initialise our xml file
    let str_path = PCSTR::from_raw(xml_file.to_str().unwrap().as_bytes().as_ptr());
    let file_stream = SHCreateStreamOnFileA(
        str_path,
        STGM_READ.0
    ).unwrap();
    // Create Xml reader
    let _ = CreateXmlReader(
        &IXmlReader::IID,
        &mut xml_text_reader as *mut _ as *mut _,
        None
    ).unwrap();
    let xml_text_reader = xml_text_reader.unwrap();
    xml_text_reader.SetProperty(XmlReaderProperty_DtdProcessing.0 as _, DtdProcessing_Prohibit.0 as _).unwrap();
    xml_text_reader.SetInput(&file_stream).unwrap();

    // Store reads
    let mut element_string = w!("");
    let mut element_value = w!("");
    // Store xml input strings
    let mut ability_name = w!("");
    let mut ability_default = w!("");
    let mut ability_spellid = w!("");
    let mut ability_actions = w!("");
    let mut ability_lua = w!("");
    let mut ability_lua_before = w!("");
    let mut ability_lua_after = w!("");
    let mut ability_recast_delay = w!("");
    let mut ability_self_cast = w!("");
    let mut ability_target = w!("target");
    let mut ability_cancel_channel = w!("false");

    // Loop through all nodes
    let mut node_type = XmlNodeType_None;
    while xml_text_reader.Read(Some(&mut node_type)).is_ok()
    {
        match node_type {
            XmlNodeType_Element => {
                xml_text_reader.GetLocalName(&mut element_string, None).unwrap();
            },
            XmlNodeType_Attribute => { },
            XmlNodeType_Text => {
                xml_text_reader.GetValue(&mut element_value, None).unwrap();
                if element_string.to_string().unwrap() == "Name".to_string() {
                    ability_name = element_value;
                }
                else if element_string.to_string().unwrap() == "Default".to_string() {
                    ability_default = element_value;
                }
                else if element_string.to_string().unwrap() == "SpellID".to_string() {
                    ability_spellid = element_value;
                }
                else if element_string.to_string().unwrap() == "Actions".to_string() {
                    ability_actions = element_value;
                }
                else if element_string.to_string().unwrap() == "Lua".to_string() {
                    ability_lua = element_value;
                }
                else if element_string.to_string().unwrap() == "LuaBefore".to_string() {
                    ability_lua_before = element_value;
                }
                else if element_string.to_string().unwrap() == "LuaAfter".to_string() {
                    ability_lua_after = element_value;
                }
                else if element_string.to_string().unwrap() == "RecastDelay".to_string() {
                    ability_recast_delay = element_value;
                }
                else if element_string.to_string().unwrap() == "SelfCast".to_string() {
                    ability_self_cast = element_value;
                    if ability_self_cast.to_string().unwrap() == "True".to_string() {
                        ability_self_cast = w!("player");
                    }
                    else {
                        ability_self_cast = w!("target");
                    }
                }
                else if element_string.to_string().unwrap() == "Target".to_string() {
                    ability_target = element_value;
                }
                else if element_string.to_string().unwrap() == "CancelChannel".to_string() {
                    ability_cancel_channel = element_value;
                }
            },
            XmlNodeType_EndElement => {
                xml_text_reader.GetLocalName(&mut element_value, None).unwrap();
                if element_value.to_string().unwrap() == "Ability".to_string() && !ability_default.to_string().unwrap().is_empty() && !ability_spellid.to_string().unwrap().is_empty() && !ability_lua.to_string().unwrap().is_empty() {
                    for i in 0..PROFILE_SIZE {
                        if return_vec[i][0].is_empty() {
                            return_vec[i][0] = decode_xml(ability_name.to_string().unwrap().as_str()) + " (SNPSubt)";
                            return_vec[i][1] = decode_xml(ability_default.to_string().unwrap().as_str());
                            return_vec[i][2] = decode_xml(ability_spellid.to_string().unwrap().as_str());
                            return_vec[i][3] = decode_xml(ability_actions.to_string().unwrap().as_str());
                            return_vec[i][4] = decode_xml(ability_lua.to_string().unwrap().as_str());
                            return_vec[i][5] = decode_xml(ability_recast_delay.to_string().unwrap().as_str());
                            return_vec[i][6] = decode_xml(ability_self_cast.to_string().unwrap().to_lowercase().as_str());
                            return_vec[i][7] = decode_xml(ability_cancel_channel.to_string().unwrap().to_lowercase().as_str());
                            return_vec[i][8] = decode_xml(ability_lua_before.to_string().unwrap().as_str());
                            return_vec[i][9] = decode_xml(ability_lua_after.to_string().unwrap().as_str());
                            ability_name = w!("");
                            ability_default = w!("");
                            ability_spellid = w!("");
                            ability_actions = w!("");
                            ability_lua = w!("");
                            ability_recast_delay = w!("0");
                            ability_self_cast = w!("target");
                            ability_cancel_channel = w!("false");
                            break;
                        }
                    }
                }
            },
            other => { if node_type.0 == 0 { break } }
        }
    }
    return_vec
}
/// Extract abilities from rotation file
pub fn extract_abilities_from_rotation(player_rotation: Vec<String>) -> Vec<String> {
    player_rotation[2]
        .as_str()
        .split("|")
        .map(|s| s.to_string())
        .collect()
}