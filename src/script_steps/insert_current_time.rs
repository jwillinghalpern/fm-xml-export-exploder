use crate::script_steps::shared::parameter_values::boolean::Kind;
use crate::script_steps::shared::ParameterValues;
use crate::utils::attributes::get_attribute;

use quick_xml::events::Event;
use quick_xml::Reader;

pub fn sanitize(step: &str) -> Option<String> {
    let mut name = String::new();
    let mut select = None;
    let mut target: Option<crate::script_steps::shared::Target> = None;

    let mut reader = Reader::from_str(step);
    reader.trim_text(true);
    let mut buf: Vec<u8> = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(_) => continue,
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"Step" => {
                    name = get_attribute(&e, "name").unwrap_or_default();
                }
                b"ParameterValues" => {
                    let parameter_values = ParameterValues::from_xml(&mut reader).unwrap();
                    select = parameter_values.get_boolean(Kind::Select);
                    target = parameter_values.get_target();
                }
                _ => {}
            },
            _ => {}
        }
        buf.clear()
    }

    if name.is_empty() {
        None
    } else {
        let mut v = Vec::with_capacity(3);
        if let Some(select) = select {
            v.push(select.label);
        }
        if let Some(target) = target {
            v.push(format!("Target: {}", target));
        }
        let params = v.join(" ; ");
        if params.is_empty() {
            Some(format!("{} []", name))
        } else {
            Some(format!("{} [ {} ]", name, params))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let xml = r#"
			<Step index="5" id="14" name="Insert Current Time" enable="True">
				<UUID>960A8BC0-7C06-4E9B-AA8A-67F07707D5E8</UUID>
				<OwnerID></OwnerID>
				<Options>4097</Options>
				<ParameterValues membercount="2">
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="True"></Boolean>
					</Parameter>
					<Parameter type="Target">
						<FieldReference id="2" name="creationAccount" UUID="165A76D2-26D4-4B54-8E1F-2C0E798FB400">
							<repetition value="123"></repetition>
							<TableOccurrenceReference id="1065089" name="lkjflkjf" UUID="04AF7D77-38A6-4E99-B4B5-F27013E04589"></TableOccurrenceReference>
						</FieldReference>
					</Parameter>
				</ParameterValues>
			</Step>
						"#;
        let expected_output = Some(
            "Insert Current Time [ Select ; Target: lkjflkjf::creationAccount[123] ]".to_string(),
        );
        assert_eq!(sanitize(xml.trim()), expected_output);
    }
}
