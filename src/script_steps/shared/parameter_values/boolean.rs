use super::Error;
use crate::utils::attributes::{try_get_attribute, try_get_attribute_cow};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub kind: Kind,
    pub value: bool,
    pub label: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Select,
    VerifySslCertificates,
    WithDialog,
}

impl Boolean {
    pub fn new(kind: Kind, value: bool, label: impl ToString) -> Self {
        Boolean {
            kind,
            value,
            label: label.to_string(),
        }
    }

    pub fn from_xml(reader: &mut Reader<&[u8]>) -> Result<Self, Error> {
        let mut buf: Vec<u8> = Vec::new();
        let mut value = false;
        let mut opt_kind = None;
        let mut label = String::new();

        loop {
            let event = reader.read_event_into(&mut buf)?;
            match event {
                Event::Start(e) if e.name().as_ref() == b"Boolean" => {
                    value = try_get_attribute(&e, "value")? == "True";
                    opt_kind = Some(Kind::from_bytes_start(&e)?);
                    label = try_get_attribute(&e, "type").unwrap_or("Select".to_string());
                }
                Event::End(e) if e.name().as_ref() == b"Boolean" => break,
                Event::Eof => return Err("unexpected end of file".into()),
                _ => {}
            }
            buf.clear();
        }

        let kind = opt_kind.ok_or("missing boolean kind")?;
        Ok(Boolean::new(kind, value, label))
    }
}

impl Kind {
    fn from_bytes_start(e: &BytesStart) -> Result<Self, Error> {
        let id = try_get_attribute_cow(e, "id")?;
        match id.as_ref() {
            b"4096" => Ok(Kind::Select),
            b"268435456" => Ok(Kind::VerifySslCertificates),
            b"128" => Ok(Kind::WithDialog),
            other => Err(format!(
                "unknown boolean type: {}",
                std::str::from_utf8(other).unwrap()
            )
            .into()),
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    fn test_boolean(xml: &str, expected: Boolean) {
        let mut reader = Reader::from_str(xml);
        let boolean = Some(Boolean::from_xml(&mut reader).unwrap());

        assert_eq!(boolean, Some(expected));
    }

    #[test]
    fn test_select_parameter() {
        let xml = r#"
			<Parameter type="Boolean">
				<Boolean type="Auswahl" id="4096" value="True"></Boolean>
			</Parameter>
		"#;
        test_boolean(xml, Boolean::new(Kind::Select, true, "Auswahl"));
    }

    #[test]
    fn test_select_raw() {
        let xml = r#"<Boolean type="Select Other Label" id="4096" value="True">
				</Boolean>"#;
        test_boolean(xml, Boolean::new(Kind::Select, true, "Select Other Label"));
    }

    #[test]
    fn test_false() {
        let xml = r#"<Boolean type="Select" id="4096" value="False"></Boolean>"#;
        test_boolean(xml, Boolean::new(Kind::Select, false, "Select"));
    }
}
