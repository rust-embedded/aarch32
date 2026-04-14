//! Generic Timer related registers
//!
//! Only valid on Armv7-A and Armv8-R

pub mod cntfrq;
pub mod cnthctl;
pub mod cnthp_ctl;
pub mod cnthp_cval;
pub mod cnthp_tval;
pub mod cntkctl;
pub mod cntp_ctl;
pub mod cntp_cval;
pub mod cntp_tval;
pub mod cntpct;
pub mod cntv_ctl;
pub mod cntv_cval;
pub mod cntv_tval;
pub mod cntvct;
pub mod cntvoff;

pub use cntfrq::Cntfrq;
pub use cnthctl::Cnthctl;
pub use cnthp_ctl::CnthpCtl;
pub use cnthp_cval::CnthpCval;
pub use cnthp_tval::CnthpTval;
pub use cntkctl::Cntkctl;
pub use cntp_ctl::CntpCtl;
pub use cntp_cval::CntpCval;
pub use cntp_tval::CntpTval;
pub use cntpct::CntPct;
pub use cntv_ctl::CntvCtl;
pub use cntv_cval::CntvCval;
pub use cntv_tval::CntvTval;
pub use cntvct::CntVct;
pub use cntvoff::CntVoff;
