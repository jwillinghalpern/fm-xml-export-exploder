use crate::utils::attributes::{try_get_attribute, try_get_attribute_cow};
use anyhow::{anyhow, Result};
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

		// #[strum(serialize = "131072")]
    // Generic, // this is used for things like Allow User Abort, which doesn't include a label/type attr
    // AllowFolderCreation,
    // AppendLineFeed,
    // AppendToExistingPdf,
    // Commit,
    // Condition,
    // CreateFolders,
    // DisableExternalControls,
    // DisableInteraction,
    // ErrorCode,
    // ExitAfterLast,
    // ExpirePassword,
    // FlushCachedExternalData,
    // FlushCachedJoinResults,
    // ForceCommit,
    // Hide,
    // IncludeEditRecordToolbar,
    // Lock,
    // MatchCase,
    // MatchWholeWordsOnly,
    // NoDialog,
    // NoStyle,
    // OpenFile,
    // OpenHidden,
    // OverrideEssLockingConflicts,
    // Password,
    // Pause,
    // PauseInBackground,
    // Select,
    // SelectPerform,
    // SkipIndexes,
    // SkipDataEntryValidation,
    // SpecifyTargetField,
    // StoreOnlyAReference,
    // UpdateEntryOptions,
    // UseAsFileDefault,
    // UseFieldNamesAsColumnNames,
    // VerifySslCertificates,
    // WaitForCompletion,
    // WithDialog,
    // Enabled,
    // Enable,
    // MaxDuration,
    // StartImmediately,
    // Wait,
}

impl Kind {
    fn from_bytes_start(e: &BytesStart) -> Result<Self> {
        let id = try_get_attribute_cow(e, "id")?;
        match id.as_ref() {
            b"4096" => Ok(Kind::Select),
            b"268435456" => Ok(Kind::VerifySslCertificates),
            b"128" => Ok(Kind::WithDialog),
            unknown_id => Err(anyhow::anyhow!(
                "unknown boolean id: {}",
                std::str::from_utf8(unknown_id).unwrap()
            )
            .into()),
        }
    }
}

impl Boolean {
    pub fn new(kind: Kind, value: bool, label: impl ToString) -> Self {
        Boolean {
            kind,
            value,
            label: label.to_string(),
        }
    }

    pub fn from_xml(reader: &mut Reader<&[u8]>) -> Result<Self> {
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
                    label = try_get_attribute(&e, "type")?;
                }
                Event::End(e) if e.name().as_ref() == b"Boolean" => break,
                Event::Eof => return Err(anyhow!("unexpected end of file")),
                _ => {}
            }
            buf.clear();
        }

        let kind = opt_kind.ok_or(anyhow!("missing boolean kind"))?;
        Ok(Boolean::new(kind, value, label))
    }

    pub fn get_label_if_true(&self) -> Option<String> {
        self.value.then(|| self.label.clone())
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

    #[test]
    fn test_unhandled_kind() {
        let xml = r#"<Boolean type="Select" id="1234" value="True"></Boolean>"#;
        let mut reader = Reader::from_str(xml);
        let result = Boolean::from_xml(&mut reader);
        assert!(result.is_err());
    }
}

/*
// Generic	<Boolean id="131072" value="True"></Boolean>
// AllowFolderCreation	<Boolean type="Allow Folder Creation" id="256" value="True"></Boolean>
// AppendLineFeed	<Boolean type="Append line feed" id="512" value="True"></Boolean>
// AppendToExistingPdf	<Boolean type="Append to existing PDF" id="256" value="True"></Boolean>
// Commit	<Boolean type="Commit" value="True"></Boolean>
// Condition	<Boolean type="Condition" id="256" value="True"></Boolean>
// CreateFolders	<Boolean type="Create folders" id="512" value="True"></Boolean>
// DisableExternalControls	<Boolean type="DisableExternalControls" value="True"></Boolean>
// DisableInteraction	<Boolean type="DisableInteraction" value="True"></Boolean>
// ErrorCode	<Boolean type="Error Code" id="512" value="True"></Boolean>
// ExitAfterLast	<Boolean type="Exit after last" position="184" value="True"></Boolean>
// ExpirePassword	<Boolean type="Expire password" value="True"></Boolean>
// FlushCachedExternalData	<Boolean type="Flush cached external data" id="512" value="True"></Boolean>
// FlushCachedJoinResults	<Boolean type="Flush cached join results" id="256" value="True"></Boolean>
// ForceCommit	<Boolean type="Force Commit" id="512" value="True"></Boolean>
// Hide	<Boolean type="Hide" value="True"></Boolean>
// IncludeEditRecordToolbar	<Boolean type="Include Edit Record Toolbar" id="256" value="True"></Boolean>
// Lock	<Boolean type="Lock" id="524288" value="True"></Boolean>
// MatchCase	<Boolean type="Match case" value="True"></Boolean>
// MatchWholeWordsOnly	<Boolean type="Match whole words only" value="True"></Boolean>
// NoDialog	<Boolean type="No dialog" value="True"></Boolean>
// NoStyle	<Boolean type="No style" id="512" value="True"></Boolean>
// OpenFile	<Boolean type="Open File" id="256" value="True"></Boolean>
// OpenHidden	<Boolean type="Open hidden" id="256" value="True"></Boolean>
// OverrideEssLockingConflicts	<Boolean type="Override ESS locking conflicts" id="512" value="True"></Boolean>
// Password	<Boolean type="Password" value="True"></Boolean>
// Pause	<Boolean type="Pause" id="16777216" value="True"></Boolean>
// PauseInBackground	<Boolean type="PauseInBackground" value="True"></Boolean>
// Select	<Boolean type="Select" id="4096" value="True"></Boolean>
// SelectPerform	<Boolean type="Select/perform" id="4096" value="True"></Boolean>
// SkipIndexes	<Boolean type="Skip Indexes" id="512" value="True"></Boolean>
// SkipDataEntryValidation	<Boolean type="Skip data entry validation" id="256" value="True"></Boolean>
// SpecifyTargetField	<Boolean type="Specify target field" id="134217728" value="True"></Boolean>
// StoreOnlyAReference	<Boolean type="Store only a reference" id="2048" value="True"></Boolean>
// UpdateEntryOptions	<Boolean type="Update Entry Options" value="True"></Boolean>
// UseAsFileDefault	<Boolean type="Use as file default" id="512" value="True"></Boolean>
// UseFieldNamesAsColumnNames	<Boolean type="Use field names as column names" value="True"></Boolean>
// VerifySslCertificates	<Boolean type="Verify SSL Certificates" id="268435456" value="True"></Boolean>
// WaitForCompletion	<Boolean type="Wait for completion" id="256" value="True"></Boolean>
// WithDialog	<Boolean type="With dialog" id="128" value="True"></Boolean>
// WithDialog	<Boolean type="With dialog" position="156" value="True"></Boolean>
// Enabled	<Boolean type="enabled" id="256" value="True"></Boolean>
// Enable	<Boolean value="True" name="Activate" type="enable"></Boolean>
// Enable	<Boolean value="True" name="Deactivate" type="enable"></Boolean>
// MaxDuration	<Boolean value="True" type="Max duration"></Boolean>
// Password	<Boolean value="True" type="Password"></Boolean>
// StartImmediately	<Boolean value="True" type="Start immediately"></Boolean>
// Wait	<Boolean value="True" type="Wait"></Boolean>
 */
