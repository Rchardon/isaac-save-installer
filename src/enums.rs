use strum_macros::{Display, FromRepr};

#[derive(Clone, Copy, Display, FromRepr)]
pub enum IsaacVersion {
    Rebirth,
    Afterbirth,
    AfterbirthPlus,
    AfterbirthPlusBP5,
    Repentance,
    RepentancePlus,
}

#[derive(Clone, Copy, FromRepr, PartialEq)]
pub enum Activity {
    Install,
    Backup,
    Delete,
    ChangeSteamCloud,
    ManuallyInstall,
}
