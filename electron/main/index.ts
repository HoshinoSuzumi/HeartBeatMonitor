// The built directory structure
//
// ├─┬ dist
// │ ├─┬ electron
// │ │ ├─┬ main
// │ │ │ └── index.js
// │ │ └─┬ preload
// │ │   └── index.js
// │ ├── index.html
// │ ├── ...other-static-files-from-public
// │
process.env.DIST = join(__dirname, '../..')
process.env.PUBLIC = app.isPackaged ? process.env.DIST : join(process.env.DIST, '../public')

import { app, BrowserWindow, shell, ipcMain, nativeImage, Tray, Menu } from 'electron'
import { release } from 'os'
import { join } from 'path'

// Disable GPU Acceleration for Windows 7
if (release().startsWith('6.1')) app.disableHardwareAcceleration()

// Set application name for Windows 10+ notifications
if (process.platform === 'win32') app.setAppUserModelId(app.getName())

if (!app.requestSingleInstanceLock()) {
  app.quit()
  process.exit(0)
}

app.commandLine.appendSwitch('enable-web-bluetooth', 'enabled');
app.commandLine.appendSwitch('enable-experimental-web-platform-features', 'enabled');

// Remove electron security warnings
// This warning only shows in development mode
// Read more on https://www.electronjs.org/docs/latest/tutorial/security
process.env['ELECTRON_DISABLE_SECURITY_WARNINGS'] = 'true'

let win: BrowserWindow | null = null
let tray: Tray | null = null;
// Here, you can also use other preload
const preload = join(__dirname, '../preload/index.js')
const url = process.env.VITE_DEV_SERVER_URL as string
const indexHtml = join(process.env.DIST, 'index.html')

async function createWindow() {
  win = new BrowserWindow({
    title: 'Main window',
    icon: join(process.env.PUBLIC, 'favicon.ico'),
    width: 800,
    height: 600,
    minWidth: 800,
    minHeight: 600,
    webPreferences: {
      preload,
      nodeIntegration: true,
      contextIsolation: false,
    },
    frame: true,
    titleBarOverlay: {
      color: '#e4e4e4',
      symbolColor: '#F25E86',
      height: 25
    },
    titleBarStyle: 'hidden'
  })

  if (app.isPackaged) {
    win.loadFile(indexHtml)
  } else {
    win.loadURL(url)
    // Open devTool if the app is not packaged
    win.webContents.openDevTools()
  }

  // Test actively push message to the Electron-Renderer
  win.webContents.on('did-finish-load', () => {
    win?.webContents.send('main-process-message', new Date().toLocaleString())
  })

  // Bluetooth device selection handler
  win.webContents.on('select-bluetooth-device', (ev, deviceList, callback) => {
    ev.preventDefault()
    win.webContents.send('ble-scan-devices', deviceList)
    // TODO: 用户发起连接请求时返回 deviceId
    callback(deviceList[0].deviceId)
    // callback('')
  })

  // Make all links open with the browser, not with the application
  win.webContents.setWindowOpenHandler(({ url }) => {
    if (url.startsWith('https:')) shell.openExternal(url)
    return { action: 'deny' }
  })

  ipcMain.on('perform-connect', (ev, deviceInfo) => {
    win.webContents.send('require-connect-request', deviceInfo);
  })

  win.addListener('close', (e) => {
    e.preventDefault();
    console.log(e);
    win.hide();
  })
}

const createOrFocusWindow = () => {
  if (win) {
    if (win.isMinimized()) win.restore()
    if (!win.isVisible()) win.show()
    win.focus()
  } else {
    createWindow();
  }
}

app.whenReady().then(() => {
  createWindow();
  const trayIcon = nativeImage.createFromPath(join(process.env.PUBLIC, 'favicon.ico'));
  tray = new Tray(trayIcon);

  const contextMenu = Menu.buildFromTemplate([
    { label: '打开面板', type: 'normal', click: createOrFocusWindow },
    { type: 'separator' },
    { label: '退出程序', type: 'normal', click: () => { win.removeAllListeners(); app.quit() } },
  ]);

  tray.setContextMenu(contextMenu);
  tray.setToolTip('This is my application');
  tray.setTitle('This is my title');

  tray.addListener('double-click', e => {
    createOrFocusWindow();
  })
})

// app.on('window-all-closed', () => {
//   // win = null
//   // if (process.platform !== 'darwin') app.quit()
// })

app.on('second-instance', () => {
  if (win) {
    // Focus on the main window if the user tried to open another
    if (win.isMinimized()) win.restore()
    win.focus()
  }
})

app.on('activate', () => {
  const allWindows = BrowserWindow.getAllWindows()
  if (allWindows.length) {
    allWindows[0].focus()
  } else {
    createWindow()
  }
})

// new window example arg: new windows url
ipcMain.handle('open-win', (event, arg) => {
  const childWindow = new BrowserWindow({
    webPreferences: {
      preload,
    },
  })

  if (app.isPackaged) {
    childWindow.loadFile(indexHtml, { hash: arg })
  } else {
    childWindow.loadURL(`${url}/#${arg}`)
    // childWindow.webContents.openDevTools({ mode: "undocked", activate: true })
  }
})
