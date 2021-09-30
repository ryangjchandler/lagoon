function println(...args) {
    args.forEach(arg => {
        console.log(arg);
    });
};
const print = println;
function type(value) {
    return {
        "boolean": "bool",
    }[typeof value] || typeof value;
}
