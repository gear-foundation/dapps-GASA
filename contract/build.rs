use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use rockets_game_io::ContractMetadata;

fn main() {
    WasmBuilder::with_meta(ContractMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}
