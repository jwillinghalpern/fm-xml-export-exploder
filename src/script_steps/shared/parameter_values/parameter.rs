use quick_xml::{
    escape::unescape,
    events::{BytesStart, Event},
    Reader,
};

// TODO: should we `mod boolean` in this file instad?
use super::{boolean::Boolean, Result};

pub use crate::script_steps::shared::parameter_values::target::Target;

use crate::{
    calculations::calculation::Calculation,
    utils::attributes::{get_attribute, try_get_attribute},
};

#[derive(Debug, PartialEq)]
pub enum Parameter {
    Boolean(Boolean),
    Target(Target),
    Text(String),
    Calculation(String),
}

impl Parameter {
    pub fn from_xml(reader: &mut Reader<&[u8]>, e: &BytesStart) -> Result<Self> {
        let parameter_type = get_attribute(e, "type").ok_or("missing attribute: type")?;
        match parameter_type.as_str() {
            "Boolean" => Ok(Parameter::Boolean(Boolean::from_xml(reader)?)),
            "Text" => Ok(Parameter::Text(parse_text(reader)?)),
            "Target" => Ok(Parameter::Target(Target::from_xml(reader)?)),
            "Calculation" => {
                let calculation = Calculation::from_xml(reader, e)?;
                Ok(Parameter::Calculation(calculation))
            }
            _ => Err(format!("unknown parameter type: {}", parameter_type).into()),
        }
    }
}

fn parse_text(reader: &mut Reader<&[u8]>) -> Result<String> {
    let mut buf = Vec::new();
    let mut text = String::new();

    loop {
        let Ok(event) = reader.read_event_into(&mut buf) else {
            break;
        };
        match event {
            Event::Start(e) => {
                if e.name().as_ref() == b"Text" {
                    let val = try_get_attribute(&e, "value").unwrap_or_default();
                    text = match unescape(&val) {
                        Ok(val) => val.to_string(),
                        Err(_) => val,
                    };
                }
            }
            Event::End(e) if e.name().as_ref() == b"Text" => break,
            Event::Eof => return Err("unexpected end of file".into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(text)
}

// #############################################################################
// TESTS
// #############################################################################
#[cfg(test)]
mod test {
    use crate::script_steps::shared::parameter_values::{
        boolean::Kind,
        target::{FieldReference, TableOccurrenceReference, Variable},
    };

    use super::*;

    #[test]
    fn test_boolean() {
        let xml = r#"
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="True"></Boolean>
					</Parameter>
				"#;

        let mut parameter = None;
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        reader.trim_text(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"Parameter" => {
                        parameter = Some(Parameter::from_xml(&mut reader, &e).unwrap());
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        assert_eq!(
            parameter,
            Some(Parameter::Boolean(Boolean {
                kind: Kind::Select,
                value: true,
            }))
        );
    }

    #[test]
    fn test_text() {
        let xml = r#"
					<Parameter type="Text">
						<Text value="&quot;hello&quot;a&#13;b&#13;c&#10;lf"></Text>
					</Parameter>
				"#;

        let mut parameter = None;
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        reader.trim_text(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"Parameter" => {
                        parameter = Some(Parameter::from_xml(&mut reader, &e).unwrap());
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        assert_eq!(
            parameter,
            Some(Parameter::Text("\"hello\"a\rb\rc\nlf".to_string()))
        );
    }

    #[test]
    fn test_field_reference_target() {
        let xml = r#"
				<Parameter type="Target">
					<FieldReference id="1" name="field_name" UUID="4FEADECE-195B-4BC7-83B7-57C5BBD4CD45">
						<repetition value="3"></repetition>
						<TableOccurrenceReference id="1065089" name="table_name" UUID="04AF7D77-38A6-4E99-B4B5-F27013E04589"></TableOccurrenceReference>
					</FieldReference>
				</Parameter>
			"#;

        let mut parameter = None;
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        reader.trim_text(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"Parameter" => {
                        parameter = Some(Parameter::from_xml(&mut reader, &e).unwrap());
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        assert_eq!(
            parameter,
            Some(Parameter::Target(Target::FieldReference(FieldReference {
                name: "field_name".to_string(),
                table_occurrence_reference: TableOccurrenceReference {
                    name: "table_name".to_string()
                },
                repetition: "3".to_string()
            })))
        );
    }

    #[test]
    fn test_varaible_target() {
        let xml = r#"
				<Parameter type="Target">
					<Variable value="$hello">
						<repetition value="4"></repetition>
					</Variable>
				</Parameter>
			"#;

        let mut parameter = None;
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        reader.trim_text(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"Parameter" => {
                        parameter = Some(Parameter::from_xml(&mut reader, &e).unwrap());
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        assert_eq!(
            parameter,
            Some(Parameter::Target(Target::Variable(Variable {
                name: "$hello".to_string(),
                repetition: "4".to_string()
            })))
        );
    }
}
