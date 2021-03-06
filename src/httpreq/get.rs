use crate::httpreq;

use std::collections::HashMap;
use serde_json;

// get particiapnts data
pub fn get_participants(date_args: crate::GetArgs)-> Result<String, Box<dyn std::error::Error>> {

    // replace :year, :month, :day in yaml file
    let request_path: httpreq::RequestPath = Default::default();
    let path: String = request_path.get_path;
    let path = path.replace(":year", &date_args.year.to_string());
    let path = path.replace(":month", &date_args.month.to_string());
    let path = path.replace(":day", &date_args.day.to_string());

    let params = httpreq::Params {
        method: httpreq::HttpMethod::Get,
        path: path,
        ..Default::default()
    };
    let response_body = httpreq::fetch(params)?;

    let json_data: serde_json::Value = serde_json::from_str(response_body.as_str())?;

    // format
    let mut minutes_format_participants = String::new();
    // Create format string by grade
    for (grade, classmates) in json_data.as_object().ok_or("JSON is not a object")? {
        // skip if null like:
        //     "B4": null
        if classmates.is_null() {
            continue;
        }

        // push id to this Vec for sort
        let mut id_list = Vec::<String>::new();
        // participants string in this grade
        let mut minutes_format_grade_participant = String::from("");
        // id: name map to get name from sorted id
        let mut minutes_format_grade_participant_map = HashMap::<String, String>::new();

        // collect id, name to id_list, map
        for participant in classmates.as_array().ok_or("classmates in JSON is not array")? {
            let name = participant["Name"].as_str().ok_or("Name in JSON is not string")?.to_string();
            let id = participant["Id"].as_str().ok_or("id in JSON is not string")?.to_string();
            // :poop:
            // id_list.push(id);
            id_list.push((&id).to_string());
            // minutes_format_grade_participant_map.insert(id, name);
            minutes_format_grade_participant_map.insert((&id).to_string(), (&name).to_string());
        }
        // sort id
        id_list.sort_by(|a, b| a.cmp(b));
        // get name from sorted id
        for id in id_list {
            minutes_format_grade_participant.push_str(format!("{}???", minutes_format_grade_participant_map.get(&id).ok_or("could not find name from id")?).as_str());
        }
        // remove last "???"
        // and push to format string
        let minutes_format_grade_participant_last_idx = minutes_format_grade_participant.char_indices().nth_back(0).ok_or("could not get last char")?.0;
        minutes_format_participants += format!("??? {} {}\n", grade, &minutes_format_grade_participant[..minutes_format_grade_participant_last_idx]).as_str();
    }


    if minutes_format_participants.is_empty() {
        Ok(String::from(""))
    } else {
        Ok(minutes_format_participants[..minutes_format_participants.len() - 1].to_string())
    }
}
