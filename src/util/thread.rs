pub fn current_thread_id() -> u64 {
    use std::cell::Cell;
    use std::sync::Mutex;

    thread_local! {
        static THREAD_ID: Cell<Option<u64>> = Cell::new(None);
    }

    static NEXT_ID: Mutex<u64> = Mutex::new(0);

    THREAD_ID.with(|id| {
        if let Some(existing) = id.get() {
            return existing;
        }

        let mut next_id = NEXT_ID.lock().unwrap();
        let new_id = *next_id;
        *next_id += 1;
        id.set(Some(new_id));
        new_id
    })
}
