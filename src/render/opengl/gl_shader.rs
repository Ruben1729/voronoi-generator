use std::fs::File;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

struct GlShader {
    name: String,
    shader_program_id: gl::GLuint,
    shader_ids: Vec<gl::GLuint>
}

impl GlShader {
    pub fn set_int(&mut self, name: String, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.shader_program_id, CString::from(name)), value);
    }

    pub fn set_int_array(&mut self, name: String, count: u32, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.shader_program_id, CString::from(name)), count, &value)
    }

    pub fn set_float(&mut self, name: String, value: f32) {
        gl::Uniform1i(gl::GetUniformLocation(self.shader_program_id, CString::from(name)), value);
    }

    pub fn set_float2() {

    }

    pub fn set_float3() {

    }

    pub fn set_float4() {

    }

    pub fn set_matrix4() {

    }

    pub fn is_functional() -> bool {
        return false;
    }

    pub fn load_shader(&mut self, file_path: String) {
        let src = Self::load_shader_src(file_path);

    }

    fn split_shader_src(source: String) {

    }

    fn create_shader_program(&mut self, shader_sources: &Map<gl::GLenum, String>) {
        self.shader_program_id = gl::CreateProgram();

        num_shaders = shader_sources.size();

        assert!(num_shaders >= 2 && num_shaders <= 3, "Invalid number of shader sources.");

        for entry in shader_sources {
            let shader: u32 = self.create_shader(entry.0, entry.1);
            gl::AttachShader(self.shader_program_id, shader);
        }

        let mut success: u32;
        gl::LinkProgram(self.shader_program_id);
        gl::GetProgramiv(self.shader_program_id, gl::LINK_STATUS, &success);
        assert!(success, "Unable to compile shader.");

        if !success {
            exit(1);
        }

        for shader in self.shader_ids {
            gl::DeleteShader(shader);
        }
    }

    pub fn create_shader(&mut self, shader_type: gl::GLenum, src: &String) -> u32 {
        let shader_src = CString::from(src);
        let shader_id: u32 = gl::CreateShader(shader_type);
        gl::ShaderSource(shader_id, 1, &shader_src, null());
        gl::CompileShader(shader_id);

        let mut success: i32;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &success);
        assert!(success, "Unable to compile shader.");

        if !success {
            exit(1);
        }

        self.shader_ids.push(shader_id);
        return shader_id;
    }

    fn load_shader_src(file_path: String) -> String {
        return fs::read_to_string(file_path).expect("Unable to read shader file.");
    }

    pub fn bind(&mut self) {
        gl::UseProgram(self.shader_program_id);
    }

    pub fn unbind(&mut self) {
        gl::UseProgram(0);
    }
}
