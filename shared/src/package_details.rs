use crate::package;
pub struct Details {
    package: package::Package,
    version: String,
    binary_file: String,
    package_size: String,
    checksum: String
}