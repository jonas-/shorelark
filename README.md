# Shorelark

Based on the very nice tutorial **[Learning to Fly](https://pwy.io/posts/learning-to-fly-pt1/)** - [Github](https://github.com/Patryk27/shorelark)

Small modifications to the tutorial:
- Use [trunk](https://trunkrs.dev) instead of npm/wasm-pack
    - Allows a cleaner and simpler rust interface with no abstraction library *simulation-wasm*
    - Overcame many obstacles with versions I faced
    - More complicated interaction with Javascript - maybe use additionally [yew](https://yew.rs) or so
- Used `f64`instad of `f32`    