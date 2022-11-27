#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Standard {
    British,

    Japanese,

    /// NEM-standards are used by model railway industry and hobbyists in Europe.
    NEM,

    /// NMRA standards are used widely in North America and by certain special
    /// interest groups all over the world.
    NMRA,
}
