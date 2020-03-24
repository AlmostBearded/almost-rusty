use std::ffi::CString;
use std::fs::read_to_string;

use gl::types::*;
use log;

use crate::assets::shader_asset::ShaderAsset;
use crate::utils::string as string_utils;
use core::ptr;

#[derive(Debug)]
pub struct Shader {
    pub id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            log::debug!("Deleting shader {}", self.id);
            gl::DeleteShader(self.id);
        }
    }
}

pub fn compile_shaders(metas: &Vec<ShaderAsset>) -> Vec<Shader> {
    let mut shaders = Vec::<Shader>::new();
    for (i, meta) in metas.iter().enumerate() {
        log::debug!("Compiling shader {} of {}", i + 1, metas.len());
        let source = read_to_string(&meta.source_path).expect(&format!(
            "Failed to read shader source from file '{}'",
            meta.source_path.display()
        ));
        let source = CString::new(source).unwrap();

        log::trace!("Shader source: {:?}", source);

        let id = unsafe { gl::CreateShader(meta.shader_type.to_gl()) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(id);
        }

        check_errors(id).expect(&format!(
            "Shader {} failed to compile from source at '{}'",
            id,
            meta.source_path.display()
        ));

        shaders.push(Shader { id });
    }
    shaders
}

fn check_errors(id: GLuint) -> Result<(), String> {
    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut log_length: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
        }
        let log_length = log_length as usize;
        let error = string_utils::create_whitespace_cstring(log_length);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                log_length as GLint,
                ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(())
}
