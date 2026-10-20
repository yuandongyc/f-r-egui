

$env:WGPU_BACKEND="gl"; cargo test -p egui_kittest --test tests --all-features -- test_shrink --exact --nocapture
