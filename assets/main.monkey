// comment

let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

if (result > 10) {
    result = result - 1;
} else {
    result = result + 1;
}

return result;
