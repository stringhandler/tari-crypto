

use wasm_bindgen::prelude::*;
use crate::script;
use crate::script::{ScriptError, ScriptContext, ExecutionStack, ExecutionTraceStep, Opcode};
use serde::Serialize;

#[wasm_bindgen]
pub struct TariScript {
}

#[wasm_bindgen]
impl TariScript {
    pub fn new() -> Self {
        Self {

        }
    }
  pub fn hello(&self) -> String {
      "Hello world".to_string()
  }

    pub fn execute(&mut self, op_codes: &[u8], input: &[u8]) -> Result<JsValue, JsValue> {
        let codes =  script::Opcode::parse(op_codes)?;
        let s = script::TariScript::new(codes);
        let input =  script::ExecutionStack::from_bytes(input) ?;
        let result =  s.execute_with_context(&input, &ScriptContext::default()) ?;
        Ok(JsValue::from_serde(&result).unwrap())

    }

    pub fn trace(&mut self, op_codes: &[u8], input: &[u8]) -> Result<JsValue, JsValue> {
        let codes =  script::Opcode::parse(op_codes)?;
        let s = script::TariScript::new(codes);
        let input =  script::ExecutionStack::from_bytes(input) ?;
        let result =  s.trace_with_context(&input, &ScriptContext::default()) ?;


        let result :Vec<TraceResult> = result.into_iter().map(|(op, res) | TraceResult{ op_code: op.to_string(), step_result: res}).collect();
        Ok(JsValue::from_serde(&result).unwrap())
    }
}

impl From<script::ScriptError> for JsValue {
    fn from(source: ScriptError) -> Self  {
        source.to_string().into()
    }
}

impl From<script::ExecutionStack> for JsValue {
    fn from(source: script::ExecutionStack) -> Self {
        Self::from_serde(&source).unwrap()
    }
}

impl From<script::ExecutionTraceStep> for JsValue {
    fn from(source: script::ExecutionTraceStep) -> Self {
        Self::from_serde(&source).unwrap()
    }
}

#[derive(Serialize)]
struct TraceResult {
    op_code: String,
    step_result: ExecutionTraceStep
}




