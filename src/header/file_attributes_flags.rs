use bitflags::bitflags;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::binread_flags::binread_flags;

bitflags! {
    /// The FileAttributesFlags structure defines bits that specify the file attributes of the link
    /// target, if the target is a file system item. File attributes can be used if the link target
    /// is not available, or if accessing the target would be inefficient. It is possible for the
    /// target items attributes to be out of sync with this value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    pub struct FileAttributeFlags: u32 {
        /// The file or directory is read-only. For a file, if this bit is set, applications can read the file but cannot write to it or delete it. For a directory, if this bit is set, applications cannot delete the directory
        const FILE_ATTRIBUTE_READONLY               = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        /// The file or directory is hidden. If this bit is set, the file or folder is not included in an ordinary directory listing.
        const FILE_ATTRIBUTE_HIDDEN                 = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        /// The file or directory is part of the operating system or is used exclusively by the operating system.
        const FILE_ATTRIBUTE_SYSTEM                 = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        /// A bit that MUST be zero.
        const RESERVED1                             = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        /// The link target is a directory instead of a file.
        const FILE_ATTRIBUTE_DIRECTORY              = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        /// The file or directory is an archive file. Applications use this flag to mark files for
        /// backup or removal.
        const FILE_ATTRIBUTE_ARCHIVE                = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        /// A bit that MUST be zero.
        const RESERVED2                             = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        /// The file or directory has no other flags set. If this bit is 1, all other bits in this
        /// structure MUST be clear.
        const FILE_ATTRIBUTE_NORMAL                 = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        /// The file is being used for temporary storage.
        const FILE_ATTRIBUTE_TEMPORARY              = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        /// The file is a sparse file.
        const FILE_ATTRIBUTE_SPARSE_FILE            = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        /// The file or directory has an associated reparse point.
        const FILE_ATTRIBUTE_REPARSE_POINT          = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        /// The file or directory is compressed. For a file, this means that all data in the file
        /// is compressed. For a directory, this means that compression is the default for newly
        /// created files and subdirectories.
        const FILE_ATTRIBUTE_COMPRESSED             = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        /// The data of the file is not immediately available.
        const FILE_ATTRIBUTE_OFFLINE                = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        /// The contents of the file need to be indexed.
        const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED    = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        /// The file or directory is encrypted. For a file, this means that all data in the file is encrypted. For a directory, this means that encryption is the default for newly created files and subdirectories.
        const FILE_ATTRIBUTE_ENCRYPTED              = 0b0000_0000_0000_0000_0100_0000_0000_0000;
    }
}

binread_flags!(FileAttributeFlags, u32);
