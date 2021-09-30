function println(...args) {
    console.log(...args)
};
const print = println;
function type(value) {
    return {
        "boolean": "bool",
    }[typeof value] || typeof value;
}
