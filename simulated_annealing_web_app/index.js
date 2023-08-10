import init, { Grid } from "./pkg/rust_annealing.js"

const wasm = await init()
console.log(wasm)

let grid = Grid.new()
console.log(grid)
console.log(grid.advance())
console.log(grid.advance())
console.log(grid.advance())
console.log(grid.advance())
console.log(grid.return_state())
