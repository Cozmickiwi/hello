let body = document.querySelector("body");
let button = document.getElementById("button1");
button.addEventListener('mousedown', () => {
    let hran = Math.random() * 100;
    let wran = Math.random() * 100;
    hey = document.createElement('h2');
    hey.textContent = "hey!";
    hey.style.top = `${hran}vh`;
    hey.style.left = `${wran}vw`;
    body.appendChild(hey);
})
