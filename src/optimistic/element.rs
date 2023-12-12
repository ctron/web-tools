/// Support working with [`web_sys::Element`]
pub trait OptimisticElement {
    /// Call [`web_sys::Element::contains`], or return `false` if this is not an [`web_sys::Element`].
    fn contains(&self, target: Option<web_sys::EventTarget>) -> bool;
}

#[cfg(feature = "yew")]
impl OptimisticElement for yew::prelude::NodeRef {
    fn contains(&self, target: Option<web_sys::EventTarget>) -> bool {
        use web_sys::wasm_bindgen::JsCast;

        let target = target
            .as_ref()
            .and_then(|target| target.dyn_ref::<web_sys::Node>());
        if let Some(element) = self.cast::<web_sys::Element>() {
            element.contains(target)
        } else {
            false
        }
    }
}
