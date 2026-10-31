

$env:WGPU_BACKEND="gl"; cargo test -p egui_kittest --test tests --all-features -- test_shrink --exact --nocapture

$env:WGPU_BACKEND="vulkan"; cargo test -p egui_kittest --test tests --all-features -- test_shrink --exact --nocapture


$env:WGPU_BACKEND="dx12"; cargo test -p egui_demo_lib  rendering_test  //Error reported
$env:WGPU_BACKEND="gl"; cargo test -p egui_demo_lib  rendering_test
$env:WGPU_BACKEND="vulkan"; cargo test -p egui_demo_lib  rendering_test



$env:WGPU_BACKEND="vulkan"; cargo test -p rendering_test_sim2 --test tests --all-features -- test_shrink --exact --nocapture
