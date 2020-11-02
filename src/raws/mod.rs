
rltk::embedded_resource!(RAW_FILE, "../../raws/spawns.json");

pub fn load_raws() {
    rltk::link_resource!(RAW_FILE, "../../raws/spawns.json");

//retrieve the raw data as an array of u8
let raw_data = rltk::embedding::EMBED
    .lock()
    .unwrap()
    .get_resource("../raws/spawns.json".to_string())
    .unwrap();
let raw_string = std::str::from_utf8(&raw_data).expect("Unable to convert to a valid utf-8 string.");

let decoder : Raws = serde_json::from_str(&raw_string).expect("Unable to parse JSON");
rltk::console::log(format!("{:?}", decoder));

}