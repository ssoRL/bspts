/// The result of this edit
pub enum EditResult<T> {
    /// Edit/create succeeded. Return new task
    Return(Box<T>),
    /// The creation was canceled with no effect
    Cancel,
    /// The task was destroyed
    Destroy,
}