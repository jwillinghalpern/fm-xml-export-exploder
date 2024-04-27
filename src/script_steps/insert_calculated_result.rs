use crate::script_steps::shared::Target;
use crate::utils::attributes::get_attribute;

// use quick_xml::escape::unescape;
use quick_xml::events::Event;
use quick_xml::Reader;

use super::shared::parameter_values::boolean::Kind;
use super::shared::ParameterValues;

pub fn sanitize(step: &str) -> Option<String> {
    let mut name = String::new();
    let mut calc = String::new();
    let mut select = false;
    let mut target: Option<Target> = None;

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
                    select = parameter_values
                        .get_boolean(Kind::Select)
                        .unwrap_or_default();
                    target = parameter_values.get_target();
                    calc = parameter_values.get_calculation().unwrap_or_default();
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
        let mut v = Vec::new();
        if select {
            v.push("Select".to_string());
        }
        if let Some(target) = target {
            v.push(format!("Target: {}", target));
        }
        if !calc.is_empty() {
            v.push(calc)
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
mod tests {
    use super::*;

    #[test]
    fn full() {
        let xml = r#"
<Step index="7" id="77" name="Insert Calculated Result" enable="True">
	<UUID>68708C7B-106E-4D6A-91B7-E7FFBD1962EF</UUID>
	<OwnerID></OwnerID>
	<Options>20485</Options>
	<ParameterValues membercount="3">
		<Parameter type="Boolean">
			<Boolean type="Select" id="4096" value="True"></Boolean>
		</Parameter>
		<Parameter type="Target">
			<FieldReference id="1" name="id" UUID="4FEADECE-195B-4BC7-83B7-57C5BBD4CD45">
				<repetition value="1"></repetition>
				<TableOccurrenceReference id="1065089" name="lkjflkjf" UUID="04AF7D77-38A6-4E99-B4B5-F27013E04589"></TableOccurrenceReference>
			</FieldReference>
		</Parameter>
		<Parameter type="Calculation">
			<Calculation datatype="1" position="0">
				<Calculation>
					<Text><![CDATA[List (
"a" ;
"b" ;
"c" ;
)]]></Text>
					<ChunkList hash="BA0A8AA601F63955DF5CF6699A65C4C7">
						<Chunk type="FunctionRef">List</Chunk>
						<Chunk type="NoRef"> (&#13;&#09;&quot;a&quot; ;&#13;&#09;&quot;b&quot; ;&#13;&#09;&quot;c&quot; ;&#13;)</Chunk>
					</ChunkList>
				</Calculation>
			</Calculation>
		</Parameter>
	</ParameterValues>
</Step>
"#;
        let expected_output = Some("Insert Calculated Result [ Select ; Target: lkjflkjf::id ; List (\n\"a\" ;\n\"b\" ;\n\"c\" ;\n) ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn empty() {
        let xml = r#"
				<Step index="1" id="77" name="Insert Calculated Result" enable="True">
					<UUID>618DD283-09E0-4FA0-A0A5-23BF7C802F98</UUID>
					<OwnerID></OwnerID>
					<Options>0</Options>
					<ParameterValues membercount="1">
						<Parameter type="Boolean">
							<Boolean type="Select" id="4096" value="False"></Boolean>
						</Parameter>
					</ParameterValues>
				</Step>
    					"#;
        let expected_output = Some("Insert Calculated Result []".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn select() {
        let xml = r#"
			<Step index="3" id="77" name="Insert Calculated Result" enable="True">
				<UUID>9992E77F-2CB7-44CF-B32C-25F84B620A17</UUID>
				<OwnerID></OwnerID>
				<Options>4096</Options>
				<ParameterValues membercount="1">
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="True"></Boolean>
					</Parameter>
				</ParameterValues>
			</Step>	
			"#;

        let expected_output = Some("Insert Calculated Result [ Select ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn var() {
        let xml = r#"
	<Step index="5" id="77" name="Insert Calculated Result" enable="True">
		<UUID>222EA221-5456-4CF2-B9CE-ED97A11F30EE</UUID>
		<OwnerID></OwnerID>
		<Options>5</Options>
		<ParameterValues membercount="2">
			<Parameter type="Boolean">
				<Boolean type="Select" id="4096" value="False"></Boolean>
			</Parameter>
			<Parameter type="Target">
				<Variable value="$hello">
					<repetition value="1"></repetition>
				</Variable>
			</Parameter>
		</ParameterValues>
	</Step>"#;

        let expected_output = Some("Insert Calculated Result [ Target: $hello ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn var_with_rep() {
        let xml = r#"
				<Step index="7" id="77" name="Insert Calculated Result" enable="True">
				<UUID>AFCABD48-A772-42D5-AACD-37825B098FF9</UUID>
				<OwnerID></OwnerID>
				<Options>5</Options>
				<ParameterValues membercount="2">
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="False"></Boolean>
					</Parameter>
					<Parameter type="Target">
						<Variable value="$hello">
							<repetition value="1"></repetition>
						</Variable>
					</Parameter>
				</ParameterValues>
			</Step>"#;
        let expected_output = Some("Insert Calculated Result [ Target: $hello ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn select_var() {
        let xml = r#"
				<Step index="9" id="77" name="Insert Calculated Result" enable="True">
							<UUID>7279B58A-4ABC-4D3A-8640-DEC8D3405C3D</UUID>
							<OwnerID></OwnerID>
							<Options>4101</Options>
							<ParameterValues membercount="2">
								<Parameter type="Boolean">
									<Boolean type="Select" id="4096" value="True"></Boolean>
								</Parameter>
								<Parameter type="Target">
									<Variable value="$hello">
										<repetition value="1"></repetition>
									</Variable>
								</Parameter>
							</ParameterValues>
						</Step>"#;
        let expected_output =
            Some("Insert Calculated Result [ Select ; Target: $hello ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn var_calc() {
        let xml = r#"
				<Step index="13" id="77" name="Insert Calculated Result" enable="True">
							<UUID>24DA3807-1FC1-4197-9E1D-64F0AD296C21</UUID>
							<OwnerID></OwnerID>
							<Options>20485</Options>
							<ParameterValues membercount="3">
								<Parameter type="Boolean">
									<Boolean type="Select" id="4096" value="False"></Boolean>
								</Parameter>
								<Parameter type="Target">
									<Variable value="$hello">
										<repetition value="1"></repetition>
									</Variable>
								</Parameter>
								<Parameter type="Calculation">
									<Calculation datatype="1" position="0">
										<Calculation>
											<Text><![CDATA[List (
	"a" ;
	"b" ;
	"c" ;
)]]></Text>
											<ChunkList hash="BA0A8AA601F63955DF5CF6699A65C4C7">
												<Chunk type="FunctionRef">List</Chunk>
												<Chunk type="NoRef"> (&#13;&#09;&quot;a&quot; ;&#13;&#09;&quot;b&quot; ;&#13;&#09;&quot;c&quot; ;&#13;)</Chunk>
											</ChunkList>
										</Calculation>
									</Calculation>
								</Parameter>
							</ParameterValues>
						</Step>"#;
        let expected_output =
            Some("Insert Calculated Result [ Target: $hello ; List (\n\t\"a\" ;\n\t\"b\" ;\n\t\"c\" ;\n) ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn select_var_calc() {
        let xml = r#"
				<Step index="13" id="77" name="Insert Calculated Result" enable="True">
							<UUID>24DA3807-1FC1-4197-9E1D-64F0AD296C21</UUID>
							<OwnerID></OwnerID>
							<Options>20485</Options>
							<ParameterValues membercount="3">
								<Parameter type="Boolean">
									<Boolean type="Select" id="4096" value="Select"></Boolean>
								</Parameter>
								<Parameter type="Target">
									<Variable value="$hello">
										<repetition value="1"></repetition>
									</Variable>
								</Parameter>
								<Parameter type="Calculation">
									<Calculation datatype="1" position="0">
										<Calculation>
											<Text><![CDATA[List (
	"a" ;
	"b" ;
	"c" ;
)]]></Text>
											<ChunkList hash="BA0A8AA601F63955DF5CF6699A65C4C7">
												<Chunk type="FunctionRef">List</Chunk>
												<Chunk type="NoRef"> (&#13;&#09;&quot;a&quot; ;&#13;&#09;&quot;b&quot; ;&#13;&#09;&quot;c&quot; ;&#13;)</Chunk>
											</ChunkList>
										</Calculation>
									</Calculation>
								</Parameter>
							</ParameterValues>
						</Step>"#;
        let expected_output =
            Some("Insert Calculated Result [ Target: $hello ; List (\n\t\"a\" ;\n\t\"b\" ;\n\t\"c\" ;\n) ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn select_field_calc() {
        let xml = r#"
				<Step index="15" id="77" name="Insert Calculated Result" enable="True">
				<UUID>16264240-4CCF-4B97-8197-D368106BA454</UUID>
				<OwnerID></OwnerID>
				<Options>16389</Options>
				<ParameterValues membercount="3">
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="True"></Boolean>
					</Parameter>
					<Parameter type="Target">
						<FieldReference id="1" name="id" UUID="4FEADECE-195B-4BC7-83B7-57C5BBD4CD45">
							<repetition>
								<Calculation datatype="1" position="10">
									<Calculation>
										<Text><![CDATA[$my_repetition]]></Text>
										<ChunkList hash="B38D709B1D555CC5CFFA56A17C626ED3">
											<Chunk type="VariableReference">$my_repetition</Chunk>
										</ChunkList>
									</Calculation>
								</Calculation>
							</repetition>
							<TableOccurrenceReference id="1065089" name="lkjflkjf" UUID="04AF7D77-38A6-4E99-B4B5-F27013E04589"></TableOccurrenceReference>
						</FieldReference>
					</Parameter>
					<Parameter type="Calculation">
						<Calculation datatype="1" position="0">
							<Calculation>
								<Text><![CDATA[List (
	"a" ;
	"b" ;
	"c" ;
)]]></Text>
								<ChunkList hash="BA0A8AA601F63955DF5CF6699A65C4C7">
									<Chunk type="FunctionRef">List</Chunk>
									<Chunk type="NoRef"> (&#13;&#09;&quot;a&quot; ;&#13;&#09;&quot;b&quot; ;&#13;&#09;&quot;c&quot; ;&#13;)</Chunk>
								</ChunkList>
							</Calculation>
						</Calculation>
					</Parameter>
				</ParameterValues>
			</Step>"#;
        let expected_output =
            Some("Insert Calculated Result [ Select ; Target: lkjflkjf::id[$my_repetition] ; List (\n\t\"a\" ;\n\t\"b\" ;\n\t\"c\" ;\n) ]".to_string());
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    #[test]
    fn select_calc() {
        let xml = r#"
				<Step index="19" id="77" name="Insert Calculated Result" enable="True">
				<UUID>028105F5-E052-40BE-8819-C1E618C923C8</UUID>
				<OwnerID></OwnerID>
				<Options>16388</Options>
				<ParameterValues membercount="2">
					<Parameter type="Boolean">
						<Boolean type="Select" id="4096" value="True"></Boolean>
					</Parameter>
					<Parameter type="Calculation">
						<Calculation datatype="1" position="0">
							<Calculation>
								<Text><![CDATA[List (
	"a" ;
	"b" ;
	"c" ;
)]]></Text>
								<ChunkList hash="BA0A8AA601F63955DF5CF6699A65C4C7">
									<Chunk type="FunctionRef">List</Chunk>
									<Chunk type="NoRef"> (&#13;&#09;&quot;a&quot; ;&#13;&#09;&quot;b&quot; ;&#13;&#09;&quot;c&quot; ;&#13;)</Chunk>
								</ChunkList>
							</Calculation>
						</Calculation>
					</Parameter>
				</ParameterValues>
			</Step>"#;
        let expected_output = Some(
            "Insert Calculated Result [ Select ; List (\n\t\"a\" ;\n\t\"b\" ;\n\t\"c\" ;\n) ]"
                .to_string(),
        );
        assert_eq!(sanitize(xml.trim()), expected_output);
    }

    // end mod tests
}
