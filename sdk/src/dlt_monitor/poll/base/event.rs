// Copyright 2022 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::time::Duration;

use super::{BatchError, BatchId};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event<T: BatchId> {
    FetchPending,
    FetchPendingComplete {
        ids: Vec<T>,
    },
    FetchStatuses {
        service_id: String,
        batches: Vec<String>,
    },
    FetchStatusesComplete {
        service_id: String,
        total: usize,
    },
    Update {
        service_id: String,
    },
    UpdateComplete {
        service_id: String,
        total: usize,
    },
    Waiting(Duration),
    Error(BatchError),
}

impl<T: BatchId> Display for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Event::FetchPending => write!(f, "fetching pending batches"),
            Event::FetchPendingComplete { ids } => write!(
                f,
                "found {batches} pending batches",
                batches = ids.len()
            ),
            Event::FetchStatuses { service_id, batches } => {
                write!(
                    f,
                    "fetching {service_id} batch statuses for [{batches}]",
                    batches = batches.join(", ")
                )
            },
            Event::FetchStatusesComplete {
                service_id,
                total,
            } => {
                write!(
                    f,
                    "service {service_id} fetched {total} batch statuses"
                )
            }
            Event::Update { service_id } => {
                write!(
                    f,
                    "updating {service_id} batch statuses"
                )
            },
            Event::UpdateComplete {
                service_id,
                total,
            } => {
                write!(
                    f,
                    "service {service_id} updated {total} batch statuses"
                )
            }
            Event::Waiting(frequency) => {
                write!(f, "waiting at interval {frequency:?}")
            }
            Event::Error(err) => write!(f, "Error: {err:?}"),
        }
    }
}
