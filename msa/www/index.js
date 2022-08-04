import * as msa from "../pkg/msa";


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
    
    /**********************************************************************
     * Canvas setup
    */
    let displayCanvas = document.getElementById("display-canvas")
    displayCanvas.width = displayCanvas.clientWidth;
    displayCanvas.height = displayCanvas.clientHeight;

    let ctx = displayCanvas.getContext("2d");
    ctx.font = `${CANVAS_FONT_SIZE}px Arial`;
    ctx.fillStyle = "white";
    ctx.textBaseline = 'middle';
    ctx.textAlign = "center";
    ctx.fillText("Empty canvas", displayCanvas.width/2, displayCanvas.height/2); 


    /**********************************************************************
     * Button listeners
    */
    buttonPlay.onclick = function(e) {
        let fastaString = fieldFasta.value;
        let costmatrixString = fieldCostMatrix.value;
        let gapCost = fieldGapCost.value;
        let maximize = fieldMaximize.value;
        let algorithm = fieldAlgorithm.value;
        let stepInterval = fieldStepInterval.value;
        
        let result = msa.wasm_gusfields(fastaString, costmatrixString, gapCost, maximize)
        
        ctx.clearRect(0, 0, displayCanvas.width, displayCanvas.height);
        
        let baseCoords = {
            width: displayCanvas.width/2,
            height: displayCanvas.height/2 - (result.sequences.length / 2) * CANVAS_FONT_SIZE
        }
        
        ctx.fillText(`The score is ${result.score}`, baseCoords.width, baseCoords.height); 
        
        result.sequences.forEach((s, i) => {
            ctx.fillText(`${s.name}: ${s.sequence}`, baseCoords.width, baseCoords.height + (i+1) * CANVAS_FONT_SIZE); 
        });
    }

})()
