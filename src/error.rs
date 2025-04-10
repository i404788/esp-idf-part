use alloc::string::String;
use alloc::vec::Vec;

use thiserror::Error;

/// Partition table errors
#[derive(Debug, Error)]
pub enum Error {
    /// Two or more partitions with the same name were found
    #[cfg_attr(
        feature = "std",
        error("Two or more partitions with the same name ('{0}') were found")
    )]
    DuplicatePartitions(String),

    /// The checksum in the binary data does not match the computed value
    #[cfg_attr(
        feature = "std",
        error(
            "The binary's checksum is invalid (expected '{expected:?}', computed '{computed:?}')"
        )
    )]
    InvalidChecksum {
        expected: Vec<u8>,
        computed: Vec<u8>,
    },

    /// Partition with type 'data' and subtype 'ota' must have size of 0x2000
    /// (8k) bytes
    #[cfg_attr(
        feature = "std",
        error("Partition with type 'data' and subtype 'ota' must have size of 0x2000 (8k) bytes")
    )]
    InvalidOtadataPartitionSize,

    /// The length of the binary data is not a multiple of 32
    #[cfg_attr(
        feature = "std",
        error("The length of the binary data is not a multiple of 32")
    )]
    LengthNotMultipleOf32,

    /// Multiple partitions with type 'app' and subtype 'factory' were found
    #[cfg_attr(
        feature = "std",
        error("Multiple partitions with type 'app' and subtype 'factory' were found")
    )]
    MultipleFactoryPartitions,

    /// Multiple partitions with type 'data' and subtype 'ota' were found
    #[cfg_attr(
        feature = "std",
        error("Multiple partitions with type 'data' and subtype 'ota' were found")
    )]
    MultipleOtadataPartitions,

    /// No partition of type 'app' was found in the partition table
    #[cfg_attr(
        feature = "std",
        error("No partition of type 'app' was found in the partition table")
    )]
    NoAppPartition,

    /// No ned marker was found in the binary data
    #[cfg_attr(feature = "std", error("No ned marker was found in the binary data"))]
    NoEndMarker,

    /// Two partitions are overlapping each other
    #[cfg_attr(
        feature = "std",
        error("Two partitions are overlapping each other: '{0}' and '{1}'")
    )]
    OverlappingPartitions(String, String),

    /// Partition is above the maximum supported size of 16MB
    #[cfg_attr(
        feature = "std",
        error("Partition larger than maximum supported size of 16MB: '{0}'")
    )]
    PartitionTooLarge(String),

    /// The partition is not correctly aligned
    #[cfg_attr(feature = "std", error("The partition is not correctly aligned"))]
    UnalignedPartition,

    /// An error which originated in the `csv` package
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[cfg_attr(feature = "std", error(transparent))]
    CsvError(#[from] csv::Error),

    /// An error which originated in the `deku` package
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[cfg_attr(feature = "std", error(transparent))]
    DekuError(#[from] deku::DekuError),

    /// An error which occurred while trying to convert bytes to a String
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[cfg_attr(feature = "std", error(transparent))]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    /// An error which originated in the `std::io` module
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[cfg_attr(feature = "std", error(transparent))]
    IoError(#[from] std::io::Error),
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
