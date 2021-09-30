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
function __lagoon_for_in(target, callback) {
    target.forEach(callback)
}
function __lagoon_register_method(target, name, callback, instance = false) {
    if (instance) {
        target.prototype[name] = callback
    } else {
        target[name] = callback
    }
}
Array.prototype.isEmpty = function () {
    return this.length <= 0
}
Array.prototype.each = function (callback) {
    return this.forEach((item) => callback(item))
}
Array.prototype.first = function (callback = undefined) {
    return (callback ? this.find(item => callback(item)) : this[0]) ?? null
}
const __lagoon_og_array_reverse = Array.prototype.reverse
Array.prototype.reverse = function () {
    return __lagoon_og_array_reverse.call([...this])
}
;