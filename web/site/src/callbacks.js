
export function js_search_metadata_update(metadata) {
    self.postMessage(["search_metadata_update", metadata]);
}

export function js_should_search_be_aborted() {
    if (self.shouldAbort) {
        self.shouldAbort = false;
        return true;
    }
    return false;
}