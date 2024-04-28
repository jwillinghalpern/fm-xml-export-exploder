use crate::script_steps::shared::parameter_values::boolean::Kind;
use crate::script_steps::shared::ParameterValues;
use crate::utils::attributes::get_attribute;

use quick_xml::events::Event;
use quick_xml::Reader;

pub fn sanitize(step: &str) -> Option<String> {
    let mut name = String::new();
    let mut text = String::new();
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
                    text = parameter_values.get_text().unwrap();
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
            select.get_label_if_true().map(|label| v.push(label));
        }
        if let Some(target) = target {
            v.push(format!("Target: {}", target));
        }
        if !text.is_empty() {
            v.push(format!("“{}”", &text));
        };
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
			<Step index="0" id="61" name="Insert Text" enable="True">
							<UUID>1FCB6945-5246-4CA4-B2DB-B0A6D3ECE00F</UUID>
							<OwnerID></OwnerID>
							<Options>0</Options>
							<ParameterValues membercount="2">
								<Parameter type="Boolean">
									<Boolean type="Select" id="4096" value="False"></Boolean>
								</Parameter>
								<Parameter type="Text">
									<Text></Text>
								</Parameter>
							</ParameterValues>
						</Step>
						"#;
        let expected_output = Some("Insert Text []".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn select_only() {
        let xml = r#"
				<Step index="1" id="61" name="Insert Text" enable="True">
							<UUID>02429DB1-743E-44D0-B1C1-7F1D13A94F5D</UUID>
							<OwnerID></OwnerID>
							<Options>4096</Options>
							<ParameterValues membercount="2">
								<Parameter type="Boolean">
									<Boolean type="Select" id="4096" value="True"></Boolean>
								</Parameter>
								<Parameter type="Text">
									<Text></Text>
								</Parameter>
							</ParameterValues>
						</Step>
						"#;
        let expected_output = Some("Insert Text [ Select ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn select_and_target() {
        let xml = r#"
				<Step index="2" id="61" name="Insert Text" enable="True">
							<UUID>0F8F3D9A-6942-452D-B0CE-1CF0F04AE7E4</UUID>
							<OwnerID></OwnerID>
							<Options>4101</Options>
							<ParameterValues membercount="3">
								<Parameter type="Boolean">
									<Boolean type="Select" id="4096" value="True"></Boolean>
								</Parameter>
								<Parameter type="Target">
									<Variable value="$hello">
										<repetition value="1"></repetition>
									</Variable>
								</Parameter>
								<Parameter type="Text">
									<Text></Text>
								</Parameter>
							</ParameterValues>
						</Step>
						"#;
        let expected_output = Some("Insert Text [ Select ; Target: $hello ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn target_var_with_crs() {
        let xml = r#"
				<Step index="4" id="61" name="Insert Text" enable="True">
				<UUID>AF75AFA2-572D-42D9-AC3E-B3ABBF73C415</UUID>
				<OwnerID></OwnerID>
				<Options>4101</Options>
				<ParameterValues membercount="3">
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="True"></Boolean>
					</Parameter>
					<Parameter type="Target">
						<Variable value="$hello">
							<repetition value="1"></repetition>
						</Variable>
					</Parameter>
					<Parameter type="Text">
						<Text value="a&#13;b&#13;c"></Text>
					</Parameter>
				</ParameterValues>
			</Step>
						"#;
        let expected_output =
            Some("Insert Text [ Select ; Target: $hello ; “a\rb\rc” ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn target_var_rep() {
        let xml = r#"
				<Step index="4" id="61" name="Insert Text" enable="True">
				<UUID>AF75AFA2-572D-42D9-AC3E-B3ABBF73C415</UUID>
				<OwnerID></OwnerID>
				<Options>4101</Options>
				<ParameterValues membercount="3">
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="True"></Boolean>
					</Parameter>
					<Parameter type="Target">
						<Variable value="$hello">
							<repetition value="4"></repetition>
						</Variable>
					</Parameter>
					<Parameter type="Text">
						<Text value="a&#13;b&#13;c"></Text>
					</Parameter>
				</ParameterValues>
			</Step>
						"#;
        let expected_output =
            Some("Insert Text [ Select ; Target: $hello[4] ; “a\rb\rc” ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn target_field_with_crs() {
        let xml = r#"
				<Step index="6" id="61" name="Insert Text" enable="True">
		<UUID>15A4BD83-3070-419A-AAC5-9AAA0EBE5D88</UUID>
		<OwnerID></OwnerID>
		<Options>4101</Options>
		<ParameterValues membercount="3">
			<Parameter type="Boolean">
				<Boolean type="Select" id="4096" value="True"></Boolean>
			</Parameter>
			<Parameter type="Target">
				<FieldReference id="1" name="id" UUID="4FEADECE-195B-4BC7-83B7-57C5BBD4CD45">
					<repetition>
						<Calculation datatype="1" position="10">
							<Calculation>
								<Text><![CDATA[$myRepetition1
		+
		$myRepetition2]]></Text>
								<ChunkList hash="FDD947ABD0DFD4804B5E5FD020096FA3">
									<Chunk type="VariableReference">$myRepetition1</Chunk>
									<Chunk type="NoRef">&#13;+&#13;</Chunk>
									<Chunk type="VariableReference">$myRepetition2</Chunk>
								</ChunkList>
							</Calculation>
						</Calculation>
					</repetition>
					<TableOccurrenceReference id="1065089" name="lkjflkjf" UUID="04AF7D77-38A6-4E99-B4B5-F27013E04589"></TableOccurrenceReference>
				</FieldReference>
			</Parameter>
			<Parameter type="Text">
				<Text value="a&#13;b&#13;c"></Text>
			</Parameter>
		</ParameterValues>
	</Step>
						"#;
        // TODO: I think the calculation should actually have \r instead of \n and the \t indents should not be added. I.e. try to keep the calc as close as possible to the value stored in the script step
        let expected_output =
            Some("Insert Text [ Select ; Target: lkjflkjf::id[$myRepetition1\n\t\t+\n\t\t$myRepetition2] ; “a\rb\rc” ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn target_field_rep() {
        let xml = r#"
<Step index="6" id="61" name="Insert Text" enable="True">
	<UUID>15A4BD83-3070-419A-AAC5-9AAA0EBE5D88</UUID>
	<OwnerID></OwnerID>
	<Options>4101</Options>
	<ParameterValues membercount="3">
		<Parameter type="Boolean">
			<Boolean type="Select" id="4096" value="True"></Boolean>
		</Parameter>
		<Parameter type="Target">
			<FieldReference id="1" name="id" UUID="4FEADECE-195B-4BC7-83B7-57C5BBD4CD45">
				<repetition value="5"></repetition>
				<TableOccurrenceReference id="1065089" name="lkjflkjf" UUID="04AF7D77-38A6-4E99-B4B5-F27013E04589"></TableOccurrenceReference>
			</FieldReference>
		</Parameter>
		<Parameter type="Text">
			<Text value="a&#13;b&#13;c"></Text>
		</Parameter>
	</ParameterValues>
</Step>
						"#;
        let expected_output =
            Some("Insert Text [ Select ; Target: lkjflkjf::id[5] ; “a\rb\rc” ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn test_sanitize() {
        let xml_input = r#"
    			<Step index="1337" id="61" name="Text einfügen" enable="True">
    				<UUID>FC69A314-34A9-4393-AC4C-B5E442996917</UUID>
    				<OwnerID></OwnerID>
    				<Options>4101</Options>
    				<ParameterValues membercount="3">
    					<Parameter type="Boolean">
    						<Boolean type="Auswahl" id="4096" value="True"></Boolean>
    					</Parameter>
    					<Parameter type="Target">
    						<Variable value="$idleCalcExpression">
    							<repetition value="1"></repetition>
    						</Variable>
    					</Parameter>
    					<Parameter type="Text">
    						<Text value="&quot;hello&quot; &amp; &#13;List(&quot;RemoteControl.PressKey&quot; ; &quot;l&quot; ; &quot;l&quot; )"></Text>
    					</Parameter>
    				</ParameterValues>
    			</Step>
        "#;

        let expected_output =
            Some("Text einfügen [ Auswahl ; Target: $idleCalcExpression ; “\"hello\" & \rList(\"RemoteControl.PressKey\" ; \"l\" ; \"l\" )” ]".to_string());
        assert_eq!(sanitize(xml_input.trim()), expected_output);
    }
}
