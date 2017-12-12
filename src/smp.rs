//! Low level SMP related instructions.

/// WFE (wait for event) instruction puts the CPU in low-power standby state if
/// the Event Register is cleared and then clears the Event Register. See also
/// [sev()](fn.sev.html)
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe" : : : : "volatile") }
}

/// SEV (send event) instruction sets all other CPUs' Event Registers which
/// wakes them up from low-power standby state. See also [wfe()](fn.wfe.html)
#[inline(always)]
pub fn sev() {
    unsafe { asm!("sev" : : : : "volatile") }
}

