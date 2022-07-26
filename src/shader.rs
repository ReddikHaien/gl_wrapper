use std::{rc::Rc, borrow::Cow, mem::size_of, collections::HashMap};

use crate::internal::{RawId, RawIdManager};

#[derive(Clone)]
pub struct Shader{
    type_: u32,
    id: Rc<RawId<Self>>
}

impl Shader{
    pub fn new(type_: u32) -> Self{
        unsafe{
            Self{
                id: Rc::new(RawId::from_id(gl::CreateShader(type_))),
                type_
            }   
        }
    }

    pub fn new_from_src(type_: u32, src: &str) -> Self {
        let mut out = Self::new(type_);
        out.assign_source(src);
        out
    }

    pub fn assign_source(&mut self, src: &str){
        let src = Program::create_legal_str(src);
        unsafe{
            let v = [
                src.as_bytes().as_ptr().cast()
            ];
            gl::ShaderSource(self.id(),1,v.as_ptr(),0 as *const i32);
            gl::CompileShader(self.id());
            let mut success = 0;
            gl::GetShaderiv(self.id(), gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as _{
                let mut buf = [0;512];
                let mut len = 0;
                gl::GetShaderInfoLog(self.id(), 512, &mut len, buf.as_mut_ptr().cast());
                let str = String::from_utf8_lossy(&buf[0..len as usize]);
                panic!("Failed to Compile Shader: {}",str)
            }       
        }
    }

    fn id(&self) -> u32{
        self.id.id()
    }
}

impl RawIdManager for Shader{
    fn create_resource() -> u32 {
        panic!("Needs to be created manually");
    }

    fn delete_resource(id: u32) {
        unsafe{
            gl::DeleteShader(id);
        }
    }
}



pub struct Program{
    id: Rc<RawId<Self>>,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

impl Program{
    pub fn new(vertex_shader: Shader, fragment_shader: Shader) -> Self{
        unsafe{
            let id = Rc::new(RawId::new());

            gl::AttachShader(id.id(), vertex_shader.id());
            gl::AttachShader(id.id(),fragment_shader.id());
            gl::LinkProgram(id.id());

            Self::check_status(id.id(), gl::LINK_STATUS, "LinkStatus");

            gl::ValidateProgram(id.id());

            Self::check_status(id.id(), gl::VALIDATE_STATUS, "ValidateStatus");

            Self{
                fragment_shader,
                vertex_shader,
                id
            }
        }
    }

    pub fn bind(&self){
        unsafe{
            gl::UseProgram(self.id.id())
        }
    }

    pub fn get_attributes(&self) -> HashMap<String,i32>{
        unsafe{
            let mut count = 0;
            gl::GetProgramiv(self.id.id(),gl::ACTIVE_ATTRIBUTES,&mut count);

            println!("Found {} attribs",count);

            let mut out = HashMap::new();

            for x in 0..count{
                let mut name_buf = [0;512];
                let mut name_len = 0;
                let mut size = 0;
                let mut type_ = 0;
                gl::GetActiveAttrib(self.id.id(), x as _, 512, &mut name_len, &mut size, &mut type_, name_buf.as_mut_ptr().cast());
                let name = String::from_utf8_lossy(&name_buf[0..name_len as usize]).to_string();
                println!("{}",name);
                out.insert(name, x);
            }

            out
        }
    }

    pub fn get_uniform(&self, name: &str) -> Option<Uniform>{

        unsafe{
            let name = Self::create_legal_str(name);
            let id = gl::GetUniformLocation(self.id.id(), name.as_ptr().cast());

            if id < 0{
                return None;
            }

            let mut name_buf = [0;512];
            let mut name_len = 0;
            let mut size = 0;
            let mut type_ = 0;

            gl::GetActiveUniform(self.id.id(), id as _, 512, &mut name_len, &mut size, &mut type_, name_buf.as_mut_ptr());
            
            Some(Uniform{
                id: id as _,
                size: size as _,
                type_: type_.into()
            })
        }
    }

    fn check_status(program: u32, pname: u32, name: &str){
        unsafe{
            let mut success = 0;
            gl::GetProgramiv(program, pname, &mut success);
            if success == gl::FALSE as _{
                let mut buf = [0;512];
                let mut len = 0;
                gl::GetProgramInfoLog(program,512,&mut len, buf.as_mut_ptr().cast());

                let str = String::from_utf8_lossy(&buf[0..len as usize]);
                panic!("Check of {} for a program failed: Error: {}",name,str);
            }
        }
    }

    fn create_legal_str<'a>(value: &'a str) -> Cow<'a, str>{
        if value.ends_with('\u{00}'){
            Cow::Borrowed(value)
        }
        else{
            let mut out = String::with_capacity(value.len() + 1);
            out.push_str(value);
            out.push('\u{00}');
            Cow::Owned(out)
        }
    }
}

impl RawIdManager for Program{
    fn create_resource() -> u32 {
        unsafe{
            gl::CreateProgram()
        }
    }

    fn delete_resource(id: u32) {
        unsafe{
            gl::DeleteProgram(id)
        }
    }    
}


pub struct Uniform{
    id: i32,
    size: i32,
    type_: UniformType
}

pub enum UniformType{
    Int,
    IntVec2,
    IntVec3,
    IntVec4,
    UInt,
    UIntVec2,
    UIntVec3,
    UIntVec4,
    Bool,
    BoolVec2,
    BoolVec3,
    BoolVec4,
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    FloatMat2,
    FloatMat3,
    FloatMat4,
    FloatMat2x3,
    FloatMat2x4,
    FloatMat3x2,
    FloatMat3x4,
    FloatMat4x2,
    FloatMat4x3,
    Double,
    DoubleVec1,
    DoubleVec2,
    DoubleVec3,
    DoubleVec4,
    DoubleMat2,
    DoubleMat3,
    DoubleMat4,
    DoubleMat2x3,
    DoubleMat2x4,
    DoubleMat3x2,
    DoubleMat3x4,
    DoubleMat4x2,
    DoubleMat4x3,
    Sampler1D,
    Sampler2D,
    Sampler3D,
    SamplerCube,
    Sampler1DShadow,
    Sampler2DShadow,
    Sampler1DArray,
    Sampler2DArray,
    Sampler1DArrayShadow,
    Sampler2DArrayShadow,
    Sampler2DMultiSample,
    Sampler2DMultiSampleArray
}

impl Uniform{
    pub fn set_uniform<T>(&self, data: &dyn UniformValid<T>)
    {
        if size_of::<T>() % 4 != 0{
            panic!("the size of type must be a multiple of 4");
        } 
        unsafe{
            let uptr = data.get_ptr().cast();
            let iptr = data.get_ptr().cast();
            let fptr = data.get_ptr().cast();
            let dptr = data.get_ptr().cast();
            match self.type_ {
                UniformType::Int => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::IntVec2 => gl::Uniform2iv(self.id,self.size,iptr),
                UniformType::IntVec3 => gl::Uniform3iv(self.id,self.size,iptr),
                UniformType::IntVec4 => gl::Uniform4iv(self.id,self.size,iptr),
                UniformType::UInt => gl::Uniform1uiv(self.id,self.size,uptr),
                UniformType::UIntVec2 => gl::Uniform2uiv(self.id,self.size,uptr),
                UniformType::UIntVec3 => gl::Uniform3uiv(self.id,self.size,uptr),
                UniformType::UIntVec4 => gl::Uniform4uiv(self.id,self.size,uptr),
                UniformType::Bool => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::BoolVec2 => gl::Uniform2iv(self.id,self.size,iptr),
                UniformType::BoolVec3 => gl::Uniform3iv(self.id,self.size,iptr),
                UniformType::BoolVec4 => gl::Uniform4iv(self.id,self.size,iptr),
                UniformType::Float => gl::Uniform1fv(self.id,self.size,fptr),
                UniformType::FloatVec2 => gl::Uniform2fv(self.id,self.size,fptr),
                UniformType::FloatVec3 => gl::Uniform3fv(self.id,self.size,fptr),
                UniformType::FloatVec4 => gl::Uniform4fv(self.id,self.size,fptr),
                UniformType::FloatMat2 => gl::UniformMatrix2fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat3 => gl::UniformMatrix3fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat4 => gl::UniformMatrix4fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat2x3 => gl::UniformMatrix2x3fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat2x4 => gl::UniformMatrix2x4fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat3x2 => gl::UniformMatrix3x2fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat3x4 => gl::UniformMatrix3x4fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat4x2 => gl::UniformMatrix4x2fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::FloatMat4x3 => gl::UniformMatrix4x3fv(self.id,self.size, gl::FALSE,fptr),
                UniformType::Double => gl::Uniform1dv(self.id,self.size,dptr),
                UniformType::DoubleVec1 => gl::Uniform1dv(self.id,self.size,dptr),
                UniformType::DoubleVec2 => gl::Uniform2dv(self.id,self.size,dptr),
                UniformType::DoubleVec3 => gl::Uniform3dv(self.id,self.size,dptr),
                UniformType::DoubleVec4 => gl::Uniform4dv(self.id,self.size,dptr),
                UniformType::DoubleMat2 => gl::UniformMatrix2dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat3 => gl::UniformMatrix3dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat4 => gl::UniformMatrix4dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat2x3 => gl::UniformMatrix2x3dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat2x4 => gl::UniformMatrix2x4dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat3x2 => gl::UniformMatrix3x2dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat3x4 => gl::UniformMatrix3x4dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat4x2 => gl::UniformMatrix4x2dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::DoubleMat4x3 => gl::UniformMatrix4x3dv(self.id, self.size, gl::FALSE, dptr),
                UniformType::Sampler1D => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler2D => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler3D => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::SamplerCube => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler1DShadow => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler2DShadow => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler1DArray => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler2DArray => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler1DArrayShadow => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler2DArrayShadow => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler2DMultiSample => gl::Uniform1iv(self.id,self.size,iptr),
                UniformType::Sampler2DMultiSampleArray => gl::Uniform1iv(self.id,self.size,iptr),
            }
        }
    }
}

impl From<u32> for UniformType{
    fn from(x: u32) -> Self {
        match x{
            gl::FLOAT => Self::Float,
            gl::FLOAT_VEC2 => Self::FloatVec2,
            gl::FLOAT_VEC3 => Self::FloatVec3,
            gl::FLOAT_VEC4 => Self::FloatVec4,
            
            gl::DOUBLE => Self::Double,
            gl::DOUBLE_VEC2 => Self::DoubleVec2,
            gl::DOUBLE_VEC3 => Self::DoubleVec3,
            gl::DOUBLE_VEC4 => Self::DoubleVec4,
            
            gl::INT => Self::Int,
            gl::INT_VEC2 => Self::IntVec2,
            gl::INT_VEC3 => Self::IntVec3,
            gl::INT_VEC4 => Self::IntVec4,

            gl::UNSIGNED_INT => Self::UInt,
            gl::UNSIGNED_INT_VEC2 => Self::UIntVec2,
            gl::UNSIGNED_INT_VEC3 => Self::UIntVec3,
            gl::UNSIGNED_INT_VEC4 => Self::UIntVec4,

            gl::BOOL => Self::Bool,
            gl::BOOL_VEC2 => Self::BoolVec2,
            gl::BOOL_VEC3 => Self::BoolVec3,
            gl::BOOL_VEC4 => Self::BoolVec4,
            
            gl::FLOAT_MAT2 => Self::FloatMat2,
            gl::FLOAT_MAT3 => Self::FloatMat3,
            gl::FLOAT_MAT4 => Self::FloatMat4,
            gl::FLOAT_MAT2x3 => Self::FloatMat2x3,
            gl::FLOAT_MAT2x4 => Self::FloatMat2x4,
            gl::FLOAT_MAT3x2 => Self::FloatMat3x2,
            gl::FLOAT_MAT3x4 => Self::FloatMat3x4,
            gl::FLOAT_MAT4x2 => Self::FloatMat4x2,
            gl::FLOAT_MAT4x3 => Self::FloatMat4x3,
            
            gl::DOUBLE_MAT2 => Self::DoubleMat2,
            gl::DOUBLE_MAT3 => Self::DoubleMat3,
            gl::DOUBLE_MAT4 => Self::DoubleMat4,
            gl::DOUBLE_MAT2x3 => Self::DoubleMat2x3,
            gl::DOUBLE_MAT2x4 => Self::DoubleMat2x4,
            gl::DOUBLE_MAT3x2 => Self::DoubleMat3x2,
            gl::DOUBLE_MAT3x4 => Self::DoubleMat3x4,
            gl::DOUBLE_MAT4x2 => Self::DoubleMat4x2,
            gl::DOUBLE_MAT4x3 => Self::DoubleMat4x3,
            
            gl::SAMPLER_1D => Self::Sampler1D,
            gl::SAMPLER_2D => Self::Sampler2D,
            gl::SAMPLER_3D => Self::Sampler3D,
            gl::SAMPLER_CUBE => Self::SamplerCube,
            gl::SAMPLER_1D_SHADOW => Self::Sampler1DShadow,
            gl::SAMPLER_2D_SHADOW => Self::Sampler2DShadow,
            gl::SAMPLER_1D_ARRAY => Self::Sampler1DArray,
            gl::SAMPLER_2D_ARRAY => Self::Sampler2DArray,
            gl::SAMPLER_1D_ARRAY_SHADOW => Self::Sampler1DArrayShadow,
            gl::SAMPLER_2D_ARRAY_SHADOW => Self::Sampler2DArrayShadow,
            gl::SAMPLER_2D_MULTISAMPLE => Self::Sampler2DMultiSample,
            gl::SAMPLER_2D_MULTISAMPLE_ARRAY => Self::Sampler2DMultiSampleArray,

            x => panic!("Unexpected value {}",x)
        }
    }
}

pub trait UniformContainer{
    fn bind(&self);
}

#[macro_export]
macro_rules! make_container {
    (struct $name:ident{
        $(
            $fname:ident : $ftype:ty
        ),*
        $(,)?
    }) => {
        pub struct $name{
            $(
                pub $fname: ($crate::shader::Uniform,$ftype),
            )*
        }

        impl $name{
            pub fn new(program: &Program) -> Self{
                $(
                    let $fname = program.get_uniform(stringify!($fname)).unwrap();
                )*
                Self{
                    $(
                        $fname: ($fname,Default::default()),
                    )*
                }
            }
        }
        
        impl $crate::shader::UniformContainer for $name{
            fn bind(&self){
                $(
                    self.$fname.0.set_uniform(&self.$fname.1);
                )*
            }
        }
    };
}

pub trait UniformValid<T>{
    fn get_ptr(&self) -> *const T;
}

impl<T> UniformValid<T> for T{
    fn get_ptr(&self) -> *const T {
        self
    }
}

impl<T> UniformValid<T> for dyn AsRef<T>{
    fn get_ptr(&self) -> *const T {
        self.as_ref().get_ptr()
    }
}


impl<T> UniformValid<T> for [T]{
    fn get_ptr(&self) -> *const T {
        self.as_ptr()
    }
}
