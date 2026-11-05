cargo-show-asm

//View rust assembly code online.
https://godbolt.org/


$env:WGPU_BACKEND="gl"; cargo test -p egui_kittest --test tests --all-features -- test_shrink --exact --nocapture

$env:WGPU_BACKEND="vulkan"; cargo test -p egui_kittest --test tests --all-features -- test_shrink --exact --nocapture


$env:WGPU_BACKEND="dx12"; cargo test -p egui_demo_lib  rendering_test  //Error reported
$env:WGPU_BACKEND="gl"; cargo test -p egui_demo_lib  rendering_test
$env:WGPU_BACKEND="vulkan"; cargo test -p egui_demo_lib  rendering_test



$env:WGPU_BACKEND="vulkan"; cargo test -p rendering_test_sim2 --test tests --all-features -- test_shrink --exact --nocapture



//Use there skills for Rust
//https://github.com/mattpocock/skills
1. /setup-matt-pocock-skills      # 每 repo 一次，选好 issue 跟踪方式
2. /grill-with-docs               # 讲清楚这次要改什么、不改什么
3. /domain-model                  # 推敲核心 struct / trait 边界
4. /to-prd + /to-issues           # 拆成小工单，每个工单是一个垂直切片
5. /tdd                           # 每个切片：先写 #[test]，再写 impl
6. cargo clippy && cargo test     # 你自己跑，或让 Agent 跑完再提交
7. /code-review                   # 按仓库规范+PRD 做双轴评审
8. 每周一次 /improve-codebase-architecture
