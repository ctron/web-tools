use web_sys::wasm_bindgen::JsCast;

/// Support working with [`web_sys::HtmlElement`]
pub trait OptimisticHtmlElement {
    /// Call [`web_sys::HtmlElement::focus`] if this is an [`web_sys::HtmlElement`].
    fn focus(&self);
}

#[cfg(feature = "yew")]
impl OptimisticHtmlElement for yew::prelude::NodeRef {
    fn focus(&self) {
        if let Some(element) = self.cast::<web_sys::HtmlElement>() {
            let _ = element.focus();
        }
    }
}

impl OptimisticHtmlElement for web_sys::Node {
    fn focus(&self) {
        if let Some(element) = self.dyn_ref::<web_sys::HtmlElement>() {
            let _ = element.focus();
        }
    }
}
