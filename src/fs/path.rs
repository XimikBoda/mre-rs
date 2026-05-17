use alloc::string::String;
use alloc::vec::Vec;
use crate::ffi::fs::*;
use crate::ffi::ucs2::{to_ucs2, from_ucs2};

pub const MAX_PATH: usize = 260;

pub fn get_app_path() -> Option<Path> {
    let mut buffer = [0u16; MAX_PATH];

    let result = unsafe{ vm_get_exec_filename(buffer.as_mut_ptr()) };
    
    if result < 0 {
        return None;
    }

    let exec_path = from_ucs2(&buffer);
    Some(Path::from_absolute_string(exec_path))
}

pub fn get_app_dir() -> Option<Path> {
    let app_path = get_app_path()?; 
    app_path.dir_path()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    absolute_path: String,
}

impl Path {
    pub(crate) fn from_absolute_string(abs_path: String) -> Self {
        Self { absolute_path: abs_path }
    }

    pub fn new(path: &str) -> Self {
        let normalized = path.replace('/', "\\");

        let is_absolute = normalized.len() >= 3 
            && normalized.chars().nth(1) == Some(':') 
            && normalized.chars().nth(2) == Some('\\');

        if is_absolute {
            Self::from_absolute_string(normalized)
        } else {
            let mut full = get_app_dir()
                .map(|p| p.absolute_path)
                .unwrap_or_else(|| String::from("e:\\"));
            
            full.push_str(&normalized);
            Self::from_absolute_string(full)
        }
    }

    pub fn join(&self, part: &str) -> Self {
        let mut new_path = self.absolute_path.clone();
        let normalized_part = part.replace('/', "\\");
        
        if !new_path.ends_with('\\') && !normalized_part.starts_with('\\') {
            new_path.push('\\');
        }
        new_path.push_str(&normalized_part);
        
        Self::from_absolute_string(new_path)
    }

    pub fn exists(&self) -> bool {
        crate::fs::fs::exists(self)
    }

    pub fn dir_path(&self) -> Option<Path> {
        if self.absolute_path.ends_with('\\') {
            Some(Self::from_absolute_string(self.absolute_path.clone()))
        } else {
            self.parent()
        }
    }

    pub fn parent(&self) -> Option<Path> {
        let path = self.absolute_path.as_str();
        
        let search_path = if path.len() > 3 && path.ends_with('\\') {
            &path[..path.len() - 1]
        } else {
            path
        };

        if let Some(idx) = search_path.rfind('\\') {
            let split_idx = if idx == 2 { idx + 1 } else { idx };
            let parent_str = String::from(&search_path[..split_idx]);
            Some(Self::from_absolute_string(parent_str))
        } else {
            None
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        let path = self.absolute_path.as_str();
        
        if path.ends_with('\\') {
            return None;
        }

        if let Some(idx) = path.rfind('\\') {
            Some(&path[idx + 1..])
        } else {
            Some(path)
        }
    }

    pub fn extension(&self) -> Option<&str> {
        let name = self.file_name()?;
        
        if let Some(idx) = name.rfind('.') {
            if idx > 0 && idx < name.len() - 1 {
                return Some(&name[idx + 1..]);
            }
        }
        None
    }

    pub fn file_stem(&self) -> Option<&str> {
        let name = self.file_name()?;
        
        if let Some(idx) = name.rfind('.') {
            if idx > 0 {
                return Some(&name[..idx]);
            }
        }
        Some(name)
    }

    pub(crate) fn as_mre_str(&self) -> Vec<u16> {
        to_ucs2(&self.absolute_path)
    }

    pub fn as_str(&self) -> &str {
        &self.absolute_path
    }
}