use std::fmt::Display;

use crate::{strings::*, LinkFlags};
use binrw::BinRead;
#[cfg(feature = "binwrite")]
use binrw::BinWrite;
use encoding_rs::Encoding;
use getset::{Getters, Setters};
use serde::Serialize;

/// StringData refers to a set of structures that convey user interface and
/// path identification information. The presence of these optional structures
/// is controlled by LinkFlags (section 2.1.1) in the ShellLinkHeader
/// (section 2.1).
#[derive(BinRead, Default, Getters, Setters, Debug, Serialize)]
#[cfg_attr(feature = "binwrite", derive(BinWrite))]
#[getset(get = "pub", set = "pub")]
#[brw(import(link_flags: LinkFlags, encoding: &'static Encoding))]
pub struct StringData {
    /// NAME_STRING: An optional structure that specifies a description of the
    /// shortcut that is displayed to end users to identify the purpose of the
    /// shell link. This structure MUST be present if the HasName flag is set.
    #[brw(args(link_flags, LinkFlags::HAS_NAME, encoding))]
    #[br(parse_with = parse_sized_string)]
    #[cfg_attr(feature="binwrite", bw(write_with=write_sized_string))]
    name_string: Option<String>,

    /// RELATIVE_PATH: An optional structure that specifies the location of the
    /// link target relative to the file that contains the shell link. When
    /// specified, this string SHOULD be used when resolving the link. This
    /// structure MUST be present if the HasRelativePath flag is set.
    #[brw(args(link_flags, LinkFlags::HAS_RELATIVE_PATH, encoding))]
    #[br(parse_with = parse_sized_string)]
    #[cfg_attr(feature="binwrite", bw(write_with=write_sized_string))]
    relative_path: Option<String>,

    /// WORKING_DIR: An optional structure that specifies the file system path
    /// of the working directory to be used when activating the link target.
    /// This structure MUST be present if the HasWorkingDir flag is set.
    #[brw(args(link_flags, LinkFlags::HAS_WORKING_DIR, encoding))]
    #[br(parse_with = parse_sized_string)]
    #[cfg_attr(feature="binwrite", bw(write_with=write_sized_string))]
    working_dir: Option<String>,

    /// COMMAND_LINE_ARGUMENTS: An optional structure that stores the
    /// command-line arguments that are specified when activating the link
    /// target. This structure MUST be present if the HasArguments flag is set.
    #[brw(args(link_flags, LinkFlags::HAS_ARGUMENTS, encoding))]
    #[br(parse_with = parse_sized_string)]
    #[cfg_attr(feature="binwrite", bw(write_with=write_sized_string))]
    command_line_arguments: Option<String>,

    /// ICON_LOCATION: An optional structure that specifies the location of the
    /// icon to be used when displaying a shell link item in an icon view. This
    /// structure MUST be present if the HasIconLocation flag is set.
    #[brw(args(link_flags, LinkFlags::HAS_ICON_LOCATION, encoding))]
    #[br(parse_with = parse_sized_string)]
    #[cfg_attr(feature="binwrite", bw(write_with=write_sized_string))]
    icon_location: Option<String>,
}

impl Display for StringData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();

        if let Some(name_string) = self.name_string().as_ref() {
            parts.push(format!("name-string={name_string}"));
        }
        if let Some(relative_path) = self.relative_path().as_ref() {
            parts.push(format!("relative-path={relative_path}"));
        }
        if let Some(working_dir) = self.working_dir().as_ref() {
            parts.push(format!("working-dir={working_dir}"));
        }
        if let Some(command_line_arguments) = self.name_string().as_ref() {
            parts.push(format!("cli-args={command_line_arguments}"));
        }
        if let Some(icon_location) = self.icon_location().as_ref() {
            parts.push(format!("icon-location={icon_location}"));
        }

        parts.join(",").fmt(f)
    }
}
