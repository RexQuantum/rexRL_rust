use rltk::{rex::XpFile};

rltk::embedded_resource!(TITLE_MENU, "../resources/titlemenu.xp");
rltk::embedded_resource!(WFC_DEMO_IMAGE1, "../resources/wfc-populated.xp");

pub struct RexAssets {
    pub menu : XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(TITLE_MENU, "../resources/titlemenu.xp");
        rltk::link_resource!(WFC_DEMO_IMAGE1, "../resources/wfc-populated.xp");

        RexAssets{
            menu : XpFile::from_resource("../resources/titlemenu.xp").unwrap()
        }
    }
}