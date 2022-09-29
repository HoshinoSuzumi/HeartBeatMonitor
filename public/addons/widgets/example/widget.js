const elemTime = document.getElementById('time');

const padZero = (num) => {
    return num < 10 ? '0' + num : num;
}

const getTime = () => {
    const date = new Date();
    const h = date.getHours();
    const m = date.getMinutes();
    const s = date.getSeconds();
    return `${padZero(h)}:${padZero(m)}:${padZero(s)}`;
}

setInterval(() => {
    elemTime.innerText = getTime();
}, 1000);
