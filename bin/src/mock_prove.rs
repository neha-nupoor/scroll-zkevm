use ethers_providers::{Http, Provider};
use std::env;
use types::eth::BlockTrace;
use zkevm::circuit::SuperCircuit;
use zkevm::prover::Prover;

const DEFAULT_BEGIN_BATCH: i64 = 1;
const DEFAULT_END_BATCH: i64 = i64::MAX;

#[tokio::main]
async fn main() {
    log::info!("mock_prove: begin");

    dotenv::dotenv().ok();
    env_logger::init();

    let setting = Setting::new();
    log::info!("mock_prove: {setting:?}");

    let provider = Provider::<Http>::try_from(setting.scroll_api_url)
        .expect("mock_prove: failed to initialize ethers Provider");

    // TODO: need to test interrupted condition for the last block batch.
    for i in setting.begin_batch..=setting.end_batch {
        let block_traces: Vec<BlockTrace> = provider
            .request("l2_getTracesByBatchIndex", i)
            .await
            .expect("mock_prove: failed to request l2_getTracesByBatchIndex with params [{i}]");

        match Prover::mock_prove_target_circuit_multi::<SuperCircuit>(&block_traces, true) {
            Ok(_) => log::info!("mock_prove: succeeded to prove batch-{i}"),
            Err(err) => log::error!("mock_prove: failed to prove batch-{i}:\n{err:?}"),
        }
    }

    log::info!("move_prove: end");
}

#[derive(Debug)]
struct Setting {
    begin_batch: i64,
    end_batch: i64,
    scroll_api_url: String,
}

impl Setting {
    pub fn new() -> Self {
        let scroll_api_url =
            env::var("SCROLL_API_URL").expect("mock_prove: Must set env SCROLL_API_URL");

        let begin_batch = env::var("BEGIN_BATCH")
            .ok()
            .and_then(|n| n.parse().ok())
            .unwrap_or(DEFAULT_BEGIN_BATCH);
        let end_batch = env::var("END_BATCH")
            .ok()
            .and_then(|n| n.parse().ok())
            .unwrap_or(DEFAULT_END_BATCH);

        Self {
            begin_batch,
            end_batch,
            scroll_api_url,
        }
    }
}
