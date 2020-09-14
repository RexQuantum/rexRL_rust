use rltk::{rex::XpFile};

rltk::embedded_resource!(TITLE_MENU, "../resources/titlemenu.xp");

pub struct RexAssets {
    pub menu : XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(TITLE_MENU, "../resources/titlemenu.xp");

        RexAssets{
            menu : XpFile::from_resource("../resources/titlemenu.xp").unwrap()
        }
    }
}