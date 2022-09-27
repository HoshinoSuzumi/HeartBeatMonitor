import { reactive } from 'vue'

const store = reactive({
    scan: {
        isScanning: false,
        scannedDevices: [] as any[],
    },
    connection: {
        isConnected: false,
        isPerformingConnect: false,
        deviceName: '',
        deviceId: ''
    },
    data: {
        hr: 0,
        h: 0,
        l: 0,
        avg: 0,
        history: [] as any[],
    },
    updateIsScanning(isScanning: boolean) {
        this.scan.isScanning = isScanning
    },
    updateIsPerformingConnect(isPerformingConnect: boolean) {
        this.connection.isPerformingConnect = isPerformingConnect;
    },
    updateIsConnected(isConnected: boolean) {
        this.connection.isConnected = isConnected;
    },
    updateScannedDevices(devices: never[]) {
        this.scan.scannedDevices = devices
    },
    updateConnectedDeviceName(deviceName: string) {
        this.connection.deviceName = deviceName;
    },
    updateConnectedDeviceId(deviceId: string) {
        this.connection.deviceId = deviceId;
    },
    updateHR(hr: number) {
        this.data.hr = hr;
        if (this.data.l <= 0) this.data.l = hr;
        this.data.h = Math.max(hr, this.data.h);
        this.data.l = Math.min(hr, this.data.l);
        this.data.avg = parseInt((this.data.history.reduce((a, b) => a + b, 0) / this.data.history.length).toFixed(0));
        this.data.history.push(hr);
        if (this.data.history.length > 500) {
            this.data.history.shift();
        }
    },
    addOrUpdateScannedDevice(device: any) {
        const index = this.scan.scannedDevices.findIndex((d: any) => d.deviceId === device.deviceId)
        if (index === -1) {
            this.scan.scannedDevices.push(device)
        } else {
            this.scan.scannedDevices[index] = device
        }
    },
    clearScannedDevices() {
        this.scan.scannedDevices = []
    }
})

export default store
