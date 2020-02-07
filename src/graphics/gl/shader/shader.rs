use crate::utils;
use gl::types::*;
use std::{ffi::CStr, ptr};

#[derive(Debug)]
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, r#type: GLenum) -> Result<Shader, String> {
        println!("Creating shader of type {}.", r#type);
        println!("Shader source: {:?}", source);

        let id = unsafe { gl::CreateShader(r#type) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(id);
        }

        match Shader::check_errors(id) {
            Ok(_) => {
                println!("Shader {} compiled successfully.", id);
            }
            Err(error) => {
                println!("Shader {} failed to compile.", id);
                return Err(error);
            }
        };

        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
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
            let error = utils::string::create_whitespace_cstring(log_length);
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

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            println!("Deleting shader {}", self.id);
            gl::DeleteShader(self.id);
        }
    }
}
