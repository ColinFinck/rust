use super::apple_base::{opts, Arch};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    let arch = Arch::Armv7;
    let llvm_target = super::apple_base::ios_llvm_target(arch);

    Target {
        llvm_target: llvm_target.into(),
        pointer_width: 32,
        data_layout: "e-m:o-p:32:32-Fi8-f64:32:64-v64:32:64-v128:32:128-a:0:32-n32-S32".into(),
        arch: "arm".into(),
        options: TargetOptions {
            features: "+v7,+vfp3,+neon".into(),
            max_atomic_width: Some(64),
            ..opts("ios", arch)
        },
    }
}
