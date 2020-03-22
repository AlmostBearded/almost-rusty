use crate::utils;

use gl::types::*;
use std::ptr;

use log;
use crate::graphics::gl::shader::shader::Shader;

#[derive(Debug)]
pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        log::debug!("Creating shader program from shaders {:?}", shaders);

        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id);
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        match ShaderProgram::check_errors(id) {
            Ok(_) => {
                log::info!("Shader program {} linked sucessfully", id);
            }
            Err(error) => {
                log::error!("Shader program {} failed to link", id);
                return Err(error);
            }
        }

        // detach shaders so they can be destroyed
        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id);
            }
        }

        Ok(ShaderProgram { id })
    }

    fn check_errors(id: GLuint) -> Result<(), String> {
        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_length: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
            }
            let error = utils::string::create_whitespace_cstring(log_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    log_length,
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

    pub fn activate(&self) {
        log::debug!("Activate shader program {}", self.id);
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        log::debug!("Deleting shader program {}", self.id);
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
