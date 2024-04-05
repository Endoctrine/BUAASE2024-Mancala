import assert from "assert";

// Choose proper "import" depending on your PL.
// import { mancalaOperator as op1 } from "./t3-2-as/build/release.js";
import { mancala_operator as op1 } from "./t3_2_rust/pkg/t3_2_rust.js"
// [Write your own "import" for other PLs.]

// Choose proper "import" depending on your PL.
// import { mancalaOperator as op2 } from "./t3-2-as-rival/build/release.js";
import { mancala_operator as op2 } from "./t3_2_rust_rival/pkg/t3_2_rust.js"
// [Write your own "import" for other PLs.]

// Choose proper "import" depending on your PL.
// import { mancalaBoard as board } from "./t3-1-as/build/release.js";
import { mancala_board as board } from "./t3_1_rust/pkg/t3_1_rust.js"
// [Write your own "import" for other PLs.]

let operator, status, operation, operationSequence, boardReturn, isEnded;
let op1Result = 0, op2Result = 0;
let op1Time = 0, op2Time = 0, timeStamp = 0;

// Firstly, start from op1.
operator = 1;
status = [4,4,4,4,4,4,0,4,4,4,4,4,4,0];
operation = 0;
operationSequence = [];
isEnded = false;

do {
    if (operator == 1) {
        timeStamp = performance.now() * 1000;
        operation = op1(1, status);
        op1Time += performance.now() * 1000 - timeStamp;
        operationSequence.push(operation);
        boardReturn = board(1, operationSequence, operationSequence.length);
    } else {
        timeStamp = performance.now() * 1000;
        operation = op2(2, status);
        op2Time += performance.now() * 1000 - timeStamp;
        operationSequence.push(operation);
        boardReturn = board(2, operationSequence, operationSequence.length);
    }
    if (boardReturn[14] == 1) {
        operator = 1;
        status = boardReturn.slice(0,14);
    } else if (boardReturn[14] == 2) {
        operator = 2;
        status = boardReturn.slice(0,14);
    } else {
        isEnded = true;
        op1Result += boardReturn[14] - 200;
        op2Result -= boardReturn[14] - 200;
    }
} while (!isEnded);

// Now change to start from op2.
operator = 2;
status = [4,4,4,4,4,4,0,4,4,4,4,4,4,0];
operation = 0;
operationSequence = [];
isEnded = false;

do {
    if (operator == 1) {
        timeStamp = performance.now() * 1000;
        operation = op1(1, status);
        op1Time += performance.now() * 1000 - timeStamp;
        operationSequence.push(operation);
        boardReturn = board(1, operationSequence, operationSequence.length);
    } else {
        timeStamp = performance.now() * 1000;
        operation = op2(2, status);
        op2Time += performance.now() * 1000 - timeStamp;
        operationSequence.push(operation);
        boardReturn = board(2, operationSequence, operationSequence.length);
    }
    if (boardReturn[14] == 1) {
        operator = 1;
        status = boardReturn.slice(0,14);
    } else if (boardReturn[14] == 2) {
        operator = 2;
        status = boardReturn.slice(0,14);
    } else {
        isEnded = true;
        op1Result += boardReturn[14] - 200;
        op2Result -= boardReturn[14] - 200;
    }
} while (!isEnded);
 
op1Time = op1Time / 1000;
op2Time = op2Time / 1000;

console.log("🎉 Finished battle, result: " + op1Result + ":" + op2Result + ".");
console.log("⏰ Processing Time: " + op1Time + ":" + op2Time + ".");