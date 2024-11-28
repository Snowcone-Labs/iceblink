export function padding(
  a: number | string,
  b?: number | string,
  c?: number | string,
  d?: number | string
) {
  return {
    paddingTop: a,
    paddingRight: b !== undefined ? b : a,
    paddingBottom: c !== undefined ? c : a,
    paddingLeft: d !== undefined ? d : b !== undefined ? b : a,
  };
}
