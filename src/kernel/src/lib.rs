// Copyright 2021 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An Engula module that provides stateful environment abstractions and
//! implementations.
//!
//! # Abstraction
//!
//! [`Kernel`] is an abstraction to provide a stateful environment to storage
//! engines.
//!
//! # Implementation
//!
//! Some built-in implementations of [`Kernel`]:
//!
//! - [`mem`](crate::mem)
//! - [`file`](crate::file)
//! - [`grpc`](crate::grpc)
//!
//! [`Kernel`]: crate::Kernel

mod error;
mod kernel;
mod manifest;
mod metadata;

pub mod file;
pub mod grpc;
mod local;
pub mod mem;

pub use async_trait::async_trait;
pub use engula_journal::{Event, Journal, Stream, Timestamp};
pub use engula_storage::{Bucket, Storage};

pub type ResultStream<T> = Box<dyn futures::Stream<Item = Result<T>> + Send + Unpin>;

pub use self::{
    error::{Error, Result},
    kernel::{Kernel, KernelUpdate},
    metadata::{Sequence, Version, VersionUpdate},
};

#[cfg(test)]
mod tests {
    use futures::TryStreamExt;

    use crate::*;

    #[tokio::test]
    async fn kernel() -> Result<()> {
        let tmp = tempfile::tempdir()?;

        let kernel = mem::Kernel::open().await?;
        test_kernel(kernel).await?;

        let kernel = file::Kernel::open(tmp.path()).await?;
        test_kernel(kernel).await?;

        Ok(())
    }

    async fn test_kernel(kernel: impl Kernel) -> Result<()> {
        let handle = {
            let mut expect = VersionUpdate {
                sequence: 1,
                ..Default::default()
            };
            expect.add_meta.insert("a".to_owned(), b"b".to_vec());
            expect.remove_meta.push("b".to_owned());
            expect.add_objects.push("a".to_owned());
            expect.remove_objects.push("b".to_owned());
            let mut version_updates = kernel.version_updates(0).await;
            tokio::spawn(async move {
                let update = version_updates.try_next().await.unwrap().unwrap();
                assert_eq!(*update, expect);
            })
        };

        let mut update = KernelUpdate::default();
        update.add_meta("a", "b");
        update.remove_meta("b");
        update.add_object("a");
        update.remove_object("b");
        kernel.apply_update(update).await?;

        handle.await.unwrap();
        Ok(())
    }
}
