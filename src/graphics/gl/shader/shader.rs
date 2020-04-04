use std::ffi::{CStr, CString};
use std::fs::read_to_string;

use gl::types::*;
use log;

use crate::assets::shader_asset::ShaderAsset;
use crate::utils;
use crate::utils::string as string_utils;
use core::ptr;

#[derive(Debug)]
pub struct Shader {
    pub id: GLuint,
}

pub fn compile_shaders(assets: &Vec<ShaderAsset>) -> Vec<Shader> {
    let mut shaders = Vec::<Shader>::new();
    for asset in assets {
        log::info!(
            "Compiling shaders from shader asset '{}'",
            asset.asset_path.display()
        );

        // # Compile shaders

        log::debug!("Compiling vertex shader");
        let vertex_shader_id = compile_shader(gl::VERTEX_SHADER, &asset.vertex_shader_source);
        log::debug!("Compiling fragment shader");
        let fragment_shader_id = compile_shader(gl::FRAGMENT_SHADER, &asset.fragment_shader_source);

        // # Link shader program

        log::info!(
            "Linking shaders from shader asset '{}'",
            asset.asset_path.display()
        );
        let shader_id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(shader_id, vertex_shader_id);
            gl::AttachShader(shader_id, fragment_shader_id);
            gl::LinkProgram(shader_id);
        }

        check_shader_program_errors(shader_id).expect("Shader failed to link");

        unsafe {
            // Detach shaders so they can be deleted
            gl::DetachShader(shader_id, vertex_shader_id);
            gl::DetachShader(shader_id, fragment_shader_id);

            // Delete shaders
            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);
        }

        shaders.push(Shader { id: shader_id });
    }
    shaders
}

fn compile_shader(shader_type: GLenum, source: &CStr) -> u32 {
    let id = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(id);
    }
    check_shader_errors(id).expect("Shader failed to compile");
    id
}

fn check_shader_errors(id: GLuint) -> Result<(), String> {
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

fn check_shader_program_errors(id: GLuint) -> Result<(), String> {
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

impl Drop for Shader {
    fn drop(&mut self) {
        log::debug!("Deleting shader {}", self.id);
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
