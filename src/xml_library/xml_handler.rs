use std::ffi::CString;
use std::path::PathBuf;
use windows::core::{ComInterface, PCSTR, w};
use windows::Win32::Data::Xml::XmlLite::{CreateXmlReader, IXmlReader, XmlReaderProperty_DtdProcessing, DtdProcessing_Prohibit, XmlNodeType_None, XmlNodeType_Element, XmlNodeType_Attribute, XmlNodeType_Text, XmlNodeType_EndElement};
use windows::Win32::Foundation::S_OK;
use windows::Win32::System::Com::STGM_READ;
use windows::Win32::UI::Shell::SHCreateStreamOnFileA;

pub const PROFILE_SIZE: usize = 1024;
// Decode xml input
pub fn decode_xml(xml_input: String) -> String {
    xml_input
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;","&")
        .replace("&apos;","'")
        .replace("&quot;", "\"")

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
    //let str_path = PCSTR::from_raw(xml_file.to_str().unwrap().as_bytes().as_ptr());
    let str_path = CString::new(xml_file.to_str().unwrap()).unwrap();
    println!("{:#?}", str_path.to_str().unwrap());
    let file_stream = SHCreateStreamOnFileA(
        PCSTR::from_raw(str_path.as_bytes().as_ptr()),
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
    let mut w_element_string = w!("");
    let mut element_string= String::default();
    let mut w_element_string_len = 0;
    let mut w_element_value = w!("");
    let mut element_value = String::default();
    let mut w_element_value_len = 0;
    // Store xml input strings
    let mut rotation_name = String::default();
    let mut rotation_default = String::default();
    let mut rotation_list = String::default();
    let mut require_combat = String::default();
    let mut rotation_notes = String::default();

    // Loop through all nodes
    let mut node_type = XmlNodeType_None;
    // Get abbreviated profile name
    let abbreviated_profile_name = xml_file.to_str().unwrap().to_string().rsplit('\\').next().unwrap_or("").split('_').next().unwrap_or("").to_string();
    while xml_text_reader.Read(Some(&mut node_type)).0 == S_OK.0
    {
        match node_type {
            XmlNodeType_Element => {
                xml_text_reader.GetLocalName(&mut w_element_string, Some(&mut w_element_string_len)).unwrap();
                element_string = String::from_utf16_lossy(std::slice::from_raw_parts(w_element_string.0, w_element_string_len as usize));
            },
            XmlNodeType_Attribute => { },
            XmlNodeType_Text => {
                xml_text_reader.GetValue(&mut w_element_value, Some(&mut w_element_value_len)).unwrap();
                element_value = String::from_utf16_lossy(std::slice::from_raw_parts(w_element_value.0, w_element_value_len as usize));
                match element_string.to_string().as_str() {
                    "RotationName" => rotation_name = element_value,
                    "RotationDefault" => rotation_default = element_value,
                    "RotationList" => rotation_list = element_value,
                    "RequireCombat" => require_combat = element_value,
                    "RotationNotes" => rotation_notes = element_value,
                    _ => {},
                }
            },
            XmlNodeType_EndElement => {
                xml_text_reader.GetLocalName(&mut w_element_value, Some(&mut w_element_value_len)).unwrap();
                element_value = String::from_utf16_lossy(std::slice::from_raw_parts(w_element_value.0, w_element_value_len as usize));
                if element_value == "Rotation".to_string() && !rotation_name.is_empty() && !rotation_default.is_empty() {
                    for i in 0..PROFILE_SIZE {
                        if return_vec[i][0].is_empty() {
                            return_vec[i][0] = decode_xml(rotation_name.clone()) + format!(" ({abbreviated_profile_name})").as_str();
                            return_vec[i][1] = decode_xml(rotation_default.clone());
                            return_vec[i][2] = decode_xml(rotation_list.clone());
                            return_vec[i][2] = return_vec[i][2].replace("|", format!(" ({abbreviated_profile_name})|").as_str());
                            return_vec[i][2] = return_vec[i][2].clone() + format!(" ({abbreviated_profile_name})").as_str();
                            return_vec[i][3] = decode_xml(require_combat.clone());
                            return_vec[i][4] = decode_xml(rotation_notes.clone());
                            rotation_name = String::default();
                            rotation_list = String::default();
                            rotation_default = String::default();
                            require_combat = "true".to_string();
                            rotation_notes = "".to_string();

                            break;
                        }
                    }
                }
            },
            _ => { }
        }
    }
    return_vec
}
/// Load all abilities from xml file into a vector
pub unsafe fn load_abilities_from_xml(xml_file: PathBuf) -> Vec<Vec<String>> {
    let mut return_vec: Vec<Vec<String>> = vec![vec![String::default(); 10]; PROFILE_SIZE];
    let mut xml_text_reader: Option<IXmlReader> = None;
    // Initialise our xml file
    //let str_path = PCSTR::from_raw(xml_file.to_str().unwrap().as_bytes().as_ptr());
    let str_path = CString::new(xml_file.to_str().unwrap()).unwrap();
    println!("{:#?}", str_path.to_str().unwrap());
    let file_stream = SHCreateStreamOnFileA(
        PCSTR::from_raw(str_path.as_bytes().as_ptr()),
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
    let mut w_element_string = w!("");
    let mut element_string= String::default();
    let mut w_element_string_len = 0;
    let mut w_element_value = w!("");
    let mut element_value = String::default();
    let mut w_element_value_len = 0;
    // Store xml input strings
    let mut ability_name = String::default();
    let mut ability_default = String::default();
    let mut ability_spellid = String::default();
    let mut ability_actions = String::default();
    let mut ability_lua = String::default();
    let mut ability_lua_before = String::default();
    let mut ability_lua_after = String::default();
    let mut ability_recast_delay = String::default();
    let mut ability_self_cast = "Target".to_string();
    let mut ability_cancel_channel = "False".to_string();

    // Loop through all nodes
    let mut node_type = XmlNodeType_None;
    // Get abbreviated profile name
    let abbreviated_profile_name = xml_file.to_str().unwrap().to_string().rsplit('\\').next().unwrap_or("").split('_').next().unwrap_or("").to_string();
    while xml_text_reader.Read(Some(&mut node_type)).0 == S_OK.0
    {
        match node_type {
            XmlNodeType_Element => {
                xml_text_reader.GetLocalName(&mut w_element_string, Some(&mut w_element_string_len)).unwrap();
                element_string = String::from_utf16_lossy(std::slice::from_raw_parts(w_element_string.0, w_element_string_len as usize));
            },
            XmlNodeType_Attribute => { },
            XmlNodeType_Text => {
                xml_text_reader.GetValue(&mut w_element_value, Some(&mut w_element_value_len)).unwrap();
                element_value = String::from_utf16_lossy(std::slice::from_raw_parts(w_element_value.0, w_element_value_len as usize));
                match element_string.to_string().as_str() {
                    "Name" => ability_name = element_value,
                    "Default" => ability_default = element_value,
                    "SpellID" => ability_spellid = element_value,
                    "Actions" => ability_actions = element_value,
                    "Lua" => ability_lua = element_value,
                    "LuaBefore" => ability_lua_before = element_value,
                    "LuaAfter" => ability_lua_after = element_value,
                    "RecastDelay" => ability_recast_delay = element_value,
                    "SelfCast" => {
                        ability_self_cast = element_value;
                        if ability_self_cast.to_string() == "True".to_string() {
                            ability_self_cast = "player".to_string();
                        }
                        else {
                            ability_self_cast = "target".to_string();
                        }
                    },
                    "Target" => ability_self_cast = element_value,
                    "CancelChannel" => ability_cancel_channel = element_value,
                    _ => {}
                };
            },
            XmlNodeType_EndElement => {
                xml_text_reader.GetLocalName(&mut w_element_value, Some(&mut w_element_value_len)).unwrap();
                element_value = String::from_utf16_lossy(std::slice::from_raw_parts(w_element_value.0, w_element_value_len as usize));
                if element_value == "Ability".to_string() &&
                    !ability_name.is_empty() &&
                    !ability_default.is_empty() &&
                    !ability_spellid.is_empty() &&
                    !ability_lua.is_empty() {
                    for i in 0..PROFILE_SIZE {
                        if return_vec[i][0].is_empty() {
                            return_vec[i][0] = decode_xml(ability_name) + format!(" ({abbreviated_profile_name})").as_str();
                            return_vec[i][1] = decode_xml(ability_default);
                            return_vec[i][2] = decode_xml(ability_spellid);
                            return_vec[i][3] = decode_xml(ability_actions);
                            return_vec[i][4] = decode_xml(ability_lua);
                            return_vec[i][5] = decode_xml(ability_recast_delay);
                            return_vec[i][6] = decode_xml(ability_self_cast.to_lowercase());
                            return_vec[i][7] = decode_xml(ability_cancel_channel.to_lowercase());
                            return_vec[i][8] = decode_xml(ability_lua_before.to_string());
                            return_vec[i][9] = decode_xml(ability_lua_after.to_string());
                            ability_name = String::default();
                            ability_default = String::default();
                            ability_spellid = String::default();
                            ability_actions = String::default();
                            ability_lua = String::default();
                            ability_recast_delay = "0".to_string();
                            ability_self_cast = "Target".to_string();
                            ability_cancel_channel = "False".to_string();
                            break;
                        }
                    }
                }
            },
            _ => { }
        }
    }
    return_vec
}
/// Extract the lua function from an ability passed in
pub fn extract_lua_from_ability(player_rotation: Vec<String>) -> String {
    player_rotation.get(4).unwrap().to_string()
}
/// Extract the ability name from ability file
pub fn extract_abilities_name_from_rotation(player_rotation: Vec<String>) -> String {
    player_rotation.get(0).unwrap().to_string()
}