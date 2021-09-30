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
function __lagoon_in(left, right) {
    if (typeof left === 'string' && typeof right === 'string') {
        return left.includes(right)
    }
    
    if (Array.isArray(right)) {
        return right.includes(left)
    }
}