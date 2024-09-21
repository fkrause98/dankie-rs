use crate::Bot;

pub trait Module {
    fn load(&self, bot: &mut Bot);
}

macro_rules! modules {
    ($($module:expr),* $(,)?) => {
        vec![$(Box::new($module) as Box<dyn Module>),*]
    };
}
