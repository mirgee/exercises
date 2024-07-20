import { all_paths_source_target } from '../src'

test('t0', () => {
  const expected = [[0,1,3],[0,2,3]]
  const actual = all_paths_source_target([[1,2],[3],[3],[]])
  expected.sort();
  actual.sort();
  expect(actual).toEqual(expected)
})

test('t1', () => {
  const expected = [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
  const actual = all_paths_source_target([[4,3,1],[3,2,4],[3],[4],[]])
  expected.sort();
  actual.sort();
  expect(actual).toEqual(expected)
})
