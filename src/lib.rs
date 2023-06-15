#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod bindings;
use bindings::*;

use std::ffi::{CString, CStr};
use std::path::Path;
use std::slice;
use std::ops::Drop;

use cstr::cstr;

#[non_exhaustive]
pub enum BuiltinVoice {
    Kal,
    Slt
}

pub struct Voice {
    voice: *mut cst_voice,

    pub tone_mean: Option<f32>, // int_f0_target_mean
    pub tone_stddev: Option<f32>, // int_f0_target_stddev
    pub duration_stretch: Option<f32>, // duration_stretch
}
impl Voice {
    pub fn new(voice_type: BuiltinVoice) -> Self {
        unsafe {
            let voice =
                match voice_type {
                    BuiltinVoice::Kal => register_cmu_us_kal(std::ptr::null()),
                    BuiltinVoice::Slt => register_cmu_us_slt(std::ptr::null()),
                };
            assert!(!voice.is_null());
            Self {
                voice,
                tone_mean: None,
                tone_stddev: None,
                duration_stretch: None
            }
        }
    }
    pub fn from_file(path: impl AsRef<Path>) -> Option<Self> {
        if !path.as_ref().exists() { return None; }

        let c_path = CString::new(path.as_ref().to_string_lossy().to_string()).expect("Failed to construct cstr from path");
        unsafe {
            // TODO: Probably need to iterate through all languages for multi language support
            // - Also detect language from the file?
            if flite_lang_list_length == 0 {
                assert_eq!(flite_add_lang(cstr!("eng").as_ptr(), usenglish_init as *mut _, cmulex_init as *mut _), 1);
            }

            let voice = flite_voice_load(c_path.as_ptr());
            if voice.is_null() { return None; }

            Some(Self {
                voice,
                tone_mean: None,
                tone_stddev: None,
                duration_stretch: None
            })
        }
    }

    pub fn text_to_speech(&self, text: &str) -> Wave {
        let c_text = CString::new(text).expect("Failed to construct cstr");
        unsafe {
            if let Some(duration_stretch) = self.duration_stretch {
                flite_feat_set_float((*self.voice).features, cstr!("duration_stretch").as_ptr(), duration_stretch);
            }
            if let Some(tone_mean) = self.tone_mean {
                flite_feat_set_float((*self.voice).features, cstr!("int_f0_target_mean").as_ptr(), tone_mean);
            }
            if let Some(tone_stddev) = self.tone_stddev {
                flite_feat_set_float((*self.voice).features, cstr!("int_f0_target_stddev").as_ptr(), tone_stddev);
            }

            let c_wave: *mut cst_wave = flite_text_to_wave(c_text.as_ptr(), self.voice);

            let wave = Wave {
                sample_rate: (*c_wave).sample_rate as usize,
                num_samples: (*c_wave).num_samples as usize,
                num_channels: (*c_wave).num_channels as usize,
                samples: slice::from_raw_parts((*c_wave).samples, (*c_wave).num_samples as usize * (*c_wave).num_channels as usize).to_vec()
            };

            cst_free(c_wave as *mut _);

            wave
        }
    }
}
impl Drop for Voice {
    fn drop(&mut self) {
        unsafe { delete_voice(self.voice) }
    }
}

#[derive(Clone, Debug)]
pub struct Wave {
    pub sample_rate: usize,
    pub num_samples: usize,
    pub num_channels: usize,
    pub samples: Vec<i16>
}
impl Wave {
    pub fn duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs_f32(self.samples.len() as f32 / self.sample_rate as f32)
    }
}