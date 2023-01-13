import * as wasm from "rust-wasm-app-template";

// Rust defined function using JS greet function
wasm.greet("Hansu")
// Rust exported function
let sum = wasm.add_unumbers(40, 2);
document.getElementById("test_value").innerHTML = "Answer to the Ultimate Question of Life,\nthe Universe,\n and Everything:\n" + sum.toString();

const obj = new wasm.Baz(3);
// get by custom function
console.log("get an private field by a custom function\n field1: ", obj.get_field1())
// set a private field
console.log("set an exposed private field\n field1=30")
obj.field1 = 30;
// get from a private field
console.log("get from an exposed private field\n field1: ", obj.field1)
// set a public field
console.log("set the public field to 100\n field2=100")
obj.field2 = 100
// get a public field
console.log("get the previous public field\n field2: ", obj.field2)
// get an only-read pub field
console.log("Get and only-read public field\n field3: ", obj.field3)
// try to set an only-read pub field
console.log("Try to set an only-read public field:\n")
try { obj.field3 = 65 } catch (e) {
    console.error(e)
};
// trying to read a private field
console.log("trying to read a private field\n field0: ", obj.field0)
console.assert(15 == obj.field0, "Unaccesible")
// trying to set a private field
console.log("Be aware of this behaviour:\n")
console.log("trying to set a private field\n field0=1000")
try { obj.field0 = 1000; } catch (e) {
    console.error(e)
};
console.log("trying to read a private field\n field0: ", obj.field0)
console.log("Now the JS object field0 attribute points to something different to the Rust structure instance field0")