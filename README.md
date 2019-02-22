# Заготовка для создания php расширений на языке Rust

## Компиляция расширения

```bash
RUSTFLAGS='-C prefer-dynamic' cargo build --lib --release -Z unstable-options --out-dir=build
```

## Проверка работоспособности
```bash
php -d extension=./build/librssystems.so -f hello.php
```
