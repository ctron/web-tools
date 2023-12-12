/// Support working with [`web_sys::HtmlElement`]
pub trait HtmlElement {
    /// Call [`web_sys::HtmlElement::focus`] if this is an [`web_sys::HtmlElement`].
    fn focus(&self);
}

#[cfg(feature = "yew")]
impl HtmlElement for yew::prelude::NodeRef {
    fn focus(&self) {
        if let Some(input) = self.cast::<web_sys::HtmlElement>() {
            let _ = input.focus();
        }
    }
}
