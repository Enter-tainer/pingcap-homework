/// The buffer size of the BufReader which read original data.
pub const READ_BUFFER_SIZE: usize = 1024 * 1024; // 1M
/// The buffer size of the BufReader which read data pieces.
pub const SLICE_READ_BUFFER_SIZE: usize = 1024 * 1024; // 1M
/// The **total** buffer size of the BufWrite which write data pieces.
pub const WRITE_BUFFER_SIZE: usize = 10 * 1024 * 1024; // 10M
/// The size of each file piece.
pub const PIECE_SIZE: u64 = 500 * 1024 * 1024; // 500M
/// The "k" of top "k" urls.
pub const TOP_URL_COUNT: usize = 100;
