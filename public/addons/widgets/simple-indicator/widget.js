const { ipcRenderer } = require('electron');
// const { getCurrentWindow } = require('@electron/remote');

// const win = getCurrentWindow();
// win.setIgnoreMouseEvents(true, { forward: true });

const elemHrValue = document.getElementById('hr');

ipcRenderer.on('heart-rate-broadcast', (ev, hr) => {
    elemHrValue.innerText = hr;
});
