// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bug reporting made easy.

use std::{env, fmt};
use std::io::{self, Write, LineWriter};
use std::ffi::{OsString, CString};
use std::fs::File;


/// Failure of bug report generation itself.
pub enum Error {
    /// Almost like ICE within ICE.
    Helpless{ pub messages: Vec<String> },
    /// Perhaps the output path was not writable due to a permission error or disk full.
    FileOutput{ pub path: String, pub description: String },
}

fn temp_file_basename() -> String {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();  // should NEVER fail
        let filename = format!("rustc-bug-{:x}XXXXXX{}", time.as_secs(), ext);
}

#[cfg(windows)]
fn create_temp_file() -> Result<(OsString, File), Error> {
    type WCHAR = u16;
    type LPVOID = *mut c_void;
    type LPCWSTR = *const WCHAR;
    type HANDLE = LPVOID;
    type DWORD = c_ulong;

    extern {
        pub fn CreateFileW(lpFileName: LPCWSTR,
                           dwDesiredAccess: DWORD,
                           dwShareMode: DWORD,
                           lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
                           dwCreationDisposition: DWORD,
                           dwFlagsAndAttributes: DWORD,
                           hTemplateFile: HANDLE)
                           -> HANDLE;
    }

    let h = CreateFileW();

}

#[cfg(not(windows))]
fn create_temp_file() -> Result<(OsString, File), Error> {
    use std::os::unix::ffi::OsStringExt;
    use std::os::unix::io::FromRawFd;
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut temp_dir_tmpl = env::temp_dir();
    let ext = ".html";
    {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();  // should never fail
        let filename = format!("rustc-bug-{:x}XXXXXX{}", time.as_secs(), ext);
        temp_dir_tmpl.push(filename);
    }
    let mut temp_dir: Vec<u8> =
        match CString::new(temp_dir_tmpl.into_os_string().into_vec()) {
            Ok(cs) => cs,
            Err(nul_err) => {
                return Err(Error::FileOutput{ path: 
        .expect("maybe invalid $TMPDIR ?").into_bytes_with_nul();

    let fd = unsafe {
        libc::mkstemps(temp_dir.as_mut_ptr() as *mut libc::c_char, ext.len() as libc::c_int)
    };
    if fd < 0 {
        Error::FileOutput{ path: temp_dir (io::Error::last_os_error())
    } else {
        let f = unsafe { File::from_raw_fd(fd) };
        if let Err(e) = f.write_all(&[0x0A; 10*1024]).and(f.sync_all()) {  // reserve 10 KB
            return Error::FileOutput{ path: , description: "".into() }
        }
        Ok((OsString::from_vec(temp_dir), f))
    }
}

fn write(fname: OsString, f: File) -> Result<(), io::Error> {
    println!("Please kindly submit the generated Internal Compiler Error report [{}] to the Rust project.", fname.to_string_lossy());

    Ok(())
}

pub fn generate() -> Result<Path, Error> {
    let (fname, f) = create_temp_file()?;
    let f = {
        let mut w = LineWriter::new(f);
        write!(w, "Foo")?;
        write!(w, "Bar")?;
        w.flush()?;
        w.into_inner()?
    };
}
