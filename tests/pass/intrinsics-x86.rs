#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86 as arch;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as arch;

    fn adc(c_in: u8, a: u32, b: u32) -> (u8, u32) {
        let mut sum = 0;
        // SAFETY: There are no safety requirements for calling `_addcarry_u32`.
        // It's just unsafe for API consistency with other intrinsics.
        let c_out = unsafe { arch::_addcarry_u32(c_in, a, b, &mut sum) };
        (c_out, sum)
    }

    pub fn main() {
        assert_eq!(adc(1, 1, 1), (0, 3));
        assert_eq!(adc(3, u32::MAX, u32::MAX), (2, 1));
    }
}

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use core::arch::x86_64 as arch;

    fn adc(c_in: u8, a: u64, b: u64) -> (u8, u64) {
        let mut sum = 0;
        // SAFETY: There are no safety requirements for calling `_addcarry_u64`.
        // It's just unsafe for API consistency with other intrinsics.
        let c_out = unsafe { arch::_addcarry_u64(c_in, a, b, &mut sum) };
        (c_out, sum)
    }

    pub fn main() {
        assert_eq!(adc(1, 1, 1), (0, 3));
        assert_eq!(adc(3, u64::MAX, u64::MAX), (2, 1));
    }
}

fn main() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    x86::main();
    #[cfg(target_arch = "x86_64")]
    x86_64::main();
}
