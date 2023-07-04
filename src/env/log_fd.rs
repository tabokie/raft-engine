// Copyright (c) 2017-present, PingCAP, Inc. Licensed under Apache-2.0.

#[cfg(not(any(target_os = "windows", feature = "std_fs")))]
mod unix;
#[cfg(not(any(target_os = "windows", feature = "std_fs")))]
pub use unix::LogFd;

#[cfg(any(target_os = "windows", feature = "std_fs"))]
mod plain;
#[cfg(any(target_os = "windows", feature = "std_fs"))]
pub use plain::LogFd;
