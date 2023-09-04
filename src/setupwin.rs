use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    AsyncComponentSender,
};

use crate::templates::MainWindow;

pub struct SetupWin {}

#[derive(Debug)]
pub enum SetupWinInput {}

#[derive(Debug)]
pub enum SetupWinOutput {}

pub struct SetupWinInit {}

#[relm4::component(pub async)]
impl AsyncComponent for SetupWin {
    type CommandOutput = ();
    type Input = SetupWinInput;
    type Output = SetupWinOutput;
    type Init = SetupWinInit;

    view! {
        #[template]
        MainWindow {}
    }

    #[allow(unused_variables)]
    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = SetupWin {};
        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }

    #[allow(unused_variables)]
    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {}
    }
}
