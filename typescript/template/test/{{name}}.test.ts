import { {{ name }} } from '../src'

test('{{ name }} test 0', () => {
  const expected = null;
  const actual = {{ name }}()
  expect(actual).toEqual(expected)
})
