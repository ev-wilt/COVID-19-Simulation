import * as PIXI from 'pixi.js';
const rust = import('./pkg/index');
const Chart = require('chart.js');

const healthyColor = 0xadd8e6;
const sickColor = 0xba6d20;
const recoveredColor = 0xa885cc;
const appWidth = 400;
const appHeight = 300;
const chartUpdateTimeMilli = 500;
const totalPeople = 50;

let simType = 'freeForAll';

function hexToRgb(hex) {
    return '#' + hex.toString(16);
}

function initChart() {
    let ctx = document.getElementById('statusChart').getContext('2d');
    return new Chart(ctx, {
        // The type of chart we want to create
        type: 'line',
    
        // The data for our dataset
        data: {
            labels: [],
            datasets: [
                {
                    label: 'Sick',
                    backgroundColor: hexToRgb(sickColor),
                    borderColor: hexToRgb(sickColor),
                    data: []
                },
                {
                    label: 'Healthy',
                    backgroundColor: hexToRgb(healthyColor),
                    borderColor: hexToRgb(healthyColor),
                    data: []
                },
                {
                    label: 'Recovered',
                    backgroundColor: hexToRgb(recoveredColor),
                    borderColor: hexToRgb(recoveredColor),
                    data: []
                },
            ]
        },
    
        // Configuration options go here
        options: {
            responsive: false,
            scales: {
                yAxes: [{
                    stacked: true,
                    ticks: {
                        max: totalPeople,
                        display: false
                    }
                }],
                xAxes: [{
                    ticks: {
                        display: false
                    }
                }]
            },
            elements: {
                point: {
                    radius: 0
                }
            }  
        }
    });
}

function getPercentage() {
    const percentageInput = document.getElementById('percentage');
    return parseInt(percentageInput.value);
}

rust.then(m => {
    const chart = initChart();
    const app = new PIXI.Application({
        width: appWidth, 
        height: appHeight, 
        backgroundColor: 0xffffff, 
        resolution: window.devicePixelRatio || 1,
        antialias: true
    });
    let simulation = new m.Simulation(appWidth, appHeight, simType, 0);
    let lastUpdateTime = Date.now();
    const viewContainer = document.createElement('div');
    const chartContainer = document.getElementById('chart');
    const graphics = new PIXI.Graphics();
    app.stage.addChild(graphics);

    app.view.setAttribute('style', 'border-style: solid;');
    viewContainer.setAttribute('style', 'text-align: center;');

    viewContainer.appendChild(app.view);
    chartContainer.parentNode.insertBefore(viewContainer, chartContainer);

    const newSimButton = document.getElementById('newSim');
    newSimButton.addEventListener('click', event => {
        simulation = new m.Simulation(appWidth, appHeight, simType, getPercentage());
        chart.data.labels = [];
        chart.data.datasets[0].data = [];
        chart.data.datasets[1].data = [];
        chart.data.datasets[2].data = [];
    });

    const radioButtons = document.getElementsByName('simType');
    radioButtons.forEach((radio) => {
        radio.onclick = function() {
            const percentageInput = document.getElementById('percentage');
            const label = document.getElementById('percentageLabel');

            if (radio.value === 'distancing' && percentageInput.hidden) {
                percentageInput.hidden = false;
                label.hidden = false;
            }
            else if (radio.value !== 'distancing' && !percentageInput.hidden){
                percentageInput.hidden = true;
                label.hidden = true;
            }

            simType = radio.value;
        }
    });

    function render() {
        graphics.clear();
        simulation.get_updated_people().forEach(person => {
            graphics.lineStyle(0);
            if (person.status === 'Healthy') {
                graphics.beginFill(healthyColor, 1);
            }
            else if (person.status === 'Sick') {
                graphics.beginFill(sickColor, 1);
            }
            else if (person.status === 'Recovered') {
                graphics.beginFill(recoveredColor, 1);
            }
            graphics.drawCircle(person.x, person.y, 5);
            graphics.endFill();
        })
    }

    function updateChart(diffTime) {
        chart.data.labels.push(diffTime.toString());
        chart.data.datasets[0].data.push(simulation.get_sick_total());
        chart.data.datasets[1].data.push(simulation.get_healthy_total());
        chart.data.datasets[2].data.push(simulation.get_recovered_total());
        chart.update()
    }

    function update() {
        app.ticker.add((delta) => {
            simulation.update();
            render();
            if (Date.now() - lastUpdateTime > chartUpdateTimeMilli && simulation.get_recovered_total() < totalPeople - 5) {
                updateChart(Date.now() - lastUpdateTime);
                lastUpdateTime = Date.now();
            }

            // Update counters
            document.getElementById('sickCount').innerHTML = simulation.get_sick_total();
            document.getElementById('healthyCount').innerHTML = simulation.get_healthy_total();
            document.getElementById('recoveredCount').innerHTML = simulation.get_recovered_total();
        });        
    }

    updateChart(Date.now() - lastUpdateTime);
    update();
});

