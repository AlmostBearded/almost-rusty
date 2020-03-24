use std::collections::HashMap;
use std::ptr;

use gl::types::*;
use log;

use crate::assets::shader_program_asset::ShaderProgramAsset;
use crate::graphics::gl::shader::shader::Shader;
use crate::utils;

#[derive(Debug)]
pub struct ShaderProgram {
    id: GLuint,
}

pub fn link_shader_programs(
    shader_program_assets: &Vec<ShaderProgramAsset>,
    shader_id_lookup_map: &HashMap<String, u8>,
    shaders: &Vec<Shader>,
) -> Vec<ShaderProgram> {
    let mut shader_programs = Vec::<ShaderProgram>::new();
    for (i, asset) in shader_program_assets.iter().enumerate() {
        log::debug!(
            "Linking shader program {} of {}",
            i + 1,
            shader_program_assets.len()
        );

        let id = unsafe { gl::CreateProgram() };

        let vertex_shader_index = shader_id_lookup_map.get(&asset.vertex_shader_id).unwrap();
        let fragment_shader_index = shader_id_lookup_map.get(&asset.vertex_shader_id).unwrap();

        let vertex_shader = &shaders[*vertex_shader_index as usize];
        let fragment_shader = &shaders[*fragment_shader_index as usize];

        unsafe {
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
        }

        unsafe {
            gl::LinkProgram(id);
        }

        check_errors(id).expect(&format!("Shader program {} failed to link", i + 1));

        // detach shaders so they can be destroyed
        unsafe {
            gl::DetachShader(id, vertex_shader.id);
            gl::DetachShader(id, fragment_shader.id);
        }

        shader_programs.push(ShaderProgram { id });
    }
    shader_programs
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

// pub fn activate(&self) {
//     log::debug!("Activate shader program {}", self.id);
//     unsafe {
//         gl::UseProgram(self.id);
//     }
// }

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        log::debug!("Deleting shader program {}", self.id);
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
