# @k0michi/now

@k0michi/now provides a function to get the current UNIX time in nanoseconds, using [NAPI-RS](https://napi.rs/).

This module relies on Rust's `std::time::SystemTime::now()`.

## Installation
```bash
npm install @k0michi/now
yarn add @k0michi/now
```

## Usage
```js
import { now } from '@k0michi/now';

console.log(await now());
// 1667912108734934000n
```

## Known issue
- The resolution is microsecond on macOS

## License
MIT