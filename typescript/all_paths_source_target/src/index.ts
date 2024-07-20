// https://leetcode.com/problems/all-paths-from-source-to-target/description/

export function all_paths_source_target(graph: number[][]): number[][] {
  const end = graph.length - 1;
  const res: number[][] = [];

  function dfs(curr_node: number, path: number[]) {
    for (const neighbor of graph[curr_node]) {
      const p = path.slice();
      p.push(neighbor);
      if (neighbor === end) {
        res.push(p)
      } else {
        dfs(neighbor, p);
      }
    }
  }

  dfs(0, [0]);

  return res;
}
