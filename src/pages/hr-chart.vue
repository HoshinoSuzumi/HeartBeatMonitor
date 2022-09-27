<script setup>
import * as echarts from 'echarts';
import { onActivated, onDeactivated, ref, watch } from 'vue';
import { computed } from '@vue/reactivity';
import store from '../store/index';

const isConnected = computed(() => store.connection.isConnected);
const hrData = computed(() => store.data);

const isActivated = ref(false);

const refChart = ref();
let hrChart = null;
let xAxisData = [];

onActivated(() => {
    isActivated.value = true;
    const loadEChart = new Promise((resolve) => {
        resolve();
    });
    if (isConnected.value) {
        loadEChart.then(() => {
            hrChart = echarts.init(refChart.value);
            hrChart.setOption({
                tooltip: {},
                textStyle: {
                    fontFamily: 'MiSans',
                },
                xAxis: {
                    type: 'category',
                    boundaryGap: false,
                    data: []
                },
                yAxis: {
                    boundaryGap: [0.5, 0.5],
                    type: 'value',
                    scale: true,
                },
                legend: {
                    bottom: 0,
                    data: [
                        {
                            name: '心率',
                        },
                        {
                            name: '平均值',
                        }
                    ],
                    lineStyle: {
                        cap: 'round',
                        shadowColor: 'rgba(242, 94, 134, 0.8)',
                        shadowBlur: 4,
                        shadowOffsetY: 2,
                    },
                    itemStyle: {
                        opacity: 0,
                    },
                    itemGap: 14,
                },
                series: [
                    {
                        // data: [0],
                        name: '心率',
                        type: 'line',
                        symbol: 'none',
                        smooth: true,
                        areaStyle: {
                            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [{
                                offset: 0,
                                color: 'rgba(242, 94, 134, 0.5)'
                            }, {
                                offset: 1,
                                color: 'rgba(242, 94, 134, 0.1)'
                            }])
                        },
                        lineStyle: {
                            cap: 'round',
                            color: 'rgb(242, 94, 134)',
                            shadowColor: 'rgba(242, 94, 134, 0.8)',
                            shadowBlur: 4,
                            shadowOffsetY: 2,
                        },
                    },
                    {
                        // data: [0],
                        name: '平均值',
                        type: 'line',
                        symbol: 'none',
                        smooth: true,
                        lineStyle: {
                            width: 1,
                            type: 'dashed',
                            color: 'rgba(242, 94, 134, .8)'
                        },
                    },
                ]
            });
            window.onresize = function () {
                hrChart.resize();
            };
        });
    }
});

onDeactivated(() => {
    isActivated.value = false;
    if (hrChart) {
        hrChart.dispose();
    }
});

watch(hrData, v => {
    const time = new Date().toTimeString().split(' ')[0];
    xAxisData.push(time);
    let avgData = new Array(xAxisData.length).fill(v.avg);
    if (hrChart && isActivated.value) {
        hrChart.setOption({
            xAxis: {
                data: xAxisData.slice(-50)
            },
            series: [
                {
                    data: v.history.slice(-50)
                },
                {
                    data: avgData.slice(-50)
                }
            ]
        });
    }
}, { deep: true });
</script>

<template>
    <div class="container">
        <div class="header">
            <h2 class="title">心率曲线</h2>
            <div class="actions">
                <span class="additional">▲ <span>{{ isConnected ? hrData.h || '--' : '--' }}</span></span>
                <span class="additional">▼ <span>{{ isConnected ? hrData.l || '--' : '--' }}</span></span>
                <span class="additional">AVG <span>{{ isConnected ? hrData.avg || '--' : '--' }}</span></span>
                <span class="additional">|</span>
                <span class="additional">实时 <span class="accent" style="margin-left: 10px;">{{ isConnected ? hrData.hr
                || '--' : '--' }}</span></span>
            </div>
        </div>
        <div class="content">
            <div v-if="!isConnected" class="empty">
                <span>连接心率采集设备后才能显示数据</span>
                <router-link class="btn outline" to="/">转到 “设备连接”</router-link>
            </div>
            <div class="viewer" v-else>
                <div class="chart-wrapper">
                    <div class="chart" ref="refChart"></div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.additional {
    margin: 0 8px;
}

.viewer {
    width: 100%;
    height: 100%;
}

.chart-wrapper {
    width: 100%;
    height: 100%;
}

.chart-wrapper .chart {
    width: 100%;
    height: 100%;
}
</style>
