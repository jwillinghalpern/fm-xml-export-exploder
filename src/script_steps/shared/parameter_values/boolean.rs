use super::Error;
use crate::utils::attributes::{try_get_attribute, try_get_attribute_cow};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

#[derive(Debug, PartialEq)]
pub struct Boolean {
    pub kind: Kind,
    pub value: bool,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Select,
}

impl Boolean {
    pub fn from_xml(reader: &mut Reader<&[u8]>) -> Result<Self, Error> {
        let mut buf: Vec<u8> = Vec::new();
        let mut value = false;
        let mut opt_kind = None;

        loop {
            let event = reader.read_event_into(&mut buf)?;
            match event {
                Event::Start(e) if e.name().as_ref() == b"Boolean" => {
                    value = try_get_attribute(&e, "value")? == "True";
                    opt_kind = Some(Kind::from_bytes_start(&e)?);
                }
                Event::End(e) if e.name().as_ref() == b"Boolean" => break,
                Event::Eof => return Err("unexpected end of file".into()),
                _ => {}
            }
            buf.clear();
        }

        let kind = opt_kind.ok_or("missing boolean kind")?;
        Ok(Boolean { kind, value })
    }
}

impl Kind {
    fn from_bytes_start(e: &BytesStart) -> Result<Self, Error> {
        let id = try_get_attribute_cow(e, "id")?;
        match id.as_ref() {
            b"4096" => Ok(Kind::Select),
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
				<Boolean type="Select" id="4096" value="True"></Boolean>
			</Parameter>
		"#;
        test_boolean(
            xml,
            Boolean {
                kind: Kind::Select,
                value: true,
            },
        );
    }

    #[test]
    fn test_select_raw() {
        let xml = r#"<Boolean type="Select" id="4096" value="True">
				</Boolean>"#;
        test_boolean(
            xml,
            Boolean {
                kind: Kind::Select,
                value: true,
            },
        );
    }

    #[test]
    fn test_false() {
        let xml = r#"<Boolean type="Select" id="4096" value="False"></Boolean>"#;
        test_boolean(
            xml,
            Boolean {
                kind: Kind::Select,
                value: false,
            },
        );
    }
}
