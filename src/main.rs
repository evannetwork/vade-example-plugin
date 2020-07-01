/*
  Copyright (c) 2018-present evan GmbH.

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
use vade::{Vade, VadePlugin, VadePluginResultValue};

const EXAMPLE_METHOD: &str = "did:example:123456789abcdefghi";
const EXAMPLE_DID_DOCUMENT_STR: &str = r###"{
    "@context": "https://www.w3.org/ns/did/v1",
    "id": "did:example:123456789abcdefghi"
}"###;

pub struct TestPlugin {}

impl TestPlugin {
    pub fn new() -> Self {
        TestPlugin {}
    }
}

impl Default for TestPlugin {
    fn default() -> Self {
        TestPlugin::new()
    }
}

#[async_trait(?Send)]
impl VadePlugin for TestPlugin {
    // test plugin did_resolve just ignores this request
    async fn did_resolve(
        &mut self,
        did: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        if !did.starts_with(EXAMPLE_METHOD) {
            return Ok(VadePluginResultValue::Ignored);
        }
        Ok(VadePluginResultValue::Success(Some(EXAMPLE_DID_DOCUMENT_STR.to_string())))
    }
}

#[tokio::main]
async fn main() {
    let mut vade = Vade::new();
    let tp: TestPlugin = TestPlugin::new();
    vade.register_plugin(Box::from(tp));

    let results = vade.did_resolve("did:example:123456789abcdefghi").await.unwrap();
    let did_document = results[0].as_ref().unwrap();
    println!("{}", did_document);
}
