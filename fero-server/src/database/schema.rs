// Copyright 2018 CoreOS, Inc.
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

table! {
    secrets (id) {
        id -> Integer,
        key_id -> BigInt,
        threshold -> Integer,
        hsm_id -> Integer,
    }
}

table! {
    user_secret_weights (id) {
        id -> Integer,
        secret_id -> Integer,
        user_id -> Integer,
        weight -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        key_id -> BigInt,
        key_data -> Binary,
    }
}

joinable!(user_secret_weights -> secrets (secret_id));
joinable!(user_secret_weights -> users (user_id));

allow_tables_to_appear_in_same_query!(
    secrets,
    user_secret_weights,
    users,
);
