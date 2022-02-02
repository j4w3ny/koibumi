#[derive(Debug)]
pub struct KBConf {
    window: WindowConf,
    base: BaseConf,
}
#[derive(Debug)]
pub struct BaseConf {
    pub title: String,
}
#[derive(Debug)]
pub struct WindowConf {}
