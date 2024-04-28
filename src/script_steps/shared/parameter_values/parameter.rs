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
            // "Comment" => todo(),
            // "Options" => todo(),
            // "URL" => todo(),
            // "UniversalPathList" => todo!(),
            // "id" => todo!(),
            // "size" => todo!(),
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
                label: "Select".to_string()
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

    /*
                                    <Step index="19" id="89" name="# (comment)" enable="True">
                                        <UUID>01B0EE4B-D571-46DC-9786-DB9EC07B0168</UUID>
                                        <OwnerID></OwnerID>
                                        <Options>4</Options>
                                        <ParameterValues membercount="1">
                                            <Parameter type="Comment">
                                                <Comment value="&quot;Data Source&quot; uses the &quot;Target&quot; dialog!"></Comment>
                                            </Parameter>
                                        </ParameterValues>
                                    </Step>


                                    <Step index="8" id="131" name="Insert File" enable="True">
                                        <UUID>5C9BB1D8-22FF-48BD-8C4E-A8179969F786</UUID>
                                        <OwnerID></OwnerID>
                                        <Options>33317</Options>
                                        <ParameterValues membercount="3">
                                            <Parameter type="Options">
                                                <Options type="Title">
                                                    <Title name="default"></Title>
                                                </Options>
                                                <Options type="Filters">
                                                    <Filters size="2">
                                                        <Filter extension="*.*">
                                                            <Calculation>
                                                                <Text><![CDATA["All Files"]]></Text>
                                                                <ChunkList hash="7C1A1180D9CF6389ACA1725A91FBC7F4">
                                                                    <Chunk type="NoRef">&quot;All Files&quot;</Chunk>
                                                                </ChunkList>
                                                            </Calculation>
                                                        </Filter>
                                                        <Filter extension="mp3; wav; aiff; m4a; wma">
                                                            <Calculation>
                                                                <Text><![CDATA["Audio"]]></Text>
                                                                <ChunkList hash="8220E97AFC59771EB94074FAF7FC04ED">
                                                                    <Chunk type="NoRef">&quot;Audio&quot;</Chunk>
                                                                </ChunkList>
                                                            </Calculation>
                                                        </Filter>
                                                    </Filters>
                                                </Options>
                                                <Options type="Storage">
                                                    <Storage name="" value="0"></Storage>
                                                </Options>
                                                <Options type="Display">
                                                    <Display name="" value="0"></Display>
                                                </Options>
                                                <Options type="Compress">
                                                    <Compress name="Never compress" value="1"></Compress>
                                                </Options>
                                            </Parameter>
                                            <Parameter type="Target">
                                                <Variable value="$myFile">
                                                    <repetition value="1"></repetition>
                                                </Variable>
                                            </Parameter>
                                            <Parameter type="UniversalPathList">
                                                <UniversalPathList membercount="2">
                                                    <ObjectList>
                                                        <Location><![CDATA[file:path/to/file.txt]]></Location>
                                                        <Location><![CDATA[file:path/to/another.txt]]></Location>
                                                    </ObjectList>
                                                </UniversalPathList>
                                            </Parameter>
                                        </ParameterValues>
                                    </Step>




                                    <Step index="12" id="160" name="Insert from URL" enable="True">
                                        <UUID>32913EB8-E03A-43B4-9E07-9DA0F47C5E99</UUID>
                                        <OwnerID></OwnerID>
                                        <Options>268455941</Options>
                                        <ParameterValues membercount="6">
                                            <Parameter type="Boolean">
                                                <Boolean type="Verify SSL Certificates" id="268435456" value="True"></Boolean>
                                            </Parameter>
                                            <Parameter type="Boolean">
                                                <Boolean type="Select" id="4096" value="True"></Boolean>
                                            </Parameter>
                                            <Parameter type="Boolean">
                                                <Boolean type="With dialog" id="128" value="True"></Boolean>
                                            </Parameter>
                                            <Parameter type="Target">
                                                <Variable value="$myTarget">
                                                    <repetition>
                                                        <Calculation datatype="1" position="10">
                                                            <Calculation>
                                                                <Text><![CDATA[$myRep
            &
            $myRep2]]></Text>
                                                                <ChunkList hash="72B08AA8ED8DC2F51F255EB6218CAE0E">
                                                                    <Chunk type="VariableReference">$myRep</Chunk>
                                                                    <Chunk type="NoRef">&#13;&amp;&#13;</Chunk>
                                                                    <Chunk type="VariableReference">$myRep2</Chunk>
                                                                </ChunkList>
                                                            </Calculation>
                                                        </Calculation>
                                                    </repetition>
                                                </Variable>
                                            </Parameter>
                                            <Parameter type="URL">
                                                <URL autoEncode="True">
                                                    <Calculation datatype="1" position="0">
                                                        <Calculation>
                                                            <Text><![CDATA["https://"
            & $myUrl]]></Text>
                                                            <ChunkList hash="3F2F5541942AE8FB83FA3E4A856131CD">
                                                                <Chunk type="NoRef">&quot;https://&quot;&#13;&amp; </Chunk>
                                                                <Chunk type="VariableReference">$myUrl</Chunk>
                                                            </ChunkList>
                                                        </Calculation>
                                                    </Calculation>
                                                </URL>
                                            </Parameter>
                                            <Parameter type="Calculation">
                                                <Calculation datatype="1" position="1">
                                                    <Calculation>
                                                        <Text><![CDATA[List (
                "-X Post" ;
                "-d @$myData" ;
            )]]></Text>
                                                        <ChunkList hash="EAD2A168FC843142524D0834BCA4EE7A">
                                                            <Chunk type="FunctionRef">List</Chunk>
                                                            <Chunk type="NoRef"> (&#13;&#09;&quot;-X Post&quot; ;&#13;&#09;&quot;-d @$myData&quot; ;&#13;)</Chunk>
                                                        </ChunkList>
                                                    </Calculation>
                                                </Calculation>
                                            </Parameter>
                                        </ParameterValues>
                                    </Step>


                                                            <Step index="18" id="191" name="Open Data File" enable="True">
                                    <UUID>AAFAA262-1FAC-4E2B-AA46-43141A340169</UUID>
                                    <OwnerID></OwnerID>
                                    <Options>37</Options>
                                    <ParameterValues membercount="2">
                                        <Parameter type="UniversalPathList">
                                            <UniversalPathList membercount="2">
                                                <ObjectList>
                                                    <Location><![CDATA[file:/path/to/file.txt]]></Location>
                                                    <Location><![CDATA[file:/path/to/alt.txt]]></Location>
                                                </ObjectList>
                                            </UniversalPathList>
                                        </Parameter>
                                        <Parameter type="Target">
                                            <Variable value="$myTarget">
                                                <repetition value="123"></repetition>
                                            </Variable>
                                        </Parameter>
                                    </ParameterValues>
                                </Step>


                                                                                <Step index="20" id="192" name="Write to Data File" enable="True">
                                    <UUID>11CDA5D2-3A45-4C2B-B346-2E53231BB825</UUID>
                                    <OwnerID></OwnerID>
                                    <Options>16903</Options>
                                    <ParameterValues membercount="3">
                                        <Parameter type="id">
                                            <Calculation datatype="2" position="0">
                                                <Calculation>
                                                    <Text><![CDATA[$file
        & $id]]></Text>
                                                    <ChunkList hash="827B8CC2DE3A2A685DC88FD9CE6C5740">
                                                        <Chunk type="VariableReference">$file</Chunk>
                                                        <Chunk type="NoRef">&#13;&amp; </Chunk>
                                                        <Chunk type="VariableReference">$id</Chunk>
                                                    </ChunkList>
                                                </Calculation>
                                            </Calculation>
                                        </Parameter>
                                        <Parameter type="Target">
                                            <Variable value="$myDataSource">
                                                <repetition>
                                                    <Calculation datatype="1" position="10">
                                                        <Calculation>
                                                            <Text><![CDATA[$my
        & $rep]]></Text>
                                                            <ChunkList hash="6E55B861955D98FEEC7DC14BCF52249B">
                                                                <Chunk type="VariableReference">$my</Chunk>
                                                                <Chunk type="NoRef">&#13;&amp; </Chunk>
                                                                <Chunk type="VariableReference">$rep</Chunk>
                                                            </ChunkList>
                                                        </Calculation>
                                                    </Calculation>
                                                </repetition>
                                            </Variable>
                                        </Parameter>
                                        <Encoding type="1" name="UTF-16"></Encoding>
                                        <Parameter type="Boolean">
                                            <Boolean type="Append line feed" id="512" value="True"></Boolean>
                                        </Parameter>
                                    </ParameterValues>
                                </Step>


                                                        <Step index="14" id="193" name="Read from Data File" enable="True">
                                    <UUID>081E1F0D-7FA9-4574-9F91-F29AD856B62C</UUID>
                                    <OwnerID></OwnerID>
                                    <Options>49159</Options>
                                    <ParameterValues membercount="3">
                                        <Parameter type="id">
                                            <Calculation datatype="2" position="0">
                                                <Calculation>
                                                    <Text><![CDATA[$file
        &
        $id]]></Text>
                                                    <ChunkList hash="549A78129B99E46A67055774975608D3">
                                                        <Chunk type="VariableReference">$file</Chunk>
                                                        <Chunk type="NoRef">&#13;&amp;&#13;</Chunk>
                                                        <Chunk type="VariableReference">$id</Chunk>
                                                    </ChunkList>
                                                </Calculation>
                                            </Calculation>
                                        </Parameter>
                                        <Parameter type="size">
                                            <Calculation datatype="2" position="1">
                                                <Calculation>
                                                    <Text><![CDATA[$amount
        &
        $in
        &
        $bytes]]></Text>
                                                    <ChunkList hash="570B5567AF7D2E25D2EF79B3DC8593D3">
                                                        <Chunk type="VariableReference">$amount</Chunk>
                                                        <Chunk type="NoRef">&#13;&amp;&#13;</Chunk>
                                                        <Chunk type="VariableReference">$in</Chunk>
                                                        <Chunk type="NoRef">&#13;&amp;&#13;</Chunk>
                                                        <Chunk type="VariableReference">$bytes</Chunk>
                                                    </ChunkList>
                                                </Calculation>
                                            </Calculation>
                                        </Parameter>
                                        <Parameter type="Target">
                                            <Variable value="$myTarget">
                                                <repetition>
                                                    <Calculation datatype="1" position="10">
                                                        <Calculation>
                                                            <Text><![CDATA[$my
        & $rep]]></Text>
                                                            <ChunkList hash="6E55B861955D98FEEC7DC14BCF52249B">
                                                                <Chunk type="VariableReference">$my</Chunk>
                                                                <Chunk type="NoRef">&#13;&amp; </Chunk>
                                                                <Chunk type="VariableReference">$rep</Chunk>
                                                            </ChunkList>
                                                        </Calculation>
                                                    </Calculation>
                                                </repetition>
                                            </Variable>
                                        </Parameter>
                                        <Encoding type="1" name="UTF-16"></Encoding>
                                    </ParameterValues>
                                </Step>

    */
}
