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

use std::collections::HashMap;

use tokio::sync::Mutex;

use crate::{error::Result, proto::*};

pub struct Control {
    inner: Mutex<Inner>,
}

impl Default for Control {
    fn default() -> Self {
        let inner = Inner::default();
        Self {
            inner: Mutex::new(inner),
        }
    }
}

#[derive(Default)]
struct Inner {
    nodes: HashMap<String, NodeDesc>,
}

impl Control {
    pub async fn desc(&self) -> ControlDesc {
        ControlDesc::default()
    }

    pub async fn list_nodes(&self) -> Result<NodeDescList> {
        let inner = self.inner.lock().await;
        Ok(inner.nodes.values().cloned().collect())
    }
}