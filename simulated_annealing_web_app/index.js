import init, { Grid } from "./pkg/rust_annealing.js"

const wasm = await init()

let grid = Grid.new()

let intervalID = 0;
let sim_on = false;
function toggleSimulation() {
  if (sim_on) {
    sim_on = false
    clearInterval(intervalID)
  } else {
    sim_on = true
    intervalID = setInterval(advance,(minimumTimePerFrame/simulationStepsPerFrame)*1000)
  }
}

let simulationStepsPerFrame = 20; //always overwritten by the value in the html file
function changeStepsPerFrame() {
	simulationStepsPerFrame = document.getElementById("stepsPerFrame").value
	toggleSimulation()
	toggleSimulation()
}

let minimumTimePerFrame = 0.1
function changeMinTimePerFrame() {
	minimumTimePerFrame = document.getElementById("minTimePerFrame").value
	toggleSimulation()
	toggleSimulation()
}

function updateView() {
	let states = grid.return_state() 

	for (let i = 0; i < states.length; i++) {
		let state = states[i]
		let square = document.getElementById('reportingBox'+i)
		if (state > 0 && square.matches('.spinDown')) {
			square.classList.remove('spinDown')
			square.classList.add('spinUp')
		}
		if (state < 0 && square.matches('.spinUp')) {
			square.classList.remove('spinUp')
			square.classList.add('spinDown')
		}
	}
}

let simulationStep = 0;
function advance() {
	console.log(grid.advance())
	simulationStep = simulationStep + 1
	if (simulationStep%simulationStepsPerFrame == 0) {
		updateView()
	}
	
}

document.getElementById('Start/Stop Button').addEventListener('click',toggleSimulation)
document.getElementById('stepsPerFrame').addEventListener('input',changeStepsPerFrame)
document.getElementById('minTimePerFrame').addEventListener('input',changeMinTimePerFrame)


updateView()
changeStepsPerFrame()
changeMinTimePerFrame()