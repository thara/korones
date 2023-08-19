/// Kinds of CPU interrupts
///
/// It currently supports NMI and IRQ only.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code, clippy::upper_case_acronyms)]
pub(crate) enum Interrupt {
    NMI,
    IRQ,
}
