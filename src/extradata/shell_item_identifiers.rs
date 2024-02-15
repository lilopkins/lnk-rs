use binread::BinRead;
use getset::Getters;
use serde::Serialize;

use crate::idlist::IdList;

#[derive(Clone, Debug, BinRead, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[br(import(block_size: u32), pre_assert(block_size != 10))]
#[get(get="pub")]
#[allow(unused)]
/// Contains a list of item identifiers.
/// <https://learn.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-itemidlist>
pub struct ShellItemIdentifiers {

    #[br(args((block_size - 8).try_into().unwrap()))]
    items: IdList
}

