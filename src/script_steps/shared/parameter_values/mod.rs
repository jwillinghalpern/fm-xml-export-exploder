pub mod boolean;
pub mod parameter;
pub(crate) mod target;
use parameter::Parameter;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

use quick_xml::{
    events::Event,
    // name::QName,
    // reader,
    Reader,
};

use self::{
    boolean::{Boolean, Kind},
    target::Target,
};

pub struct ParameterValues {
    parameters: Vec<Parameter>,
}

impl ParameterValues {
    pub fn new() -> Self {
        ParameterValues {
            parameters: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, parameter: Parameter) {
        self.parameters.push(parameter);
    }

    pub fn from_xml(reader: &mut Reader<&[u8]>) -> Result<Self> {
        let mut res = Self::new();

        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"Parameter" {
                        let parameter = Parameter::from_xml(reader, &e)?;
                        res.add_parameter(parameter);
                    }
                }
                Ok(Event::End(e)) => {
                    if e.name().into_inner() == b"ParameterValues" {
                        break;
                    }
                }
                Ok(Event::Eof) => return Err("unexpected end of file".into()),
                _ => {}
            }
            buf.clear();
        }

        Ok(res)
    }

    // convenience methods for getting the first parameter matching that type

    pub fn get_text(&self) -> Option<String> {
        for param in &self.parameters {
            if let Parameter::Text(text) = param {
                return Some(text.clone());
            }
        }
        None
    }

    pub fn get_target(&self) -> Option<Target> {
        for param in &self.parameters {
            if let Parameter::Target(target) = param {
                return Some(target.clone());
            }
        }
        None
    }

    pub fn get_calculation(&self) -> Option<String> {
        for param in &self.parameters {
            if let Parameter::Calculation(calculation) = param {
                return Some(calculation.clone());
            }
        }
        None
    }

    pub fn get_boolean(&self, kind: Kind) -> Option<Boolean> {
        for param in &self.parameters {
            if let Parameter::Boolean(boolean) = param {
                if boolean.kind == kind {
                    return Some(boolean.clone());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use tests::{
        boolean::Boolean,
        target::{FieldReference, TableOccurrenceReference},
    };

    use super::*;

    #[test]
    fn test_get_text() {
        let mut parameter_values = ParameterValues::new();
        parameter_values.add_parameter(Parameter::Text("Hello".to_string()));
        parameter_values.add_parameter(Parameter::Text("World".to_string()));

        assert_eq!(parameter_values.get_text(), Some("Hello".to_string()));
    }

    #[test]
    fn test_get_target() {
        let mut params = ParameterValues::new();
        let fr = Target::FieldReference(FieldReference {
            name: "Field1".to_string(),
            table_occurrence_reference: TableOccurrenceReference {
                name: "my_table".to_string(),
            },
            repetition: "1".to_string(),
        });
        params.add_parameter(Parameter::Target(fr.clone()));
        assert_eq!(params.get_target(), Some(fr));
    }

    #[test]
    fn test_get_calculation() {
        let mut parameter_values = ParameterValues::new();
        parameter_values.add_parameter(Parameter::Calculation("1 + 2".to_string()));

        assert_eq!(
            parameter_values.get_calculation(),
            Some("1 + 2".to_string())
        );
    }

    #[test]
    fn test_get_boolean() {
        let mut params = ParameterValues::new();
        params.add_parameter(Parameter::Boolean(Boolean {
            kind: Kind::Select,
            value: true,
            label: "Select".to_string(),
        }));

        assert_eq!(
            params.get_boolean(Kind::Select).unwrap(),
            Boolean::new(Kind::Select, true, "Select")
        );

        params.parameters.pop();
        params.add_parameter(Parameter::Boolean(Boolean {
            kind: Kind::Select,
            value: false,
            label: "Select".to_string(),
        }));
        assert_eq!(
            params.get_boolean(Kind::Select).unwrap(),
            Boolean::new(Kind::Select, false, "Select")
        );
    }
}
