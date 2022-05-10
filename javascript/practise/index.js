// input handlers
const source = document.getElementById('source');
const result = document.getElementById('result');

const inputHandler = function (e) {
    result.innerHTML = e.target.value;
}

source.addEventListener('input', inputHandler);
source.addEventListener('propertychange', inputHandler); // for IE8

