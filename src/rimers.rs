use super::cdef::rime_get_api;
use super::cdef::RimeSessionId;
use super::cdef::{RimeApi, RimeSizedStruct, RimeTraits};
use crate::cdef::CBool;
use crate::cdef::RimeCandidate;
use crate::cdef::RimeCommit;
use crate::cdef::RimeComposition;
use crate::cdef::RimeContext;
use crate::cdef::RimeMenu;
use crate::cdef::RimeStatus;
use std::ffi::NulError;
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_int;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::Utf8Error;

/// Configure paths and start using Rime
pub struct RimeRs {
    api: Rc<RimeApi>,
}

/// Used for api calls that require a session ID
pub struct Session {
    api: Rc<RimeApi>,
    session_id: RimeSessionId,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed convert string with nul to CString: {:?}.", error)]
    CStringError { error: NulError },
    #[fail(display = "CString isn't in valid UTF-8: {:?}.", error)]
    UtfError { error: Utf8Error },
    #[fail(display = "Can't find data directories needed.")]
    DirsNotFound,
    #[fail(display = "Directory path is not valid string.")]
    PathStrInvalid,
    #[fail(display = "get_context returned 0.")]
    NoContext,
}

impl RimeRs {
    /// Start Rime with default paths
    pub fn new() -> Result<RimeRs, Error> {
        let home = dirs::home_dir().ok_or(Error::DirsNotFound)?;
        let user_dir = home.join(".config").join("rimers");
        let data_dir = PathBuf::from("/usr/share/rime-data/");
        RimeRs::new_at(&user_dir, &data_dir)
    }

    /// Start Rime with the given paths
    pub fn new_at(user_dir: &Path, shared_data_dir: &Path) -> Result<RimeRs, Error> {
        let mut traits = unsafe { RimeTraits::uninitialized() };
        let shared_data_dir = shared_data_dir.to_str().ok_or(Error::PathStrInvalid)?;
        let shared_data_dir = CString::new(shared_data_dir)?;
        let user_data_dir = user_dir.to_str().ok_or(Error::PathStrInvalid)?;
        let user_data_dir = CString::new(user_data_dir)?;
        let app_name = CString::new("rime.rimers")?;
        let dist_name = CString::new("Rime")?;
        let dist_code_name = CString::new("rs-rime")?;
        let dist_version = CString::new("0.1.0")?;
        traits.shared_data_dir = shared_data_dir.as_ptr();
        traits.user_data_dir = user_data_dir.as_ptr();
        traits.distribution_name = dist_name.as_ptr();
        traits.distribution_code_name = dist_code_name.as_ptr();
        traits.distribution_version = dist_version.as_ptr();
        traits.app_name = app_name.as_ptr();
        let api = unsafe {
            let a = rime_get_api();
            a.read()
        };
        unsafe {
            api.setup.unwrap()(&mut traits);
            api.initialize.unwrap()(&mut traits);
            api.start_maintenance.unwrap()(0);
        }
        Ok(RimeRs { api: Rc::new(api) })
    }

    pub fn create_session(&self) -> Session {
        let sid = unsafe {
            self.api
                .create_session
                .expect("create_session is null function")()
        };
        Session {
            api: self.api.clone(),
            session_id: sid,
        }
    }

    pub fn user_dir(&self) -> Result<&str, Utf8Error> {
        let cstr = unsafe {
            let d = self.api.get_user_data_dir.unwrap()();
            CStr::from_ptr(d)
        };
        cstr.to_str()
    }
    pub fn shared_dir(&self) -> Result<&str, Utf8Error> {
        let cstr = unsafe {
            let d = self.api.get_shared_data_dir.unwrap()();
            CStr::from_ptr(d)
        };
        cstr.to_str()
    }
}

impl Session {
    pub fn status(&self) -> Result<Status, Error> {
        // Should self.api.free_status be called at some point?
        let mut status = unsafe { RimeStatus::uninitialized() };
        unsafe {
            self.api.get_status.unwrap()(self.session_id, &mut status);
        }
        let status = Status::from_raw(self.api.clone(), status)?;
        Ok(status)
    }

    pub fn context(&self) -> Result<Context, Error> {
        unsafe {
            let mut context = RimeContext::uninitialized();
            if self.api.get_context.expect("get_context is null function")(
                self.session_id,
                &mut context,
            ) == 0
            {
                Err(Error::NoContext)
            } else {
                let context = Context::new(self.api.clone(), context).unwrap();
                Ok(context)
            }
        }
    }
    pub fn get_commit(&self) -> Result<Option<Commit>, Error> {
        let mut rime_commit: RimeCommit;
        let status = unsafe {
            rime_commit = RimeCommit::uninitialized();
            self.api.get_commit.expect("get_commit is null function")(
                self.session_id,
                &mut rime_commit,
            )
        };
        let commit = if status == 0 {
            None
        } else {
            let commit = Commit::from_raw(self.api.clone(), rime_commit)?;
            Some(commit)
        };
        Ok(commit)
    }

    pub fn process_key(&self, key: c_int) -> CBool {
        unsafe {
            self.api.process_key.expect("process_key is null function")(self.session_id, key, 0)
        }
    }

    pub fn session_id(&self) -> RimeSessionId {
        self.session_id
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Status {
    /// TODO: is schema_id and name ever null?
    pub schema_id: Option<String>,
    pub schema_name: Option<String>,
    pub disabled: bool,
    pub composing: bool,
    pub ascii_mode: bool,
    pub full_shape: bool,
    pub simplified: bool,
    pub traditional: bool,
    pub ascii_punct: bool,
    #[derivative(Debug = "ignore")]
    api: Rc<RimeApi>,
    #[derivative(Debug = "ignore")]
    rime: RimeStatus,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Context {
    #[derivative(Debug = "ignore")]
    rime_context: RimeContext,
    #[derivative(Debug = "ignore")]
    api: Rc<RimeApi>,
    pub commit_text_preview: Option<String>,
    pub composition: Composition,
    pub menu: Menu,
}

/// Used to choose among input candidates
#[derive(Debug)]
pub struct Menu {
    pub page_size: c_int,
    pub page_no: c_int,
    pub is_last_page: bool,
    pub highlighted_candidate_index: c_int,
    pub select_keys: Option<String>,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug)]
pub struct Candidate {
    pub text: String,
    pub comment: Option<String>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Commit {
    /// get_commit will probably not return RimeCommit without valid text, I think
    pub text: String,
    #[derivative(Debug = "ignore")]
    rime_commit: RimeCommit,
    #[derivative(Debug = "ignore")]
    api: Rc<RimeApi>,
}

#[derive(Debug)]
pub struct Composition {
    pub length: c_int,
    pub cursor_pos: c_int,
    pub sel_start: c_int,
    pub sel_end: c_int,
    pub preedit: Option<String>,
}

impl Candidate {
    fn from_raw(rime_candidate: &RimeCandidate) -> Result<Candidate, Utf8Error> {
        let text = char_ptr_to_string(rime_candidate.text)?;
        let text = text.expect("Rime returned candidate with no text");
        let comment = char_ptr_to_string(rime_candidate.comment)?;
        Ok(Candidate { text, comment })
    }
}

impl Menu {
    fn from_raw(rime_menu: &RimeMenu) -> Result<Menu, Utf8Error> {
        use std::slice;
        let select_keys = char_ptr_to_string(rime_menu.select_keys)?;
        let rime_candidates = unsafe {
            slice::from_raw_parts(rime_menu.candidates, rime_menu.num_candidates as usize)
        };
        let mut candidates = Vec::with_capacity(rime_candidates.len());
        for rime_candidate in rime_candidates {
            candidates.push(Candidate::from_raw(rime_candidate)?)
        }
        let menu = Menu {
            page_size: rime_menu.page_size,
            page_no: rime_menu.page_no,
            is_last_page: rime_menu.is_last_page != 0,
            highlighted_candidate_index: rime_menu.highlighted_candidate_index,
            candidates: candidates,
            select_keys,
        };
        Ok(menu)
    }
}

impl Commit {
    fn from_raw(api: Rc<RimeApi>, rime_commit: RimeCommit) -> Result<Commit, Utf8Error> {
        let text = char_ptr_to_string(rime_commit.text)?;
        let commit = Commit {
            api,
            rime_commit,
            text: text.expect("Rime actually returned RimeCommit without text."),
        };
        Ok(commit)
    }
}

impl Status {
    fn from_raw(api: Rc<RimeApi>, rime: RimeStatus) -> Result<Status, Utf8Error> {
        let schema_id = char_ptr_to_string(rime.schema_id)?;
        let schema_name = char_ptr_to_string(rime.schema_name)?;
        let status = Status {
            schema_id,
            schema_name,
            disabled: rime.is_disabled != 0,
            composing: rime.is_composing != 0,
            ascii_mode: rime.is_ascii_mode != 0,
            full_shape: rime.is_full_shape != 0,
            simplified: rime.is_simplified != 0,
            traditional: rime.is_traditional != 0,
            ascii_punct: rime.is_ascii_punct != 0,
            api,
            rime,
        };
        Ok(status)
    }
}

impl Composition {
    fn from_raw(rime_composition: &RimeComposition) -> Result<Composition, Utf8Error> {
        let preedit = char_ptr_to_string(rime_composition.preedit)?;
        let composition = Composition {
            length: rime_composition.length,
            cursor_pos: rime_composition.cursor_pos,
            sel_start: rime_composition.sel_start,
            sel_end: rime_composition.sel_end,
            preedit,
        };
        Ok(composition)
    }
}

impl Context {
    fn new(api: Rc<RimeApi>, rime_context: RimeContext) -> Result<Context, Utf8Error> {
        let preview = char_ptr_to_string(rime_context.commit_text_preview)?;
        let composition = Composition::from_raw(&rime_context.composition)?;
        let menu = Menu::from_raw(&rime_context.menu)?;
        let context = Context {
            rime_context,
            api,
            commit_text_preview: preview,
            composition,
            menu,
        };
        Ok(context)
    }
}

impl std::convert::From<std::ffi::NulError> for Error {
    fn from(error: NulError) -> Self {
        Error::CStringError { error }
    }
}

impl std::convert::From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::UtfError { error }
    }
}

impl Drop for Commit {
    fn drop(&mut self) {
        unsafe {
            self.api.free_commit.expect("free_commit is null")(&mut self.rime_commit);
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.api.free_context.unwrap()(&mut self.rime_context);
        }
    }
}

impl Drop for Status {
    fn drop(&mut self) {
        unsafe {
            self.api.free_status.expect("free_status is null")(&mut self.rime);
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            self.api.destroy_session.unwrap()(self.session_id);
        }
    }
}

fn char_ptr_to_string(ptr: *mut std::os::raw::c_char) -> Result<Option<String>, Utf8Error> {
    if ptr.is_null() {
        Ok(None)
    } else {
        let cstr = unsafe { CStr::from_ptr(ptr) };
        Ok(Some(cstr.to_str()?.to_string()))
    }
}
