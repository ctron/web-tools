/// Support working with [`web_sys::HtmlFormElement`]
pub trait HtmlFormElement {
    /// [`web_sys::HtmlFormElement::submit`] if the element is an [`web_sys::HtmlFormElement`].
    fn submit(&self);
}

impl HtmlFormElement for web_sys::HtmlFormElement {
    fn submit(&self) {
        let _ = self.submit();
    }
}

impl HtmlFormElement for Option<web_sys::HtmlFormElement> {
    fn submit(&self) {
        if let Some(form) = self {
            let _ = form.submit();
        }
    }
}

#[cfg(feature = "yew")]
impl HtmlFormElement for yew::prelude::NodeRef {
    fn submit(&self) {
        if let Some(form) = self.cast::<web_sys::HtmlFormElement>() {
            let _ = form.submit();
        }
    }
}
