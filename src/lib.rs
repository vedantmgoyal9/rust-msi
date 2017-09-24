//! A library for reading/writing [Windows
//! Installer](https://en.wikipedia.org/wiki/Windows_Installer) (MSI) files.
//!
//! A Windows Installer file, or *MSI file*, represents a Windows software
//! package and a declarative description of how it should be installed.
//! An MSI file consists of a relational database stored with a [Compound
//! File Binary](https://en.wikipedia.org/wiki/Compound_File_Binary_Format)
//! file.

#![warn(missing_docs)]

extern crate byteorder;
extern crate cfb;

mod internal;

pub use internal::package::Package;
pub use internal::summary::SummaryInfo;
use std::fs;
use std::io;
use std::path::Path;

// ========================================================================= //

/// Opens an existing MSI file at the given path in read-only mode.
pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Package<fs::File>> {
    Package::open(fs::File::open(path)?)
}

// ========================================================================= //
