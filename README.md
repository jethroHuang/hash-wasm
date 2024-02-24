
## 一个编译为 WebAssembly 的文件 hash 校验库
在开始之前，请先确保您安装了 wasm-pack，如果你没有安装，可以运行

``` shell
cargo install wasm-pack
```

### 打包

``` shell
wasm-pack build --release
```

### 在浏览器中测试

``` shell
wasm-pack test --headless --firefox
```

### 发布到 NPM

``` shell
wasm-pack publish
```
