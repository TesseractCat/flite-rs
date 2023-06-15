use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cst_wave {
    pub type_: *const ::std::os::raw::c_char,
    pub sample_rate: ::std::os::raw::c_int,
    pub num_samples: ::std::os::raw::c_int,
    pub num_channels: ::std::os::raw::c_int,
    pub samples: *mut ::std::os::raw::c_short,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cst_voice_struct {
    pub name: *const ::std::os::raw::c_char,
    pub features: *mut c_void,
    pub ffunctions: *mut c_void,
    pub utt_init: ::std::option::Option<
        unsafe extern "C" fn(u: *mut c_void, v: *mut cst_voice_struct) -> *mut c_void,
    >,
}
pub type cst_voice = cst_voice_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cst_lang_struct {
    pub lang: *const ::std::os::raw::c_char,
    pub lang_init: ::std::option::Option<unsafe extern "C" fn(vox: *mut cst_voice)>,
    pub lex_init: ::std::option::Option<unsafe extern "C" fn() -> *mut c_void>,
}
pub type cst_lang = cst_lang_struct;

extern "C" {
    pub fn flite_init() -> ::std::os::raw::c_int;

    pub static mut flite_lang_list: [cst_lang; 20usize];
    pub static mut flite_lang_list_length: ::std::os::raw::c_int;
    pub fn usenglish_init(voice: *mut cst_voice);
    pub fn cmulex_init() -> *mut c_void;
    pub fn flite_add_lang(
        langname: *const ::std::os::raw::c_char,
        lang_init: *mut c_void,
        lex_init: *mut c_void,
    ) -> ::std::os::raw::c_int;

    pub fn flite_add_voice(voice: *mut cst_voice) -> ::std::os::raw::c_int;
    pub fn flite_voice_select(name: *const ::std::os::raw::c_char) -> *mut cst_voice;
    pub fn flite_voice_load(voice_filename: *const ::std::os::raw::c_char) -> *mut cst_voice;
    pub fn delete_voice(u: *mut cst_voice);

    pub fn register_cmu_us_kal(voxdir: *const ::std::os::raw::c_char) -> *mut cst_voice;
    pub fn register_cmu_us_slt(voxdir: *const ::std::os::raw::c_char) -> *mut cst_voice;

    pub fn flite_feat_set_float(f: *mut c_void, name: *const ::std::os::raw::c_char, v: f32);
    pub fn flite_get_param_float(
        f: *const c_void,
        name: *const ::std::os::raw::c_char,
        def: f32,
    ) -> f32;

    pub fn flite_text_to_wave(
        text: *const ::std::os::raw::c_char,
        voice: *mut cst_voice,
    ) -> *mut cst_wave;

    pub fn cst_free(p: *mut ::std::os::raw::c_void);
}