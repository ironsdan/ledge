#[allow(unused_imports)] 
use std::sync::Arc ; 
#[allow(unused_imports)] 
use std::vec::IntoIter as VecIntoIter ; 
#[allow(unused_imports)] 
use vulkano::device::Device ; 
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor::DescriptorDesc ; 
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor::DescriptorDescTy ; 
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor::DescriptorBufferDesc ;
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor ::
DescriptorImageDesc ; 
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor::DescriptorImageDescDimensions ; 
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor::DescriptorImageDescArray;
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor ::ShaderStages; 
#[allow(unused_imports)] 
use vulkano::descriptor::descriptor_set::DescriptorSet; 
#[allow(unused_imports)] 
use vulkano ::descriptor::descriptor_set::UnsafeDescriptorSet; 
#[allow(unused_imports)]
use vulkano::descriptor::descriptor_set::UnsafeDescriptorSetLayout;
#[allow(unused_imports)] 
use vulkano::descriptor::pipeline_layout::PipelineLayout; 
#[allow(unused_imports)] 
use vulkano::descriptor::pipeline_layout::PipelineLayoutDesc; 
#[allow(unused_imports)] 
use vulkano:: descriptor::pipeline_layout::PipelineLayoutDescPcRange;
#[allow(unused_imports)] 
use vulkano::pipeline::shader ::SpecializationConstants as SpecConstsTrait; 
#[allow(unused_imports)] 
use vulkano::pipeline::shader::SpecializationMapEntry; 

pub struct Shader {
    shader: std::sync::Arc <vulkano::pipeline::shader::ShaderModule>,
} 

impl Shader {
    #[doc = r" Loads the shader in Vulkan as a `ShaderModule`."] 
    #[inline]
    #[allow(unsafe_code)] 
    pub fn load(device : :: std :: sync :: Arc < :: vulkano :: device :: Device >) ->
    Result < Shader, :: vulkano :: OomError >
    {
        let _bytes = :: std :: include_bytes !
        ("/home/danielirons/Coding/Rust/ledge_engine/src/particle.frag");
        static WORDS : & [u32] = &
        [119734787u32, 66304u32, 851978u32, 38u32, 0u32, 131089u32, 1u32,
         393227u32, 1u32, 1280527431u32, 1685353262u32, 808793134u32, 0u32,
         196622u32, 0u32, 1u32, 458767u32, 4u32, 4u32, 1852399981u32, 0u32,
         9u32, 23u32, 196624u32, 4u32, 7u32, 196611u32, 2u32, 450u32,
         655364u32, 1197427783u32, 1279741775u32, 1885560645u32,
         1953718128u32, 1600482425u32, 1701734764u32, 1919509599u32,
         1769235301u32, 25974u32, 524292u32, 1197427783u32, 1279741775u32,
         1852399429u32, 1685417059u32, 1768185701u32, 1952671090u32,
         6649449u32, 262149u32, 4u32, 1852399981u32, 0u32, 393221u32, 9u32,
         1348430951u32, 1953393007u32, 1919905603u32, 100u32, 262149u32,
         23u32, 1868783462u32, 7499628u32, 262149u32, 25u32, 1869377379u32,
         114u32, 327686u32, 25u32, 0u32, 1970037110u32, 101u32, 262149u32,
         27u32, 1868783478u32, 7499628u32, 262215u32, 9u32, 11u32, 16u32,
         196679u32, 23u32, 0u32, 262215u32, 23u32, 30u32, 0u32, 262216u32,
         25u32, 0u32, 0u32, 327752u32, 25u32, 0u32, 35u32, 0u32, 196679u32,
         25u32, 2u32, 262215u32, 27u32, 34u32, 0u32, 262215u32, 27u32, 33u32,
         0u32, 196679u32, 32u32, 0u32, 196679u32, 34u32, 0u32, 196679u32,
         35u32, 0u32, 196679u32, 36u32, 0u32, 196679u32, 37u32, 0u32,
         131091u32, 2u32, 196641u32, 3u32, 2u32, 196630u32, 6u32, 32u32,
         262167u32, 7u32, 6u32, 2u32, 262176u32, 8u32, 1u32, 7u32, 262203u32,
         8u32, 9u32, 1u32, 262187u32, 6u32, 11u32, 1056964608u32, 327724u32,
         7u32, 12u32, 11u32, 11u32, 262187u32, 6u32, 15u32, 1056125747u32,
         131092u32, 16u32, 262167u32, 21u32, 6u32, 4u32, 262176u32, 22u32,
         3u32, 21u32, 262203u32, 22u32, 23u32, 3u32, 262167u32, 24u32, 6u32,
         3u32, 196638u32, 25u32, 24u32, 262176u32, 26u32, 2u32, 25u32,
         262203u32, 26u32, 27u32, 2u32, 262165u32, 28u32, 32u32, 1u32,
         262187u32, 28u32, 29u32, 0u32, 262176u32, 30u32, 2u32, 24u32,
         262187u32, 6u32, 33u32, 1065353216u32, 327734u32, 2u32, 4u32, 0u32,
         3u32, 131320u32, 5u32, 262205u32, 7u32, 10u32, 9u32, 327811u32, 7u32,
         13u32, 10u32, 12u32, 393228u32, 6u32, 14u32, 1u32, 66u32, 13u32,
         327866u32, 16u32, 17u32, 14u32, 15u32, 196855u32, 19u32, 0u32,
         262394u32, 17u32, 18u32, 19u32, 131320u32, 18u32, 65788u32,
         131320u32, 19u32, 327745u32, 30u32, 31u32, 27u32, 29u32, 262205u32,
         24u32, 32u32, 31u32, 327761u32, 6u32, 34u32, 32u32, 0u32, 327761u32,
         6u32, 35u32, 32u32, 1u32, 327761u32, 6u32, 36u32, 32u32, 2u32,
         458832u32, 21u32, 37u32, 34u32, 35u32, 36u32, 33u32, 196670u32,
         23u32, 37u32, 65789u32, 65592u32]; 
         unsafe {
            Ok(Shader
               {
                   shader : :: vulkano :: pipeline::shader ::ShaderModule
                   :: from_words(device, WORDS) ?
               })
        }
    } 
    
    #[doc = r" Returns the module that was created."] 
    #[allow(dead_code)]
    #[inline] 
    pub fn module(& self) -> & :: std :: sync :: Arc < :: vulkano ::
    pipeline :: shader :: ShaderModule > { 
        & self . shader 
    }

    #[doc = r" Returns a logical struct describing the entry point named `{ep_name}`."]
    #[inline] 
    #[allow(unsafe_code)] 
    pub fn main_entry_point(& self) -> ::
    vulkano :: pipeline :: shader :: GraphicsEntryPoint < (), MainInput,
    MainOutput, MainLayout >
    {
        unsafe
        {
            #[allow(dead_code)] static NAME : [u8 ; 5usize] =
            [109u8, 97u8, 105u8, 110u8, 0] ; self . shader .
            graphics_entry_point(:: std :: ffi :: CStr ::
                                 from_ptr(NAME . as_ptr() as * const _),
                                 MainInput, MainOutput,
                                 MainLayout(ShaderStages
                                            {
                                                fragment : true, ..
                                                ShaderStages :: none()
                                            }), :: vulkano :: pipeline ::
                                 shader :: GraphicsShaderType :: Fragment)
        }
    }
} #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] pub struct MainInput ;
#[allow(unsafe_code)] unsafe impl :: vulkano :: pipeline :: shader ::
ShaderInterfaceDef for MainInput
{
    type Iter = MainInputIter ; fn elements(& self) -> MainInputIter
    { MainInputIter { num : 0 } }
} #[derive(Debug, Copy, Clone)] pub struct MainInputIter { num : u16 } impl
Iterator for MainInputIter
{
    type Item = :: vulkano :: pipeline :: shader :: ShaderInterfaceDefEntry ;
    #[inline] fn next(& mut self) -> Option < Self :: Item > { None }
    #[inline] fn size_hint(& self) -> (usize, Option < usize >)
    { let len = 0usize - self . num as usize ; (len, Some(len)) }
} impl ExactSizeIterator for MainInputIter { }
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] pub struct MainOutput ;
#[allow(unsafe_code)] unsafe impl :: vulkano :: pipeline :: shader ::
ShaderInterfaceDef for MainOutput
{
    type Iter = MainOutputIter ; fn elements(& self) -> MainOutputIter
    { MainOutputIter { num : 0 } }
} #[derive(Debug, Copy, Clone)] pub struct MainOutputIter { num : u16 } impl
Iterator for MainOutputIter
{
    type Item = :: vulkano :: pipeline :: shader :: ShaderInterfaceDefEntry ;
    #[inline] fn next(& mut self) -> Option < Self :: Item >
    {
        if self . num == 0u16
        {
            self . num += 1 ; return
            Some(:: vulkano :: pipeline :: shader :: ShaderInterfaceDefEntry
                 {
                     location : 0u32 .. 1u32, format : :: vulkano :: format ::
                     Format :: R32G32B32A32Sfloat, name :
                     Some(:: std :: borrow :: Cow :: Borrowed("f_color"))
                 }) ;
        } None
    } #[inline] fn size_hint(& self) -> (usize, Option < usize >)
    { let len = 1usize - self . num as usize ; (len, Some(len)) }
} 

impl ExactSizeIterator for MainOutputIter { } 


#[derive(Debug, Clone)] pub struct MainLayout(pub ShaderStages); 

#[allow(unsafe_code)] 
unsafe impl PipelineLayoutDesc for MainLayout {
    fn num_sets(& self) -> usize { 
        1usize 
    } 

    fn num_bindings_in_set(& self, set : usize) -> Option <usize> {
        match set { 
            0usize => Some(1usize), 
            _ => None 
        } 
    } 

    fn descriptor(& self, set : usize, binding : usize) -> Option <DescriptorDesc>
    {
        match(set, binding) {
            (0usize, 0usize) => Some(DescriptorDesc {
                    ty: DescriptorDescTy::Buffer(DescriptorBufferDesc { 
                        dynamic: None, 
                        storage: false, 
                    }), 
                    array_count: 1u32, 
                    stages: self.0.clone(), 
                    readonly : false,
                 }), 
            _ => None
        }
    } 
    fn num_push_constants_ranges(& self) -> usize { 
        0usize 
    } 

    fn push_constants_range(& self, num: usize) -> Option<PipelineLayoutDescPcRange> {
        if num != 0 || 0usize == 0 { 
            None 
        } else {
            Some(PipelineLayoutDescPcRange {
                offset: 0, 
                size: 0usize, 
                stages: ShaderStages::all(),
            })
        }
    }
} 

pub mod ty {
    #[repr(C)] 
    #[derive(Copy)] 
    #[allow(non_snake_case)] 
    pub struct color { 
        pub value: [f32; 3usize], 
    } 
    
    impl Clone for color { 
        fn clone(&self) -> Self { 
            color { 
                value: self.value, 
            } 
        } 
    }
} 

#[derive(Debug, Copy, Clone)] 
#[allow(non_snake_case)]
#[repr(C)] 
pub struct SpecializationConstants { } 

impl Default for SpecializationConstants
{ 
    fn default() -> SpecializationConstants { 
        SpecializationConstants { } 
    } 
}

unsafe impl SpecConstsTrait for SpecializationConstants
{
    fn descriptors() -> & 'static [SpecializationMapEntry] {
        static DESCRIPTORS : [SpecializationMapEntry ; 0usize] = [] ; 
        &DESCRIPTORS
    }
}
