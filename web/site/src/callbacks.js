
export function js_search_metadata_update(metadata) {
    self.postMessage(["search_metadata_update", metadata]);
}