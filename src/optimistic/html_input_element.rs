use web_sys::wasm_bindgen::JsCast;

/// Support working with [`web_sys::HtmlInputElement`]
pub trait OptimisticHtmlInputElement {
    /// Get the [`web_sys::HtmlFormElement`] of an input.
    fn form(&self) -> Option<web_sys::HtmlFormElement>;

    /// Get the [`web_sys::HtmlInputElement::checked`] state of an input.
    fn checked(&self) -> bool;
}

impl OptimisticHtmlInputElement for web_sys::Node {
    fn form(&self) -> Option<web_sys::HtmlFormElement> {
        self.dyn_ref::<web_sys::HtmlInputElement>()
            .and_then(|input| input.form())
    }

    fn checked(&self) -> bool {
        self.dyn_ref::<web_sys::HtmlInputElement>()
            .map(|input| input.checked())
            .unwrap_or_default()
    }
}

#[cfg(feature = "yew")]
impl OptimisticHtmlInputElement for yew::prelude::NodeRef {
    fn form(&self) -> Option<web_sys::HtmlFormElement> {
        self.cast::<web_sys::HtmlInputElement>()
            .and_then(|input| input.form())
    }

    fn checked(&self) -> bool {
        self.cast::<web_sys::HtmlInputElement>()
            .map(|input| input.checked())
            .unwrap_or_default()
    }
}
