use anyhow::{anyhow, bail, Result};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

use crate::{
    calculations::calculation::Calculation,
    utils::attributes::{get_attribute, try_get_attribute},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    FieldReference(FieldReference),
    Variable(Variable),
}

impl Target {
    pub fn from_xml(reader: &mut Reader<&[u8]>) -> Result<Self> {
        let mut buf: Vec<u8> = Vec::new();
        let mut name = String::new();
        let mut repetition = String::new();
        let mut table_ref = None;

        loop {
            let Ok(event) = reader.read_event_into(&mut buf) else {
                break;
            };
            match event {
                Event::Start(e) => match e.name().as_ref() {
                    b"FieldReference" => name = try_get_attribute(&e, "name")?,
                    b"Variable" => name = try_get_attribute(&e, "value")?,
                    b"TableOccurrenceReference" => table_ref = Some(try_get_attribute(&e, "name")?),
                    b"repetition" => repetition = get_repetition(reader, &e)?,
                    _ => {}
                },
                Event::End(e) if e.name().as_ref() == b"Parameter" => break,
                Event::Eof => bail!("unexpected end of file"),
                _ => {}
            }
            buf.clear()
        }

        let target = match table_ref {
            Some(table_name) => Target::FieldReference(FieldReference {
                name,
                table_occurrence_reference: TableOccurrenceReference { name: table_name },
                repetition,
            }),
            None => Target::Variable(Variable { name, repetition }),
        };
        Ok(target)
    }
}

// TODO: handle undefined name
#[derive(Debug, Clone, PartialEq)]
pub struct FieldReference {
    pub name: String,
    pub table_occurrence_reference: TableOccurrenceReference,
    /// describe repetition as a string because it can be stored in the value attr or in a nested calculation
    pub repetition: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub repetition: String,
}

// TODO: handle undefined name
#[derive(Debug, Clone, PartialEq)]
pub struct TableOccurrenceReference {
    pub name: String,
    // id: u32,
    // uuid: String,
}

impl std::fmt::Display for TableOccurrenceReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn get_repetition(reader: &mut Reader<&[u8]>, e: &BytesStart) -> Result<String> {
    let name = e.name();
    let value = get_attribute(e, "value");
    let res = match (name.as_ref(), value) {
        (b"repetition", Some(rep)) => rep,
        _ => Calculation::from_xml(reader, e).map_err(|e| anyhow!(e))?,
    };
    Ok(res)
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::FieldReference(FieldReference {
                table_occurrence_reference,
                name,
                repetition,
            }) => {
                if *repetition != "1" && !repetition.is_empty() {
                    write!(
                        f,
                        "{}::{}[{}]",
                        table_occurrence_reference, name, repetition
                    )
                } else {
                    write!(f, "{}::{}", table_occurrence_reference, name)
                }
            }
            Target::Variable(Variable { name, repetition }) => {
                if *repetition != "1" && !repetition.is_empty() {
                    write!(f, "{}[{}]", name, repetition)
                } else {
                    write!(f, "{}", name)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_reader(xml: &str) -> Reader<&[u8]> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        reader
    }

    fn get_start<'a>(reader: &mut Reader<&'a [u8]>) -> BytesStart<'a> {
        let mut buf = Vec::new();
        let event = reader.read_event_into(&mut buf).unwrap();
        match event {
            Event::Start(e) => e.to_owned(),
            _ => panic!("unexpected event"),
        }
    }

    #[test]
    fn get_repetition_simple() {
        let xml = r#"<repetition value="2"></repetition>"#;

        let mut reader = get_reader(xml);
        let e = get_start(&mut reader);

        let res = get_repetition(&mut reader, &e).unwrap();
        assert_eq!(res, "2");
    }

    #[test]
    fn get_reptition_calc() {
        let xml = r#"
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
"#;

        let mut reader = get_reader(xml);
        let e = get_start(&mut reader);

        let res = get_repetition(&mut reader, &e).unwrap();
        assert_eq!(res, "$myRepetition1\n+\n$myRepetition2");
    }
}
