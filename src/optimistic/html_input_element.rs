/// Support working with [`web_sys::HtmlInputElement`]
pub trait HtmlInputElement {
    /// Get the [`web_sys::HtmlFormElement`] of an input.
    fn form(&self) -> Option<web_sys::HtmlFormElement>;
}

#[cfg(feature = "yew")]
impl HtmlInputElement for yew::prelude::NodeRef {
    fn form(&self) -> Option<web_sys::HtmlFormElement> {
        self.cast::<web_sys::HtmlInputElement>()
            .and_then(|input| input.form())
    }
}
