/// The buffer size of the BufReader which read original data.
pub const READ_BUFFER_SIZE: usize = 4 * 1024 * 1024; // 4M
/// The buffer size of the BufReader which read data pieces.
pub const SLICE_READ_BUFFER_SIZE: usize = 4 * 1024 * 1024; // 4M
/// The **total** buffer size of the BufWrite which write data pieces.
pub const WRITE_BUFFER_SIZE: usize = 500_000_000; // 500M
/// The size of each file piece.
pub const PIECE_SIZE: u64 = 500_000_000; // 500M
/// The "k" of top "k" urls.
pub const TOP_URL_COUNT: usize = 100;
