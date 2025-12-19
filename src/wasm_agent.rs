use anyhow::Result;
use wasmtime::*;

use crate::agent::Agent;

pub struct WasmAgent {
    pub id: String,
    store: Store<()>,
    decide_fn: TypedFunc<u64, u64>,
}

impl WasmAgent {
    pub fn load(id: String, wasm_bytes: &[u8]) -> Result<Self> {
        let mut config = Config::new();
        config.consume_fuel(true);
        config.cranelift_opt_level(OptLevel::None);

        let engine = Engine::new(&config)?;
        let mut store = Store::new(&engine, ());

        // Hard fuel cap per match
        store.set_fuel(10_000)?;

        let module = Module::new(&engine, wasm_bytes)?;
        let instance = Instance::new(&mut store, &module, &[])?;

        let decide_fn = instance
            .get_typed_func::<u64, u64>(&mut store, "decide")?;

        Ok(Self {
            id,
            store,
            decide_fn,
        })
    }
}

impl Agent<u64, u64> for WasmAgent {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn decide(&mut self, obs: u64) -> u64 {
        // If fuel is exhausted or WASM traps, agent loses the step
        self.decide_fn.call(&mut self.store, obs).unwrap_or(0)
    }
}
