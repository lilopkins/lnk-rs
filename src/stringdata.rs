use crate::{strings::{SizedString, StringEncoding}, LinkFlags};
use binread::BinRead;
use encoding_rs::Encoding;
use getset::Getters;

#[derive(BinRead, Getters)]
#[getset(get="pub")]
#[br(import(link_flags: LinkFlags, default_codepage: &'static Encoding))]
pub struct StringData {
    /// NAME_STRING: An optional structure that specifies a description of the
    /// shortcut that is displayed to end users to identify the purpose of the
    /// shell link. This structure MUST be present if the HasName flag is set.
    #[br(
        if(link_flags & LinkFlags::HAS_NAME == LinkFlags::HAS_NAME),
        args(StringEncoding::from(link_flags, default_codepage)),
        map=|s: Option<SizedString>|s.map(|t| t.to_string())
    )]
    name_string: Option<String>,

    /// RELATIVE_PATH: An optional structure that specifies the location of the
    /// link target relative to the file that contains the shell link. When
    /// specified, this string SHOULD be used when resolving the link. This
    /// structure MUST be present if the HasRelativePath flag is set.
    #[br(
        if(link_flags & LinkFlags::HAS_RELATIVE_PATH == LinkFlags::HAS_RELATIVE_PATH),
        args(StringEncoding::from(link_flags, default_codepage)),
        map=|s: Option<SizedString>|s.map(|t| t.to_string())
    )]
    relative_path: Option<String>,

    /// WORKING_DIR: An optional structure that specifies the file system path
    /// of the working directory to be used when activating the link target.
    /// This structure MUST be present if the HasWorkingDir flag is set.
    #[br(
        if(link_flags & LinkFlags::HAS_WORKING_DIR == LinkFlags::HAS_WORKING_DIR),
        args(StringEncoding::from(link_flags, default_codepage)),
        map=|s: Option<SizedString>|s.map(|t| t.to_string())
    )]
    working_dir: Option<String>,

    /// COMMAND_LINE_ARGUMENTS: An optional structure that stores the
    /// command-line arguments that are specified when activating the link
    /// target. This structure MUST be present if the HasArguments flag is set.
    #[br(
        if(link_flags & LinkFlags::HAS_ARGUMENTS == LinkFlags::HAS_ARGUMENTS),
        args(StringEncoding::from(link_flags, default_codepage)),
        map=|s: Option<SizedString>|s.map(|t| t.to_string())
    )]
    command_line_arguments: Option<String>,

    /// ICON_LOCATION: An optional structure that specifies the location of the
    /// icon to be used when displaying a shell link item in an icon view. This
    /// structure MUST be present if the HasIconLocation flag is set.
    #[br(
        if(link_flags & LinkFlags::HAS_ICON_LOCATION == LinkFlags::HAS_ICON_LOCATION),
        args(StringEncoding::from(link_flags, default_codepage)),
        map=|s: Option<SizedString>|s.map(|t| t.to_string())
    )]
    icon_location: Option<String>,
}

#[cfg(feature = "experimental_save")]
pub fn to_data<S: Into<String>>(str_data: S, flags: LinkFlags) -> Vec<u8> {
    let s = str_data.into();
    if !flags.contains(LinkFlags::IS_UNICODE) {
        let mut bytes = vec![0u8; 2];
        for c in s.chars() {
            bytes.push(c as u8); // FIXME: clips non-Latin-1 characters!
        }
        let len = bytes.len() - 2;
        LE::write_u16(&mut bytes, len as u16); // writes u16 len at the start
        bytes
    } else {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let mut bytes = vec![0u8; 2 + utf16.len() * 2];
        LE::write_u16(&mut bytes, utf16.len() as u16);
        LE::write_u16_into(&utf16, &mut bytes[2..]);
        bytes
    }
}
