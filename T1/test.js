import assert from "assert";

// Choose proper "import" depending on your PL.
// import { bocchiShutUp } from "./t1-as/build/release.js";
import { bocchi_shut_up as bocchiShutUp } from "./t1_rust/pkg/t1_rust.js"
// [Write your own "import" for other PLs.]

assert.strictEqual(bocchiShutUp(1,[13,14,15,21,11,16],6),10);
assert.strictEqual(bocchiShutUp(2,[13,14,13,11,21,13,21,22],8),21);

console.log("🎉 You have passed all the tests provided.");
