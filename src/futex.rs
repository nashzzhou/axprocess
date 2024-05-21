#[derive(Default)]
/// 用于存储 robust list 的结构
pub struct FutexRobustList {
    /// The location of the head of the robust list in user space
    pub head: usize,
    /// The length of the robust list
    pub len: usize,
}

impl FutexRobustList {
    /// Create a new robust list
    pub fn new(head: usize, len: usize) -> Self {
        Self { head, len }
    }
}

