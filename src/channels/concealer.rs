use super::{BasicClickable, Clickable};
use crate::{
    widgets::{Widget, WidgetConfig},
    Response, SingularLayerId,
};

///A trait for every channel which can be used to hide/unhide multiple other widgets
pub trait Concealer {
    ///set if the widgets should be hidden or shown
    fn set_concealed(&mut self, new_consealed_state: bool);
    ///get if the widgets are hidden or shown
    fn is_concealing(&self) -> bool;
    ///Adds another widget to the hidden layer
    fn add_widget<ReturnChannel, W: Widget + 'static>(
        &mut self,
        widget_config: impl WidgetConfig<ReturnChannel, W> + 'static,
    ) -> Response<ReturnChannel>;
}
///A basic implementation of the Concealer channel. Used by the Concealer widget
pub struct ConcealerReturn {
    pub(crate) layer: SingularLayerId,
    pub(crate) main_button: BasicClickable,
}
impl Concealer for ConcealerReturn {
    fn set_concealed(&mut self, new_consealed_state: bool) {
        self.layer.set_is_active(!new_consealed_state)
    }
    fn is_concealing(&self) -> bool {
        !self.layer.get_active()
    }
    fn add_widget<ReturnChannel, W: Widget + 'static>(
        &mut self,
        widget_config: impl WidgetConfig<ReturnChannel, W> + 'static,
    ) -> Response<ReturnChannel> {
        self.layer.add_widget(widget_config)
    }
}

///This is hooked up to the button that hides/unhides the widgets.
impl Clickable for ConcealerReturn {
    fn has_clicked(&mut self) -> bool {
        self.main_button.has_clicked()
    }
}
