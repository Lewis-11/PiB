import "bulma";
import "./index.css";
import * as msa from "../pkg/msa";
import * as CONST from './constants';

(() => {

	const CANVAS_FONT_SIZE = 32

	/**********************************************************************
	 * HTML elements
	 */
	let buttonPlay = document.getElementById("button-play");
	let fieldFasta = document.getElementById("field-fasta");
	let fieldCostMatrix = document.getElementById("field-cost-matrix");
	let fieldGapCost = document.getElementById("field-gap-cost");
	let fieldAlgorithm = document.getElementById("field-algorithm");
	let fieldMaximize = document.getElementById("field-maximize");
	let fieldStepInterval = document.getElementById("field-step-interval");
	let helpFastaFormat = document.getElementById("help-fasta-format");
	let helpCostMatrixFormat = document.getElementById("help-cost-matrix-format"); 
	let helpFastaExample = document.getElementById("help-fasta-example");
	let helpCostMatrixExample = document.getElementById("help-cost-matrix-example");
	let selectorFasta = document.getElementById("selector-fasta");
	let selectorCostMatrix = document.getElementById("selector-cost-matrix");
	let fileSelectorFasta = document.getElementById("file-selector-fasta");
	let fileSelectorCostMatrix = document.getElementById("file-selector-cost-matrix");
	let nameSelectedFasta = document.getElementById("name-selected-fasta-file");
	let nameSelectedCostMatrix = document.getElementById("name-selected-cost-matrix-file");

	/**********************************************************************
	 * Canvas setup
	 */
	let displayCanvas = document.getElementById("display-canvas")
	displayCanvas.width = displayCanvas.clientWidth;
	displayCanvas.height = displayCanvas.clientHeight;

	let ctx = displayCanvas.getContext("2d");
	ctx.font = `${CANVAS_FONT_SIZE}px Courier New`;
	ctx.fillStyle = "white";
	ctx.textBaseline = 'middle';
	ctx.textAlign = "center";
	ctx.fillText("Empty canvas", displayCanvas.width / 2, displayCanvas.height / 2);

	/**********************************************************************
	 * Utils
	 */
	function readFileAsync(file) {
		return new Promise((resolve, reject) => {
			let reader = new FileReader();

			reader.onload = () => {
				resolve(reader.result);
			};

			reader.onerror = reject;

			reader.readAsText(file);
		})
	}

	/**********************************************************************
	 * Button listeners
	 */
	buttonPlay.onclick = async function (e) {
		let fastaString;
		let costmatrixString;
		let gapCost = fieldGapCost.value;
		let maximize = fieldMaximize.value;
		let algorithm = fieldAlgorithm.value;
		let stepInterval = fieldStepInterval.value;

		// Retrieve text from files (or not)
		if (fileSelectorFasta.files.length > 0) {
			fastaString = await readFileAsync(fileSelectorFasta.files[0])
		} else {
			fastaString = fieldFasta.value;
		}

		if (fileSelectorCostMatrix.files.length > 0) {
			costmatrixString = await readFileAsync(fileSelectorCostMatrix.files[0])
		} else {
			costmatrixString = fieldCostMatrix.value;
		}

		let result = msa.wasm_gusfields(fastaString, costmatrixString, gapCost, maximize)

		ctx.clearRect(0, 0, displayCanvas.width, displayCanvas.height);

		let baseCoords = {
			width: displayCanvas.width / 2,
			height: displayCanvas.height / 2 - (result.sequences.length / 2) * CANVAS_FONT_SIZE
		}

		ctx.fillText(`The score is ${result.score}`, baseCoords.width, baseCoords.height);

		result.sequences.forEach((s, i) => {
			ctx.fillText(`${s.name}: ${s.sequence}`, baseCoords.width, baseCoords.height + (i + 1) * CANVAS_FONT_SIZE);
		});
	}

	/**********************************************************************
	 * OnDrop & OnDrag
	 */
	selectorFasta.ondrop = function (e) {
		let dropItems = e.dataTransfer.items
		e.preventDefault();
		// Has the user dropped a single file?
		if (dropItems && dropItems.length == 1 && dropItems[0].kind === 'file') {
			const file = dropItems[0].getAsFile()
			// Is the file a FASTA file?
			if (file.name.match(/\.fasta$/i)) {
				file.text().then(v => fieldFasta.value = v)
			} else {
				alert("You sure that's a FASTA file?")
			}
		}
		this.classList.remove("component-has-drop")
	}

	selectorFasta.ondragenter = function (_e) {
		console.log("file has been dragged inside the container")
		this.classList.add("component-has-drop")
	}

	selectorFasta.ondragleave = function (_e) {
		console.log("file has been dragged outside the container")
		this.classList.remove("component-has-drop")
	}

	selectorCostMatrix.ondrop = function (e) {
		let dropItems = e.dataTransfer.items
		e.preventDefault();
		// Has the user dropped a single file?
		if (dropItems && dropItems.length == 1 && dropItems[0].kind === 'file') {
			const file = dropItems[0].getAsFile()
			// Is the file a TXT file?
			if (file.name.match(/\.txt$/i)) {
				file.text().then(v => fieldCostMatrix.value = v)
			} else {
				alert("You sure that's a TXT file?")
			}
		}
		this.classList.remove("component-has-drop")
	}

	selectorCostMatrix.ondragenter = function (_e) {
		console.log("file has been dragged inside the container")
		this.classList.add("component-has-drop")
	}

	selectorCostMatrix.ondragleave = function (_e) {
		console.log("file has been dragged outside the container")
		this.classList.remove("component-has-drop")
	}

	/**********************************************************************
	 * FASTA & Cost matrix file selectors
	 */
	selectorFasta.onclick = function (_e) {
		fileSelectorFasta.click();
	}

	selectorCostMatrix.onclick = function (_e) {
		fileSelectorCostMatrix.click();
	}

	/**********************************************************************
	 * <input type="file"> updates
	 */
	function updateUI2ReflectFileChange(input, field, selector, uiName) {
		let icon = selector.querySelector("em");
		// A file is selected
		if (input.files.length > 0) {
			let nameToDisplay = input.files[0].name
			icon.classList.replace("fa-regular", "fa-solid");
			uiName.style.display = "block";

			if (nameToDisplay.length > 20) {
				nameToDisplay = nameToDisplay.substring(0, 17) + "..."
			}

			uiName.querySelector(".name-selected-value").innerText = nameToDisplay
			field.disabled = true
			// A file is no longer selected
		} else {
			icon.classList.replace("fa-solid", "fa-regular");
			uiName.style.display = "none";
			field.disabled = false
		}
	}

	fileSelectorFasta.onchange = function (_e) {
		updateUI2ReflectFileChange(fileSelectorFasta, fieldFasta, selectorFasta, nameSelectedFasta);
	}

	fileSelectorCostMatrix.onchange = function (_e) {
		updateUI2ReflectFileChange(fileSelectorCostMatrix, fieldCostMatrix, selectorCostMatrix, nameSelectedCostMatrix);
	}

	/**********************************************************************
	 * Current file label listener
	 */
	nameSelectedFasta.onclick = function(e) {
		fileSelectorFasta.value = "" 
		updateUI2ReflectFileChange(fileSelectorFasta, fieldFasta, selectorFasta, nameSelectedFasta);
		e.stopPropagation();
	}

	nameSelectedCostMatrix.onclick = function(e) {
		fileSelectorCostMatrix.value = "" 
		updateUI2ReflectFileChange(fileSelectorCostMatrix, fieldCostMatrix, selectorCostMatrix, nameSelectedCostMatrix);
		e.stopPropagation();
	}

	/**********************************************************************
	 * Help functionality
	 */
	helpFastaFormat.onclick = function (_e) {
		alert("NOT IMPLEMENTED")
	}

	helpFastaExample.onclick = function (_e) {
		if (!fieldFasta.disabled) {
			fieldFasta.value = CONST.FASTA_EXAMPLE_VALUE
		}
	}

	helpCostMatrixFormat.onclick = function (_e) {
		alert("NOT IMPLEMENTED")
	}

	helpCostMatrixExample.onclick = function (_e) {
		if (!fieldCostMatrix.disabled) {
			fieldCostMatrix.value = CONST.COST_MATRIX_EXAMPLE_VALUE
		}
	}

})()
