import test from 'ava';

import { now } from '../index.js';

test('now() returns bigint', (t) => {
  t.is(typeof now(), 'bigint');
});