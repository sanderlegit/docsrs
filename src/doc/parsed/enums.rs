
/*impl Doc<Parsed> {
    pub(super) fn search_keys_enums<'a>(
        &self,
        id: &'a Id,
        base_path: &'a str,
    ) -> impl Iterator<Item = SearchKey> + 'a {
        let krate = &self.0.ast;
        krate.index.iter().filter_map(move |(id, item)| {
            Some(SearchKey {
                id: 0,
                key: String::new(),
            })
        })
    }
}
*/
