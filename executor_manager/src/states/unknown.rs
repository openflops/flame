/*
Copyright 2023 The Flame Authors.
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use async_trait::async_trait;

use crate::executor::Executor;
use crate::states::State;

use common::ctx::FlameContext;
use common::{trace::TraceFn, trace_fn, FlameError};

#[derive(Clone)]
pub struct UnknownState {
    pub executor: Executor,
}

#[async_trait]
impl State for UnknownState {
    async fn execute(&mut self, _ctx: &FlameContext) -> Result<Executor, FlameError> {
        trace_fn!("UnknownState::execute");

        Ok(self.executor.clone())
    }
}
