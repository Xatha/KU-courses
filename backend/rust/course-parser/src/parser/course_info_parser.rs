// File for the course info side-table
use crate::parser;
use tl::VDom;

pub fn parse_course_info(dom: &VDom) -> Result<parser::CourseInformation, Box<dyn std::error::Error>> {
    // find all div class="panel-body" elements and assert that there is only one
    let panel_bodies = dom.get_elements_by_class_name("panel-body");
    let parser = dom.parser();

    // there might be multiple panel-bodies, so we need to check each one
    // for the dl element (only the course info should have a dl element)
    for (_i, panel_body) in panel_bodies.enumerate() {
        let mut dl_elements = panel_body
            .get(parser)
            .ok_or("Failed to get panel-body")?
            .as_tag()
            .ok_or("Failed to get panel-body as tag")?
            .query_selector(parser, "dl")
            .ok_or("Failed to get dl from panel-body")?;
        match dl_elements.next() {
            Some(handle) => {
                let node = handle
                    .get(parser)
                    .ok_or("Failed to get node")?
                    .as_tag()
                    .ok_or("Failed to get node as tag")?;
                // parse DL
                let course_infos = parse_dl(node, parser)?;
                // println!("{course_infos:?}");
                // parse the course information
                let coerced_course_info = coerce_course_info(course_infos);
                return coerced_course_info;
            }
            None => continue,
        }
    }
    Err("No dl element found in the panel-body".into())
}

// return a list of tuples of (key, value)
fn parse_dl(
    dl_tag: &tl::HTMLTag,
    parser: &tl::Parser,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(String, String)> = Vec::new();
    let children = dl_tag.children();
    // for even numbers, we expect a dt element, odd numbers we expect a dd element
    // make a pair of precisely two strings
    let mut pair: Vec<String> = Vec::with_capacity(2);
    for (_i, child) in children.top().iter().enumerate() {
        let node = child
            .get(parser)
            .ok_or("Failed to get node whilst parsing dl")?;
        match node.as_tag() {
            Some(tag) => {
                if tag.name() == "dt" {
                    pair.push(node.inner_text(parser).to_string());
                } else if tag.name() == "dd" {
                    pair.push(node.inner_text(parser).to_string());
                    if pair.len() == 2 {
                        result.push((pair[0].clone(), pair[1].clone()));
                        pair.clear();
                    }
                } else {
                    return Err("Expected dt or dd element".into());
                }
            }
            None => continue,
        }
    }
    // if the pair is not empty then we have had an odd number of elements and should error
    if !pair.is_empty() {
        return Err("Odd number of elements in dl".into());
    }
    Ok(result)
}

fn parse_language(language: &str) -> Result<Vec<parser::Language>, Box<dyn std::error::Error>> {
    // println!("parser::Language information passed in: {language}");

    let mut languages: Vec<parser::Language> = Vec::new();

    if language.contains("Danish") | language.contains("Dansk") {
        languages.push(parser::Language::Danish);
    }

    if language.contains("English") | language.contains("Engelsk") {
        languages.push(parser::Language::English);
    }

    if languages.len() > 0 {
        Ok(languages)
    } else {
        Err("Unable to parse languages".into())
    }
}

fn parse_ects(ects: &str) -> Result<f32, Box<dyn std::error::Error>> {
    println!("Ects info: {}", ects); // Fixed formatting string

    // Extract numeric characters, '.' and ',' from the input string
    let ects_info = ects
        .chars()
        .take_while(|c| c.is_numeric() || *c == '.' || *c == ',')
        .collect::<String>();

    // Replace ',' with '.' to ensure correct parsing
    let ects_info = ects_info.replace(',', ".");

    // Parse the string to a f32
    let ects_value = ects_info.parse::<f32>()?;

    Ok(ects_value)
}

#[allow(dead_code)]
fn parse_degree(degree: &str) -> Result<Vec<parser::Degree>, Box<dyn std::error::Error>> {
    // println!("parser::Degree information: {degree}");
    let mut result: Vec<parser::Degree> = Vec::new();
    // Loop through the degree string and find all the degrees
    // Look for either "Master", "Bachelor", "Kandidat" or "Ph.d."

    // loop through a 2 character sliding window and deal with the fact that they might not be alphabetic
    const WINDOW_LENGTH: usize = 4;
    for i in 0..degree.len() - WINDOW_LENGTH - 1 {
        let sliding_window = &degree.to_lowercase()[i..i + WINDOW_LENGTH];
        match sliding_window {
            "bach" => result.push(parser::Degree::Bachelor),
            "mast" | "kand" => result.push(parser::Degree::Master),
            "ph.d" => result.push(parser::Degree::Phd),
            _ => continue,
        }
    }

    // Sort and deduplicate
    result.sort();
    result.dedup();
    if result.is_empty() {
        return Err("No degree found".into());
    }
    Ok(result)
}

fn parse_capacity(capacity: &str) -> parser::Capacity {
    println!("parser::Capacity information passed in: {capacity}");

    // find the first number and parse it
    parser::Capacity(
        capacity
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .ok(),
    )
}

fn parse_schedule(schedule: &str) -> Result<Vec<parser::Schedule>, Box<dyn std::error::Error>> {
    // println!("parser::Schedule info passed in: {schedule}");
    let mut schedule_vec: Vec<parser::Schedule> = Vec::new();

    if schedule.contains("A") {
        schedule_vec.push(parser::Schedule::A);
    }

    if schedule.contains("B") {
        schedule_vec.push(parser::Schedule::B);
    }

    if schedule.contains("C") {
        schedule_vec.push(parser::Schedule::C);
    }

    if schedule.contains("D") {
        schedule_vec.push(parser::Schedule::D);
    }

    if schedule_vec.len() > 0 {
        Ok(schedule_vec)
    } else {
        Err("Unknown schedule".into())
    }
}

fn parse_block(input: &str) -> Result<Vec<parser::Block>, Box<dyn std::error::Error>> {
    // println!("{input}");
    let mut blocks: Vec<parser::Block> = Vec::new();

    for c in input.chars() {
        match c {
            '1' => blocks.push(parser::Block::One),
            '2' => blocks.push(parser::Block::Two),
            '3' => blocks.push(parser::Block::Three),
            '4' => blocks.push(parser::Block::Four),
            '5' => blocks.push(parser::Block::Five),
            _ => (),
        }
    }

    if blocks.len() > 0 {
        Ok(blocks)
    } else {
        Err("Unknown block".into())
    }
}

fn parse_duration(duration: &str) -> Result<parser::Duration, Box<dyn std::error::Error>> {
    // either 1 blo(c)k, 2 blo(c)ks or 1 semester
    // grab the first 3 chars
    let chopped_duration = duration.chars().take(3).collect::<String>();
    match chopped_duration.as_str() {
        "1 b" => Ok(parser::Duration::One),
        "2 b" | "1 s" => Ok(parser::Duration::Two),
        _ => Err("Unknown duration".into()),
    }
}

fn coerce_course_info(
    course_info: Vec<(String, String)>,
) -> Result<parser::CourseInformation, Box<dyn std::error::Error>> {
    // dbg!(&course_info);
    let mut id: Option<String> = None;
    let mut ects: Option<f32> = None;
    let mut block: Option<Vec<parser::Block>> = None;
    let mut schedule: Option<Vec<parser::Schedule>> = None;
    let mut language: Option<Vec<parser::Language>> = None;
    let mut duration: Option<parser::Duration> = None;
    let mut degree: Option<Vec<parser::Degree>> = None;
    let mut capacity: parser::Capacity = parser::Capacity(None);

    for (key, value) in &course_info {
        // first iterate through only to find the block, since  this will tell us if we
        // are dealing with the faculty of science (they use blocks) or the other faculties
        // Check the first 5 chars of the key to see if it is "Place"
        let chopped_key = key.chars().take(5).collect::<String>();
        if chopped_key == "Place" {
            block = Some(parse_block(value)?);
        }
    }

    for (key, value) in course_info {
        match key.as_str() {
            "Language" | "Sprog" => language = Some(parse_language(&value)?),
            "Course code" | "Kursuskode" => id = Some(value), // "Kursuskode" is the danish version of "Course code
            "Point" | "Credit" => ects = Some(parse_ects(&value)?), // "Point" is the danish version of "Credit"
            "Level" | "Niveau" => degree = Some(parse_degree(&value)?),
            "Duration" | "Varighed" => duration = Some(parse_duration(&value)?),
            "Schedule" | "Skemagruppe" => schedule = Some(parse_schedule(&value)?),
            "Course capacity" | "Kursuskapacitet" => capacity = parse_capacity(&value),
            _ => continue,
        }
    }
    // print every error with the contents of the course_info
    let id = id.ok_or("Failed to get id")?;
    let ects = ects.ok_or("Failed to get ects")?;
    let block = block.ok_or("Failed to get block")?;
    let schedule = schedule.ok_or("Failed to get schedule")?;
    let language = language.ok_or("Failed to get language")?;
    let duration = duration.ok_or("Failed to get duration")?;
    let degree = degree.ok_or("Failed to get degree")?;
    println!("{id}: {degree:?}");

    Ok(parser::CourseInformation {
        id,
        ects,
        block,
        schedule,
        language,
        duration,
        degree,
        capacity,
    })
}
