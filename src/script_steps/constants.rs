use strum_macros::FromRepr;

const UNKNOWN_SCRIPT_STEP: [u32; 16] = [
    2, 3, 15, 52, 53, 54, 58, 100, 110, 162, 163, 170, 171, 173, 198, 204,
];

#[derive(Debug, FromRepr, PartialEq)]
#[repr(u32)]
pub enum ScriptStep {
    Unknown = 0,
    PerformScript = 1,
    GoToPreviousField = 4,
    GoToNextField = 5,
    GoToLayout = 6,
    NewRecordRequest = 7,
    DuplicateRecordRequest = 8,
    DeleteRecordRequest = 9,
    DeleteAllRecords = 10,
    GoToRecordRequestPage = 16,
    GoToField = 17,
    CheckRecord = 19,
    CheckFoundSet = 20,
    UnsortRecords = 21,
    EnterFindMode = 22,
    ShowAllRecords = 23,
    ModifyLastFind = 24,
    OmitRecord = 25,
    OmitMultipleRecords = 26,
    ShowOmittedOnly = 27,
    PerformFind = 28,
    OpenHelp = 32,
    OpenManageDatabase = 38,
    ExitApplication = 44,
    SelectAll = 50,
    EnterBrowseMode = 55,
    IfStart = 68,
    Else = 69,
    IfEnd = 70,
    LoopStart = 71,
    ExitLoopIf = 72,
    LoopEnd = 73,
    CommitRecordRequests = 75,
    SetFieldData = 76,
    FixWindow = 79,
    NewFile = 82,
    AllowUserAbort = 85,
    SetErrorRecording = 86,
    OpenScriptWorkspace = 88,
    Comment = 89,
    HaltScript = 90,
    ReplaceFieldContents = 91,
    Beep = 93,
    SetUseSystemFormats = 94,
    GoToPortalRow = 99,
    CopyRecordRequest = 101,
    FlushCacheToDisk = 102,
    ExitScript = 103,
    OpenSettings = 105,
    CorrectWord = 106,
    SpellingOptions = 107,
    SelectDictionaries = 108,
    EditUserDictionary = 109,
    OpenManageValueLists = 112,
    OpenSharing = 113,
    OpenFileOptions = 114,
    AllowFormattingBar = 115,
    OpenHosts = 118,
    CloseWindow = 121,
    NewWindow = 122,
    IfElse = 125,
    ConstrainFoundSet = 126,
    ExtendFoundSet = 127,
    OpenFindReplace = 129,
    OpenManageDataSources = 140,
    SetVariable = 141,
    GoToObject = 145,
    OpenEditSavedFinds = 149,
    OpenManageLayouts = 151,
    OpenManageContainers = 156,
    OpenManageThemes = 165,
    RefreshObject = 167,
    ClosePopover = 169,
    UploadToServer = 172,
    OpenMyApps = 183,
}

pub fn id_to_script_step(id: &str) -> ScriptStep {
    let id = id.parse::<u32>().unwrap();
    if UNKNOWN_SCRIPT_STEP.contains(&id) {
        ScriptStep::Unknown
    } else {
        ScriptStep::from_repr(id).unwrap_or(ScriptStep::Unknown)
    }
}
