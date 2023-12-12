/// Allow iterating over an [`HtmlCollection`].
pub struct IterableHtmlCollection<'a>(pub &'a web_sys::HtmlCollection);

impl<'a> IntoIterator for IterableHtmlCollection<'a> {
    type Item = web_sys::Element;
    type IntoIter = HtmlCollectionIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
    }
}

#[doc(hidden)]
pub struct HtmlCollectionIter<'a> {
    list: &'a web_sys::HtmlCollection,
    index: u32,
}

impl<'a> HtmlCollectionIter<'a> {
    pub fn new(list: &'a web_sys::HtmlCollection) -> Self {
        Self { list, index: 0 }
    }
}

impl<'a> Iterator for HtmlCollectionIter<'a> {
    type Item = web_sys::Element;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.list.item(self.index);
        self.index += 1;
        next
    }
}
