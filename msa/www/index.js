import "bulma";
import "./index.css";
import * as msa from "../pkg/msa";
import * as CONST from './constants';
import "./relativeplugin";

(() => {
	
	let paused = false
	let stepEnabled = false
	let stepRequest = false
	let requestStop = false
	let visRunning = false

	/**********************************************************************
	 * HTML elements
	 */
	let buttonPlay = document.getElementById("button-play");
	let buttonToggle = document.getElementById("button-toggle");
	let buttonStep = document.getElementById("button-step");
	let buttonReset = document.getElementById("button-reset");
	let fieldFasta = document.getElementById("field-fasta");
	let fieldCostMatrix = document.getElementById("field-cost-matrix");
	let fieldGapCost = document.getElementById("field-gap-cost");
	let fieldAlgorithm = document.getElementById("field-algorithm");
	let fieldMaximize = document.getElementById("field-maximize");
	let fieldStepInterval = document.getElementById("field-step-interval");
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
	const ratio = window.devicePixelRatio;
	let canvas = document.getElementById("display-canvas")
    canvas.style.width = canvas.clientWidth + "px";
    canvas.style.height = canvas.clientHeight + "px";
	canvas.width = canvas.clientWidth * ratio
	canvas.height = canvas.clientHeight * ratio
    canvas.getContext("2d").scale(ratio, ratio);

	let stage = new createjs.Stage("display-canvas")
	createjs.RelativePlugin.install();
	createjs.Ticker.setFPS(60);
	createjs.Ticker.addEventListener("tick", stage);

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
	
	function clusterStepParser(stringToParse) {
		let result = []
		let stepsAndScore = stringToParse.split(CONST.DELIMITER_STEP)
		
		for (let x = 0; x < stepsAndScore.length - 1; x += 1) {
			let clusters = stepsAndScore[x].split(CONST.DELIMITER_CLUSTER)
			result.push([
				clusters[0].split(CONST.DELIMITER_LINE),
				clusters[1].split(CONST.DELIMITER_LINE),
				clusters[2].split(CONST.DELIMITER_LINE),
			])
		}
		
		return {
			steps: result,
			score: stepsAndScore[stepsAndScore.length - 1]
		}
	}

	function sleep(ms) {
		return new Promise(resolve => setTimeout(resolve, ms));
	}
	
	function createBaseInCanvas(letter, fontSize, x, y) {
		let base = new createjs.Text(letter, fontSize + "px Courier", "#FFFFFF");
		base.x = x; base.y = y;
		base.baseline = "middle"
		base.textAlign = "center"
		return base
	}
	
	function createPerCharacterUiAlignment(alignment, fontSize) {
		let uiElements = []
		for (let i = 0; i < alignment.data.length; i++) {
			uiElements.push([])
			for (let j = 0; j < alignment.data[0].length; j++) {
				let xLetterShift = fontSize * CONST.X_CHAR_SHIFT_MULTIPLIER;
				let yLetterShift = fontSize * CONST.Y_CHAR_SHIFT_MULTIPLIER;
				let x = alignment.pos.x + xLetterShift * j; let y = alignment.pos.y + yLetterShift * i;
				let base = createBaseInCanvas(alignment.data[i][j], fontSize, x, y)
				uiElements[i].push(base)
				stage.addChild(base)
			}
		}
		return uiElements
	}
	
	function createPerLineUiAlignment(alignment, fontSize) {
		let uiElements = []
		for (let i = 0; i < alignment.data.length; i++) {
			let yLetterShift = fontSize * CONST.Y_CHAR_SHIFT_MULTIPLIER;
			let x = alignment.pos.x; let y = alignment.pos.y + yLetterShift * i;
			let base = createBaseInCanvas(alignment.data[i], fontSize, x, y)
			uiElements.push(base)
			stage.addChild(base)
		}
		return uiElements
	}

	/**********************************************************************
	 * Visualization
	 */
	async function visualization() {
		
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
		
		stage.removeAllChildren();
		
		let result = msa.msa_wasm(fastaString, costmatrixString, gapCost, maximize, algorithm)
		let parsedResult = clusterStepParser(result)

		let offScreenXOffset = canvas.clientWidth * ratio;
		let offScreenYOffset = canvas.clientHeight * ratio;
		
		let alignmentCluster1, alignmentCluster2, alignmentResultingCluster
		
		visRunning = true;

		let uiStep = new createjs.Text("Step 1/" + parsedResult.steps.length, "28px Courier", "#000000");
		uiStep.baseline = "middle"
		uiStep.textAlign = "center"
		uiStep.x = canvas.clientWidth * ratio - 200; uiStep.y = canvas.clientHeight * ratio - 100;
		uiStep.alpha = 0;
		
		var rect = new createjs.Shape();
		rect.graphics.beginFill('white');
		rect.graphics.drawRoundRectComplex(uiStep.x - 90, uiStep.y - 10, 200, 50, 0.25, 0.25, 0.25, 0.25);
		rect.graphics.endFill();
		rect.alpha = 0;

		stage.addChild(rect)
		stage.addChild(uiStep)

		createjs.Tween.get(uiStep).to({alpha:1}, stepInterval);
		createjs.Tween.get(rect).to({alpha:1}, stepInterval);
		
		let uiAlignmentResultingCluster
		
		for (let idx = 0; idx < parsedResult.steps.length; idx += 1) {

			if (requestStop) { stage.removeAllChildren(); break; }
			while (paused) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
			
			const curr = parsedResult.steps[idx]
			
			// Draw current step
			uiStep.text = "Step " + (idx+1) + "/" + parsedResult.steps.length;
			
			// Compute the font size and (x,y) for each alignemnt.
			// Big alignments *should* be able to fit.
			let fontSizeCluster1 = Math.floor(canvas.clientWidth / ratio * 0.8 / curr[0][0].length) * 4 * ratio;
			let fontSizeCluster2 = Math.floor(canvas.clientWidth / ratio * 0.8 / curr[1][0].length) * 4 * ratio;
			let fontSizeResultingCluster = Math.floor(canvas.clientWidth / ratio * 0.8 / curr[2][0].length) * 4 * ratio;

			// Make sure font size isn't too big
			fontSizeCluster1 = Math.min(16 * ratio, fontSizeCluster1);
			fontSizeCluster2 = Math.min(16 * ratio, fontSizeCluster2);
			fontSizeResultingCluster = Math.min(16 * ratio, fontSizeResultingCluster);
			
			alignmentCluster1 = {pos: {x: (canvas.clientWidth / 2) * ratio, y: canvas.clientHeight * 0.33 * ratio + offScreenYOffset}, data: curr[0]}
			alignmentCluster2 = {pos: {x: (canvas.clientWidth / 2) * ratio, y: canvas.clientHeight * 0.66 * ratio + offScreenYOffset}, data: curr[1]}
			alignmentResultingCluster = {pos: {x: (canvas.clientWidth / 2) * ratio + offScreenXOffset, y: (canvas.clientHeight / 2) * ratio}, data: curr[2]}
	
			let uiAlignmentCluster1 = createPerLineUiAlignment(alignmentCluster1, fontSizeCluster1)
			let uiAlignmentCluster2 = createPerLineUiAlignment(alignmentCluster2, fontSizeCluster2)
			uiAlignmentResultingCluster = createPerLineUiAlignment(alignmentResultingCluster, fontSizeResultingCluster)
			
			alignmentCluster1.pos.y -= offScreenYOffset
			for (const row of uiAlignmentCluster1) {
				while (paused) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
				createjs.Tween.get(row, { loop: false })
					.to({ y: "-" + offScreenYOffset }, stepInterval, createjs.Ease.getPowInOut(4))
			}
	
			alignmentCluster2.pos.y -= offScreenYOffset
			for (const row of uiAlignmentCluster2) {
				while (paused) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
				createjs.Tween.get(row, { loop: false })
					.to({ y: "-" + offScreenYOffset }, stepInterval, createjs.Ease.getPowInOut(4))
			}
	
			await sleep(stepInterval)
	
			alignmentCluster1.pos.x -= offScreenXOffset // + alignmentCluster1.data[0].length * 8
			for (const row of uiAlignmentCluster1) {
				while (paused) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
				createjs.Tween.get(row, { loop: false })
					.to({ x: "-" + offScreenXOffset }, stepInterval, createjs.Ease.getPowInOut(4))
			}
	
			alignmentCluster2.pos.x -= offScreenXOffset // + alignmentCluster2.data[0].length * 8
			for (const row of uiAlignmentCluster2) {
				while (paused) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
				createjs.Tween.get(row, { loop: false })
					.to({ x: "-" + offScreenXOffset }, stepInterval, createjs.Ease.getPowInOut(4))
			}
	
			alignmentResultingCluster.pos.x -= offScreenXOffset
			for (const row of uiAlignmentResultingCluster) {
				while (paused) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
				createjs.Tween.get(row, { loop: false })
					.to({ x: "-" + offScreenXOffset }, stepInterval, createjs.Ease.getPowInOut(4))
			}
	
			await sleep(stepInterval)
			
			while (stepEnabled && !stepRequest) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
			stepRequest = false;
	
			var lastStepOffset = offScreenYOffset

			// Last index is different
			if (idx == parsedResult.steps.length - 1) {
				lastStepOffset = alignmentResultingCluster.pos.y - 100
			}

			alignmentResultingCluster.pos.y -= lastStepOffset
			for (const row of uiAlignmentResultingCluster) {
				while (paused) { await sleep(CONST.PAUSE_SLEEP_TIMEOUT) }
				createjs.Tween.get(row, { loop: false })
					.to({ y: "-" + lastStepOffset }, stepInterval, createjs.Ease.getPowInOut(4))
			}
			
		}

		visRunning = false;
		stepEnabled = false;
		
		await sleep(stepInterval)

		let uiScore = new createjs.Text("Score: " + parsedResult.score, 24 * ratio + "px Courier", "#FFFFFF");
		uiScore.baseline = "middle"
		uiScore.textAlign = "center"
		uiScore.x = offScreenXOffset / 2; uiScore.y = uiAlignmentResultingCluster[uiAlignmentResultingCluster.length - 1].y + 50 * ratio
		uiScore.alpha = 0;
		stage.addChild(uiScore)
		createjs.Tween.get(uiScore).to({alpha:1}, stepInterval);
		createjs.Tween.get(uiStep).to({alpha:0}, stepInterval);
		createjs.Tween.get(rect).to({alpha:0}, stepInterval);
	}

	/**********************************************************************
	 * Button listeners
	 */
	buttonPlay.onclick = function (e) {
		if (stepEnabled) {
			stepEnabled = false;
		} else if (visRunning) {
			return;
		} else {
			visualization()
		}
	}
	
	buttonToggle.onclick = function (e) {
		createjs.Ticker.paused = !createjs.Ticker.paused
		paused = createjs.Ticker.paused
	}
	
	buttonStep.onclick = function (e) {
		stepEnabled = stepRequest = true;

		if (visRunning == false) {
			stepRequest = false;
			visualization()
		}
		
	}
	
	buttonReset.onclick = function (e) {
		if (visRunning) {
			requestStop = true;
		} else {
			stage.removeAllChildren();
		}
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

	helpFastaExample.onclick = function (_e) {
		if (!fieldFasta.disabled) {
			fieldFasta.value = CONST.FASTA_EXAMPLE_VALUE
		}
	}

	helpCostMatrixExample.onclick = function (_e) {
		if (!fieldCostMatrix.disabled) {
			fieldCostMatrix.value = CONST.COST_MATRIX_EXAMPLE_VALUE
		}
	}

})()
