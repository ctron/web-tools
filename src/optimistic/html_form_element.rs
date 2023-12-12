/// Support working with [`web_sys::HtmlFormElement`]
pub trait OptimisticHtmlFormElement {
    /// [`web_sys::HtmlFormElement::submit`] if the element is an [`web_sys::HtmlFormElement`].
    fn submit(&self);
}

impl OptimisticHtmlFormElement for web_sys::HtmlFormElement {
    fn submit(&self) {
        let _ = self.submit();
    }
}

impl OptimisticHtmlFormElement for Option<web_sys::HtmlFormElement> {
    fn submit(&self) {
        if let Some(form) = self {
            let _ = form.submit();
        }
    }
}

#[cfg(feature = "yew")]
impl OptimisticHtmlFormElement for yew::prelude::NodeRef {
    fn submit(&self) {
        if let Some(form) = self.cast::<web_sys::HtmlFormElement>() {
            let _ = form.submit();
        }
    }
}
