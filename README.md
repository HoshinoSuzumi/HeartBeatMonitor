# HeartBeat Monitor

![Cover](https://socialify.git.ci/HoshinoSuzumi/HeartBeatMonitor/image?description=1&descriptionEditable=%E8%93%9D%E7%89%99%E5%BF%83%E7%8E%87%E5%B9%BF%E6%92%AD%E8%AE%BE%E5%A4%87%E9%87%87%E9%9B%86%E5%99%A8%E3%80%81%E6%A1%8C%E9%9D%A2%E6%82%AC%E6%B5%AE%E7%AA%97%E5%92%8C%E6%8E%A8%E6%B5%81%E6%8F%92%E4%BB%B6&font=KoHo&issues=1&logo=https%3A%2F%2Fraw.githubusercontent.com%2FHoshinoSuzumi%2FHoshinoSuzumi%2Fmaster%2Fimages%2F202209282211354.png&owner=1&pattern=Circuit%20Board&pulls=1&stargazers=1&theme=Light)

[![Jenkins status](http://ci.uniiem.com/job/HeartBeatMonitor/badge/icon)](http://ci.uniiem.com/job/HeartBeatMonitor/lastBuild/)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/HoshinoSuzumi/HeartBeatMonitor)](https://github.com/HoshinoSuzumi/HeartBeatMonitor/releases/latest)
[![wakatime](https://wakatime.com/badge/user/589c46ee-6ba6-403c-bc9f-3a7aef5b206c/project/09dbf99c-f931-465c-829d-d1648bf7c4ef.svg)](https://wakatime.com/badge/user/589c46ee-6ba6-403c-bc9f-3a7aef5b206c/project/09dbf99c-f931-465c-829d-d1648bf7c4ef)

**🚧 [WIP]** 蓝牙心率广播设备采集器、桌面悬浮窗和推流插件

## 简介

这个工具可以从低功耗蓝牙 (BLE) 心率广播设备采集数据，并显示到桌面悬浮窗或者推流插件。

## 截图

![设备扫描](https://raw.githubusercontents.com/HoshinoSuzumi/HoshinoSuzumi/master/images/202209282245007.png)

![心率曲线](https://raw.githubusercontents.com/HoshinoSuzumi/HoshinoSuzumi/master/images/202209282246455.png)

## 设备支持

原理上这个程序支持所有不加密<sup>(1)</sup>的低功耗心率广播 (BLE heart_rate) 设备，例如心率环，消费市场的华为手环 7、华为 WATCH GT3 Pro 系列等。

因为我只有华为手环 7，所以目前只对这一种设备进行了测试。**如果您手里恰好有标称或带有“心率广播”功能的设备，希望您能够抽出一点时间帮助我完成测试，如果可以正常工作，请开 issue 告知我设备型号，谢谢！**

### 已经测试的设备列表

设备|可用性|备注
-----|:---:|:---:
华为手环 7|✅|-
华为 WATCH GT3|❌|非 Pro 款不含心率广播功能
小米手环 5|❌|有加密

### 角注

<sup>(1)</sup> 本项目使用 Electron 开发，蓝牙设备接口使用的 [Web Bluetooth API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Bluetooth_API)，这个接口有局限性且不完善，所以很难支持带有加密的设备。Electron 可以调用 C++ 的方法，有能力实现的大佬欢迎 PR。也考虑后期开发配套 Android 应用程序透传数据。

## 贡献

欢迎任何形式的 Issue, Pull Request。

任何一行有用的代码、一款设备的测试都是巨大的贡献。
