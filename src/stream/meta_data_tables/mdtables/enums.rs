use crate::Result;

#[derive(Debug, Clone)]
pub enum CorTypeVisibility{
    NotPublic,
    Public,
    NestedPublic,
    NestedPrivate,
    NestedFamily,
    NestedAssembly,
    NestedFamANDAssem,
    NestedFamORAssem
}

impl CorTypeVisibility{
    pub fn new(val: usize) -> Self{
        match val & 0x7{
            1 => Self::Public,
            2 => Self::NestedPublic,
            3 => Self::NestedPrivate,
            4 => Self::NestedFamily,
            5 => Self::NestedAssembly,
            6 => Self::NestedFamANDAssem,
            7 => Self::NestedFamORAssem,
            _ => Self::NotPublic,
        }
    }
}

impl Default for CorTypeVisibility{
    fn default() -> Self{
        Self::NotPublic
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeLayout{
    AutoLayout,
    SequentialLayout,
    ExplicitLayout
}

impl CorTypeLayout{
    pub fn new(val: usize) -> Self{
        match val & 0x18{
            0x8 => Self::SequentialLayout,
            0x10 => Self::ExplicitLayout,
            _ => Self::AutoLayout,
        }
    }
}

impl Default for CorTypeLayout{
    fn default() -> Self{
        Self::AutoLayout
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeSemantics{
    Class,
    Interface
}

impl CorTypeSemantics{
    pub fn new(val: usize) -> Self{
        match val & 0x20{
            0x20 => Self::Interface,
            _ => Self::Class,
        }
    }
}

impl Default for CorTypeSemantics{
    fn default() -> Self{
        Self::Class
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeAttrFlags{
    Abstract,
    Sealed,
    SpecialName,
    RTSpecialName,
    Import,
    Serializable,
    WindowsRuntime,
    HasSecurity,
    BeforeFieldInit,
    Forwarder
}

impl CorTypeAttrFlags{
    pub fn new(val: usize) -> Vec<Self>{
        let mut res = vec![];
        if val & 0x00000080 != 0 {
            res.push(Self::Abstract);
        }
        if val & 0x00000100 != 0 {
            res.push(Self::Sealed);
        }
        if val & 0x00000400 != 0 {
            res.push(Self::SpecialName);
        }
        if val & 0x00000800 != 0 {
            res.push(Self::RTSpecialName);
        }
        if val & 0x00001000 != 0 {
            res.push(Self::Import);
        }
        if val & 0x00002000 != 0 {
            res.push(Self::Serializable);
        }
        if val & 0x00004000 != 0 {
            res.push(Self::WindowsRuntime);
        }
        if val & 0x00040000 != 0 {
            res.push(Self::HasSecurity);
        }
        if val & 0x00100000 != 0 {
            res.push(Self::BeforeFieldInit);
        }
        if val & 0x00200000 != 0 {
            res.push(Self::Forwarder);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeStringFormat{
    AnsiClass,
    UnicodeClass,
    AutoClass,
    CustomFormatClass
}

impl CorTypeStringFormat{
    pub fn new(val: usize) -> Self{
        match val & 0x00030000{
            0x00010000 => Self::UnicodeClass,
            0x00020000 => Self::AutoClass,
            0x00030000 => Self::CustomFormatClass,
            _ => Self::AnsiClass,
        }
    }
}

impl Default for CorTypeStringFormat{
    fn default() -> Self{
        Self::AnsiClass
    }
}

#[derive(Debug, Clone)]
pub enum ClrMethodImpl{}

#[derive(Debug, Clone, Default)]
pub struct ClrTypeAttr{
    visibility: CorTypeVisibility,
    layout: CorTypeLayout,
    class_semantics:  CorTypeSemantics,
    flags: Vec<CorTypeAttrFlags>,
    string_format: CorTypeStringFormat
}

impl ClrTypeAttr{
    pub fn set(&mut self, data: &[u8]) -> Result<()>{
        if data.len() != 4{
            return Err(crate::error::Error::FormatError(format!("CtrlTypeAttr incorrect size {} {}", data.len(), 4)));
        }
        let val = crate::utils::read_usize(data)?;
        self.visibility = CorTypeVisibility::new(val);
        self.layout = CorTypeLayout::new(val);
        self.class_semantics = CorTypeSemantics::new(val);
        self.flags = CorTypeAttrFlags::new(val);
        self.string_format = CorTypeStringFormat::new(val);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ClrFieldAttr{}

#[derive(Debug, Clone)]
pub enum ClrMethodAttr{}

#[derive(Debug, Clone)]
pub enum ClrParamAttr{}

#[derive(Debug, Clone)]
pub enum ClrEventAttr{}

#[derive(Debug, Clone)]
pub enum ClrPropertyAttr{}

#[derive(Debug, Clone)]
pub enum ClrMethodSemanticsAttr{}

#[derive(Debug, Clone)]
pub enum ClrPinvokeMap{}

#[derive(Debug, Clone)]
pub enum AssemblyHashAlgorithm{
    None,
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512
}

impl AssemblyHashAlgorithm{
    pub fn new(val: usize) -> AssemblyHashAlgorithm{
        match val{
            0x8003 => Self::Md5,
            0x8004 => Self::Sha1,
            0x800c => Self::Sha256,
            0x800d => Self::Sha384,
            0x800e => Self::Sha512,
            _ => Self::None
        }
    }
}

impl Default for AssemblyHashAlgorithm{
    fn default() -> Self{
        Self::None
    }
}

#[derive(Debug, Clone)]
pub enum CorAssemblyFlagsPA{
    PaNone,
    PaMSIL,
    PaX86,
    PaIA64,
    PaAMD64,
    PaUnknown1,
    PaUnknown2,
    PaUnknown3
}

impl CorAssemblyFlagsPA{
    pub fn new(value: usize) -> Self{
        match value & 0x70{
            0x0010 => Self::PaMSIL,
            0x0020 => Self::PaX86,
            0x0030 => Self::PaIA64,
            0x0040 => Self::PaAMD64,
            0x0050 => Self::PaUnknown1,
            0x0060 => Self::PaUnknown2,
            0x0070 => Self::PaUnknown3,
            _ => Self::PaNone
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClrAssemblyFlags{
    PublicKey,
    PA(CorAssemblyFlagsPA),
    PaSpecified,
    EnableJITcompileTracking,
    DisableJITcompileOptimizer,
    Retargetable
}

impl ClrAssemblyFlags{
    pub fn new(value: usize) -> Vec<ClrAssemblyFlags>{
        let mut res = vec![];
        if value & 1 != 0{
            res.push(ClrAssemblyFlags::PublicKey);
        }
        if value & 0x0100 != 0{
            res.push(ClrAssemblyFlags::Retargetable);
        }
        if value & 0x4000 != 0{
            res.push(ClrAssemblyFlags::DisableJITcompileOptimizer);
        }
        if value & 0x8000 != 0{
            res.push(ClrAssemblyFlags::EnableJITcompileTracking);
        }
        if value & 0x0080 != 0{
            res.push(ClrAssemblyFlags::PaSpecified);
            res.push(ClrAssemblyFlags::PA(CorAssemblyFlagsPA::new(value)));
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum ClrFileFlags{}

#[derive(Debug, Clone)]
pub enum ClrManifestResourceFlags{}

#[derive(Debug, Clone)]
pub enum ClrGenericParamAttr{}
