// Decode xml input
pub fn decode_xml(xml_input: &str) -> &str {
    let mut decoded_xml = xml_input.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;","'").replace("&amp","&");
    decoded_xml
}
// Encode xml input
pub fn encode_xml(xml_input: &str) -> &str {
    let mut encoded_xml = xml_input.replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&apos;").replace("&", "&amp;");
    encoded_xml
}
