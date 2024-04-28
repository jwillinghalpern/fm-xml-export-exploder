use anyhow::{anyhow, Result};
use quick_xml::events::BytesStart;
use quick_xml::name::QName;
use std::borrow::Cow;

pub fn key_to_string(key: QName) -> String {
    return String::from_utf8_lossy(key.as_ref()).to_string();
}

pub fn value_to_string(value: Cow<[u8]>) -> String {
    return String::from_utf8_lossy(value.as_ref()).to_string();
}

pub fn get_attributes(e: &BytesStart) -> Option<Vec<(String, String)>> {
    let mut attributes = Vec::new();

    for attr in e.attributes() {
        let attr = match attr {
            Ok(attribute) => attribute,
            Err(_) => continue,
        };

        let key = key_to_string(attr.key);
        let value = value_to_string(attr.value);
        attributes.push((key, value));
    }

    Some(attributes)
}

pub fn get_attribute(e: &BytesStart, name: &str) -> Option<String> {
    for attr in get_attributes(e).unwrap() {
        if attr.0 != name {
            continue;
        }

        return Some(attr.1);
    }

    None
}

pub fn try_get_attribute(e: &BytesStart, name: &str) -> Result<String> {
    match get_attribute(e, name) {
        Some(value) => Ok(value),
        None => Err(anyhow!("missing attribute: {}", name)),
    }
}

pub fn try_get_attribute_cow<'a>(e: &'a BytesStart, name: &str) -> Result<Cow<'a, [u8]>> {
    for attr in e.attributes() {
        let Ok(attr) = attr else {
            continue;
        };
        if attr.key.as_ref() == name.as_bytes() {
            return Ok(attr.value);
        }
    }
    Err(anyhow!("missing attribute: {}", name))
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use quick_xml::events::Event;
    use quick_xml::Reader;

    #[test]
    fn test_get_name_attribute() {
        let xml_tag = "<Name value=\"$Serverscript\">";
        let mut reader = Reader::from_str(xml_tag);
        reader.trim_text(true);

        if let Ok(Event::Start(ref e)) = reader.read_event() {
            assert_eq!(
                utils::attributes::get_attribute(e, "value").unwrap(),
                "$Serverscript"
            );
        }
    }

    #[test]
    fn test_try_get_attribute_cow() {
        let xml = r#"<Name value="$Serverscript" id="4096"></Name>"#;
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        if let Ok(Event::Start(ref e)) = reader.read_event() {
            let id = utils::attributes::try_get_attribute_cow(e, "id").unwrap();
            assert_eq!(id, "4096".as_bytes());
        }
    }
}
